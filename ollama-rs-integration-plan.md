# Plan d'intégration de `ollama-rs`

Statut : proposition — non implémenté
Branche : `claude/ollama-rs-integration-8an4bc`
Dépendance visée : [`ollama-rs`](https://github.com/pepperoni21/ollama-rs) (crates.io)

## 1. Objectif

Ajouter un provider **Ollama natif**, basé sur la crate `ollama-rs`, en plus des
providers existants — **sans rien retirer ni casser** pour :

- LM Studio (via `providers.openai.base_url`, endpoint OpenAI-compatible)
- OpenAI / GPT (`providers.openai`)
- Anthropic / Claude (`providers.anthropic`)
- Qwen / DashScope / vLLM (`providers.qwen`)
- Azure OpenAI (`providers.azure`)

Le nouveau provider est **additif** : Ollama reste utilisable comme aujourd'hui
via le mode OpenAI-compatible (`OPENAI_BASE_URL=http://localhost:11434/v1`),
et devient **en plus** utilisable via `providers.ollama`, qui parle à l'API
native d'Ollama (`/api/chat`, `/api/generate`, `/api/tags`, `/api/pull`, …).

En complément du provider lui-même, ce plan couvre la **remontée dans la TUI**
d'informations utiles à l'utilisateur — modèle réellement utilisé, provider
actif, débit de génération (tokens/s), temps de chargement du modèle,
progression d'un `pull` — sans dégrader l'affichage existant pour les
providers qui n'exposent pas ces données (Anthropic, OpenAI, Qwen, Azure).

## 2. Pourquoi un provider natif, alors qu'Ollama fonctionne déjà ?

Le shim OpenAI-compatible (`src/llm/provider/openai.rs`) couvre déjà le chat,
le streaming, les tool calls et la détection heuristique de vision. Ce qu'il
**ne** couvre pas, et que l'API native d'Ollama (et donc `ollama-rs`) permet :

| Fonctionnalité                                   | Shim OpenAI-compat | API native Ollama |
|---------------------------------------------------|:---:|:---:|
| Chat / streaming / tool calls                     | ✅ | ✅ |
| Gestion des modèles (`list`, `pull`, `show`, `copy`, `delete`) | ❌ | ✅ |
| `pull` avec progression (téléchargement de modèle) | ❌ | ✅ |
| `keep_alive` (contrôle du déchargement mémoire)   | ❌ | ✅ |
| `num_ctx` / options Modelfile natives              | ❌ (approximé) | ✅ |
| Embeddings (`/api/embeddings`)                     | ❌ | ✅ |
| Images en entrée (multimodal) sans base64 manuel   | partiel | ✅ (typed) |
| Structured output (`format: <json schema>`)        | partiel (`response_format`) | ✅ |
| **Métriques de performance** (durées de chargement, prefill, génération) | ❌ (absentes de la réponse OpenAI-compat) | ✅ (`total_duration`, `load_duration`, `prompt_eval_duration`, `eval_duration`, `eval_count`) |

L'intérêt de `ollama-rs` est donc double : débloquer la **gestion de
modèles** (pull depuis Crustly, liste des modèles installés, suppression) et
les **embeddings**, et exposer des **métriques de performance natives** que
le shim OpenAI-compatible ne renvoie jamais (Ollama ne les inclut pas dans sa
réponse au format `/v1/chat/completions`, seulement dans `/api/chat`).

## 3. Contraintes de compatibilité (non négociables)

1. Aucune modification de la trait `Provider` (`src/llm/provider/trait.rs`) —
   le nouveau provider doit s'y conformer tel quel.
2. Aucune régression sur `OpenAIProvider::local()` — LM Studio, Ollama via
   compat OpenAI, LocalAI continuent de fonctionner exactement comme avant.
3. `create_provider()` (factory) garde son comportement actuel par défaut :
   l'ajout d'Ollama ne doit pas changer quel provider est choisi quand
   seule une config `openai`/`anthropic`/`qwen` existe déjà.
4. `ollama-rs` est ajouté comme dépendance **optionnelle**, activée par une
   feature Cargo (`ollama`), cohérente avec le pattern existant
   (`openai`, `aws-bedrock`, `all-llm`).
5. Tous les champs ajoutés à `LLMResponse`/`AgentResponse`/`DisplayMessage`/DB
   sont des `Option<T>` avec valeur par défaut `None` : un provider qui ne les
   renseigne pas (Anthropic, OpenAI, Qwen, Azure) n'a **aucun changement de
   comportement ni d'affichage**.
6. Tous les tests existants (`cargo test`) doivent continuer à passer.

## 4. Architecture du provider

### 4.1 Dépendances (`Cargo.toml`)

```toml
[dependencies]
ollama-rs = { version = "0.2", optional = true, features = ["stream"] }

[features]
ollama = ["ollama-rs"]
all-llm = ["openai", "aws-bedrock", "ollama"]
```

À vérifier à l'implémentation : version exacte publiée sur crates.io et
compatibilité avec `tokio 1.35` / `reqwest 0.11` déjà présents dans l'arbre de
dépendances (éviter un doublon de version de reqwest).

### 4.2 Nouveau fichier `src/llm/provider/ollama.rs`

Implémente `Provider` pour `OllamaProvider`, sur le modèle de
`src/llm/provider/openai.rs` :

- `OllamaProvider::new(host: String, port: u16)` / `OllamaProvider::default_local()`
  (`http://localhost:11434`)
- `with_default_model(model: String)`
- `with_keep_alive(duration: KeepAlive)` (option native, absente du shim actuel)
- Mapping `LLMRequest -> ollama_rs::generation::chat::ChatMessageRequest` :
  - `messages`, `system` → message `system` en tête (comme `openai.rs`)
  - `tools` → format d'outils natif d'Ollama (compatible OpenAI function
    calling depuis Ollama ≥ 0.3)
  - `temperature`, `top_p`, `seed`, `stop`, `frequency_penalty`,
    `presence_penalty` → `ModelOptions` d'`ollama-rs`
  - `response_format` → paramètre `format` natif (JSON mode / JSON Schema)
  - `thinking` (Anthropic `ThinkingConfig`) → pas d'équivalent natif direct ;
    conserver la stratégie actuelle de `extract_think_tags()` sur le texte de
    réponse pour les modèles type DeepSeek-R1/QwQ (réutiliser la fonction
    existante dans `types.rs`, ne pas la dupliquer)
- Mapping réponse `ChatMessageResponse -> LLMResponse`, y compris
  `ContentBlock::ToolUse` pour les tool calls, `TokenUsage` depuis
  `prompt_eval_count` / `eval_count`, **et `PerfMetrics` depuis les champs de
  durée natifs** (voir §5.2)
- `stream()` : consommer le stream natif d'`ollama-rs` et le traduire vers
  `StreamEvent` (`MessageStart` → `ContentBlockDelta` → `MessageStop`), en
  suivant le même séquencement que celui déjà présent dans `openai.rs`
  (accumulation des fragments de tool calls avant de les émettre). Le dernier
  chunk du stream Ollama porte les mêmes champs de durée/compteurs que la
  réponse non-streamée — à capturer dans le `MessageDelta`/`usage` final pour
  alimenter `PerfMetrics` aussi en mode streaming.
- `supports_vision()` : réutiliser la même liste de motifs de noms de modèles
  que `openai.rs` (`llava`, `vision`, `minicpm-v`, etc.), extraite dans une
  fonction partagée `super::model_hints::is_vision_model()` pour éviter la
  duplication entre les deux providers
- Erreurs `ollama-rs::error::OllamaError` → `ProviderError` existant
  (`ApiError`, `HttpError`, `Timeout`, …)

### 4.3 Extension du modèle de configuration (`src/config/mod.rs`)

```rust
pub struct ProviderConfigs {
    // ... existants inchangés ...
    #[serde(default)]
    pub ollama: Option<OllamaProviderConfig>,
}

pub struct OllamaProviderConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default = "default_ollama_host")]
    pub host: String,          // "http://localhost:11434"
    pub default_model: Option<String>,
    pub keep_alive: Option<String>,   // "5m", "-1" (toujours chargé), "0"
    #[serde(default)]
    pub num_ctx: Option<u32>,
}
```

Variables d'environnement (suivant le pattern `OPENAI_BASE_URL` /
`QWEN_BASE_URL` déjà en place dans `Config::from_env`/équivalent) :

- `OLLAMA_HOST` (ou réutiliser `OLLAMA_BASE_URL` si on veut rester cohérent
  avec la convention CLI officielle d'Ollama, qui utilise `OLLAMA_HOST`)
- `OLLAMA_MODEL` pour le modèle par défaut

### 4.4 Factory (`src/llm/provider/factory.rs`)

Ajouter `try_create_ollama()`, inséré dans `create_provider()` **après** Qwen
et **avant** OpenAI, pour ne pas changer la résolution par défaut existante
quand seul `providers.openai` (LM Studio) est configuré :

```
1. Qwen (si configuré)
2. Ollama natif (si providers.ollama configuré)   <-- nouveau
3. OpenAI / compat local (LM Studio, Ollama via /v1, LocalAI)
4. Anthropic (fallback par défaut)
```

Comme pour Qwen/OpenAI, le choix ne s'active que si `providers.ollama` est
explicitement renseigné dans la config ou via variable d'env — sinon
comportement 100 % identique à aujourd'hui.

### 4.5 Gestion de modèles (nouvelle capacité, hors trait `Provider`)

Ajouter un module utilitaire optionnel `src/llm/provider/ollama_models.rs`
(derrière la feature `ollama`) exposant :

- `list_models()`, `pull_model(name, on_progress)`, `show_model(name)`,
  `delete_model(name)`

`pull_model` accepte un callback/canal de progression (voir §5.4) — c'est ce
qui permet d'afficher une vraie barre de progression dans la TUI plutôt qu'un
simple spinner.

Exposés via une sous-commande CLI `crustly ollama <list|pull|rm|show>` dans
`src/cli/mod.rs` (nouveau, additif — n'affecte pas les commandes existantes).

### 4.6 Documentation

- `config.toml.example` : ajouter un bloc `[providers.ollama]` commenté, à
  côté du bloc existant `[providers.openai]` (qui reste la méthode
  recommandée pour LM Studio).
- `README.md` : nouvelle sous-section « Ollama natif (via `ollama-rs`) » dans
  la section Local LLMs, en clarifiant explicitement les **deux** chemins
  disponibles :
  - `providers.openai.base_url = "http://localhost:11434/v1"` (existant,
    compatible tool calling actuel)
  - `providers.ollama.host = "http://localhost:11434"` (nouveau, gestion de
    modèles + embeddings + métriques de performance dans la TUI)

## 5. Remontée d'informations dans la TUI

### 5.1 Ce que l'API native d'Ollama expose (et que `ollama-rs` typera)

Chaque réponse `/api/chat` (non-streamée et dernier chunk en streaming)
contient, en nanosecondes :

| Champ Ollama          | Signification                                   |
|------------------------|--------------------------------------------------|
| `total_duration`       | Durée totale de la requête                        |
| `load_duration`        | Temps de chargement du modèle en mémoire (0 si déjà chargé — "warm") |
| `prompt_eval_count` / `prompt_eval_duration` | Tokens et durée du prefill (lecture du prompt) |
| `eval_count` / `eval_duration`               | Tokens et durée de la génération (sortie) |

À partir de ces champs on calcule directement une métrique très parlante pour
l'utilisateur : **le débit de génération en tokens/seconde**
(`eval_count / (eval_duration / 1e9)`), ainsi que le **temps de chargement du
modèle** (utile pour diagnostiquer un premier appel lent après un
"cold start", ou un modèle qui se décharge trop vite faute de `keep_alive`).

### 5.2 Extension des types de données (additive, `Option<T>` partout)

**`src/llm/provider/types.rs`** — nouveau type, réutilisable par n'importe
quel provider futur (pas seulement Ollama) :

```rust
/// Runtime performance metrics reported by local inference backends.
/// `None` for providers that don't expose this level of detail
/// (Anthropic, OpenAI, Qwen, Azure) — purely additive, no behavior change.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PerfMetrics {
    /// Time to load/warm the model into memory (ms). `Some(0)` when the
    /// model was already resident ("warm" — see `model_was_loaded`).
    pub load_duration_ms: Option<u64>,
    /// Prefill duration — time spent evaluating the input prompt (ms).
    pub prompt_eval_duration_ms: Option<u64>,
    /// Generation duration — time spent producing the output (ms).
    pub eval_duration_ms: Option<u64>,
    /// Total wall-clock duration for the request (ms).
    pub total_duration_ms: Option<u64>,
    /// `true` if the model was already loaded (warm start), `false` if it
    /// had to be loaded first (cold start), `None` if unknown/unsupported.
    pub model_was_loaded: Option<bool>,
}

impl PerfMetrics {
    /// Generation throughput in tokens/second.
    pub fn tokens_per_second(&self, output_tokens: u32) -> Option<f64> {
        let ms = self.eval_duration_ms?;
        (ms > 0).then(|| output_tokens as f64 / (ms as f64 / 1000.0))
    }
}
```

Ajout à `LLMResponse` (suit exactement le pattern déjà utilisé pour
`cache_metrics: Option<CacheMetrics>`, spécifique Anthropic) :

```rust
#[serde(skip_serializing_if = "Option::is_none")]
pub perf_metrics: Option<PerfMetrics>,
```

**`src/llm/agent/service.rs`** — `AgentResponse` gagne deux champs :

```rust
pub struct AgentResponse {
    // ... champs existants inchangés ...
    /// Name of the provider that served this response (e.g. "ollama",
    /// "openai", "anthropic"). Lets the TUI show which backend answered,
    /// useful once several providers are configured side by side.
    pub provider_name: String,
    /// Performance metrics, if the provider exposes them.
    pub perf_metrics: Option<crate::llm::provider::PerfMetrics>,
}
```

`provider_name` vient de `Provider::name()` (méthode déjà présente sur la
trait) — il suffit de la propager depuis le point d'appel dans
`service.rs` où `AgentResponse` est construit, au lieu de se limiter au nom du
modèle comme aujourd'hui.

**`src/tui/app.rs`** — `DisplayMessage` gagne les mêmes deux champs
(`provider_name: Option<String>`, `perf_metrics: Option<PerfMetrics>`),
peuplés dans `complete_response()` à côté de `token_count`/`cost` déjà
présents (lignes ~715-726).

`Session` (`src/db/models.rs`) gagne `pub provider: Option<String>`, peuplé
en lazy comme `model` l'est déjà :

```rust
// app.rs complete_response(), à côté du bloc existant :
if session.model.is_none() { session.model = Some(response.model.clone()); }
// nouveau :
if session.provider.is_none() { session.provider = Some(response.provider_name.clone()); }
```

**Migration DB** (`migrations/2026XXXX_add_provider_perf_metrics.sql`) :

```sql
ALTER TABLE sessions ADD COLUMN provider TEXT;
ALTER TABLE messages ADD COLUMN provider_name TEXT;
ALTER TABLE messages ADD COLUMN perf_metrics_json TEXT; -- PerfMetrics sérialisé en JSON
```

Colonnes nullables, aucune donnée existante affectée, aucun changement pour
les sessions/messages déjà en base (elles auront simplement `NULL`).

### 5.3 Pipeline de propagation

```
OllamaProvider::complete()/stream()
    → LLMResponse { usage, perf_metrics: Some(..), .. }
        → llm/agent/service.rs (construit AgentResponse)
            → AgentResponse { provider_name, perf_metrics, .. }
                → tui/app.rs::complete_response()
                    → DisplayMessage { provider_name, perf_metrics, .. }
                    → Session.provider (lazy, une fois par session)
                        → tui/render.rs (affichage, §5.4)
                        → session_service / message repo (persistance DB, §5.2)
```

Aucun de ces types n'est modifié de façon incompatible : ce sont uniquement
des champs ajoutés, initialisés à `None`/valeur par défaut pour tous les
chemins existants (Anthropic, OpenAI, Qwen, Azure).

### 5.4 Nouveaux éléments d'interface (TUI)

Concrètement, dans `src/tui/render.rs` :

1. **Ligne d'en-tête (`render_header`, actuellement lignes 72-130)** : ajout
   d'un badge provider et, si `perf_metrics` est disponible sur le dernier
   message assistant, du débit courant :

   ```
   📝 Session: my-session │ 🦙 ollama · qwen2.5-coder:7b │ 💬 Tokens: 1234 │ 💰 Cost: $0.0000 │ ⚡ 42 tok/s
   ```

   L'icône provider est choisie par une petite fonction `provider_icon(name)`
   (`🦙 ollama`, `🏠 lm-studio/openai-local`, `🤖 anthropic`, `🌀 qwen`,
   `☁️ azure`) — purement cosmétique, aucun impact fonctionnel.
   Quand `perf_metrics` est `None` (Anthropic, OpenAI, Qwen, Azure), le
   segment `⚡ tok/s` est simplement omis — pas de `0 tok/s` trompeur.

2. **Pied de message assistant** (dans la boucle `render_chat`, après le
   bloc `[Thinking]` existant, ~ligne 250) : une ligne discrète optionnelle
   sous chaque réponse d'un modèle Ollama :

   ```
   🤖 qwen2.5-coder:7b (14:32:10)
   ... contenu de la réponse ...
   ⏱ 812 ms génération · 46 tok/s · 🧊 cold start (modèle chargé en 1.2 s)
   ```

   Rendue uniquement si `msg.perf_metrics.is_some()`.

3. **Panneau "Model Info"** (nouveau, sur le modèle des dialogs existants
   dans `src/tui/components/dialogs/mod.rs`, qui utilisent déjà
   `Clear` + `Block` en overlay pour le panel de progression de plan) :
   overlay déclenché par un raccourci (`Ctrl+M` par exemple, à confirmer côté
   UX), affichant les infos de `ollama_models::show_model()` : famille,
   taille de paramètres, quantization, longueur de contexte du Modelfile,
   `keep_alive` effectif, VRAM approximative. Nécessite un nouveau variant
   `AppMode::ModelInfo` dans `src/tui/events.rs` (même pattern que
   `AppMode::ToolApproval`/`FilePicker`).

4. **Barre de progression `pull`** : overlay analogue au
   `render_auto_exec_progress` déjà présent pour l'exécution de plans
   (`components/dialogs/mod.rs`). `pull_model()` (§4.5) pousse des
   évènements de progression (`{ digest, total, completed }`) sur un
   `mpsc::UnboundedSender`, consommé par la boucle d'évènements TUI
   (`src/tui/events.rs`, même mécanisme que `ToolApprovalRequested`) pour
   mettre à jour une barre `completed/total` en temps réel pendant le
   téléchargement d'un modèle lancé depuis `crustly ollama pull`.

5. **Barre de statut** (`render_status_bar`, ligne ~1381) : en cas d'erreur
   spécifique Ollama (modèle non trouvé, Ollama non démarré), afficher un
   message actionnable au lieu du message générique, par ex. :
   `Ollama unreachable at http://localhost:11434 — run 'ollama serve'`.

### 5.5 Compatibilité et dégradation gracieuse

- Sessions utilisant Anthropic/OpenAI/Qwen/Azure : `provider_name` est
  quand même renseigné (ex. `"anthropic"`, déjà retourné par
  `Provider::name()`), donc le badge provider dans l'en-tête profite à
  **tous** les providers, pas seulement Ollama — amélioration transverse
  cohérente avec l'objectif multi-provider du projet.
- `perf_metrics` reste `None` pour ces providers ⇒ aucune ligne de
  métriques de performance n'apparaît pour eux ⇒ zéro régression visuelle.
- Le panneau "Model Info" et la barre de progression `pull` ne sont
  accessibles/pertinents que lorsque le provider actif est Ollama —
  raccourcis/dialogs no-op (ou masqués) sinon.

### 5.6 Configuration utilisateur

Nouveau champ optionnel, non-bloquant, sous la section TUI existante de
`config.toml` (à localiser précisément dans `src/config/mod.rs` lors de
l'implémentation) :

```toml
[tui]
show_performance_metrics = true   # défaut : true : masque le segment tok/s
                                   # si mis à false, même quand disponible
```

## 6. Ce qui NE change PAS

- `OpenAIProvider`, `AnthropicProvider`, `QwenProvider`, `AzureOpenAIProvider` :
  aucune modification de comportement ni de rendu.
- Le trait `Provider` : signature inchangée.
- `crabrace` (registry de providers) : pas de dépendance croisée avec ce plan
  sauf si l'on souhaite y déclarer Ollama comme provider connu (à évaluer
  séparément, hors scope v1).
- Comportement par défaut de `create_provider()` sans config `ollama` :
  identique bit-à-bit à l'existant.
- Rendu TUI existant pour les sessions/messages déjà en base (colonnes
  `NULL` sur les nouveaux champs) : identique à l'existant.

## 7. Plan de test

1. Tests unitaires (offline, sans dépendre d'un serveur Ollama réel) :
   - construction du provider, mapping requête/réponse, mapping d'erreurs —
     même style que les tests déjà présents dans `openai.rs`/`qwen.rs`.
   - `PerfMetrics::tokens_per_second()` : cas nominal, `eval_duration_ms =
     0`/`None` (pas de division par zéro, retourne `None`).
   - Mapping des durées ns → ms depuis des fixtures `ChatMessageResponse`.
2. Tests de rendu TUI (snapshot avec `insta`, déjà utilisé dans le projet en
   dev-dependency) :
   - en-tête avec `perf_metrics: None` (Anthropic/OpenAI/Qwen/Azure) → rendu
     identique aux snapshots existants (non-régression).
   - en-tête avec `perf_metrics: Some(..)` → nouveau snapshot avec le
     segment tok/s.
3. Tests de non-régression : `cargo test --no-default-features` et
   `cargo test --features all-llm` doivent tous les deux passer.
4. Test de migration DB : appliquer la migration sur une base existante
   (fixture avec des lignes `sessions`/`messages` préexistantes) et vérifier
   que les colonnes ajoutées sont bien `NULL` sans erreur.
5. Test manuel local (nécessite un Ollama réel, non exécutable en CI) :
   - `ollama pull llama3.2` puis validation chat/stream/tool-call/pull via
     `providers.ollama`, et vérification visuelle du badge provider, du
     débit tok/s et du panneau Model Info dans la TUI.
6. Vérifier que la configuration existante (`providers.openai.base_url =
   "http://localhost:11434/v1"`) continue de fonctionner à l'identique en
   parallèle (non-régression LM Studio/Ollama-compat), y compris pour
   l'affichage TUI (pas de segment tok/s puisque le shim OpenAI-compat ne
   renvoie pas ces données).

## 8. Phasage proposé

- **Phase 1 (MVP provider)** : dépendance optionnelle + `OllamaProvider`
  (chat, streaming, tool calls, vision) + config + factory + doc. Livrable
  testable isolément, sans toucher aux autres providers ni à la TUI.
- **Phase 2 (observabilité TUI)** : `PerfMetrics`, propagation
  `LLMResponse → AgentResponse → DisplayMessage → Session`, migration DB,
  badge provider + segment tok/s dans l'en-tête, pied de message avec durée
  de génération. C'est la partie qui bénéficie **aussi** aux providers déjà
  en place (badge provider transverse).
- **Phase 3 (gestion de modèles)** : `list`/`pull`/`rm`/`show` + sous-commande
  CLI + panneau "Model Info" + barre de progression `pull` dans la TUI.
- **Phase 4** : embeddings natifs Ollama, exposés à la couche RAG/recherche
  interne si Crustly en a une (à confirmer selon le code existant).

## 9. Points ouverts à trancher avant implémentation

1. Nom de la variable d'environnement : `OLLAMA_HOST` (convention officielle
   Ollama) vs `OLLAMA_BASE_URL` (cohérence avec `OPENAI_BASE_URL`/`QWEN_BASE_URL`
   internes) — recommandation : accepter les deux, `OLLAMA_HOST` prioritaire.
2. Faut-il activer `ollama` dans la feature `default` ou la garder strictement
   optionnelle (recommandé, pour ne pas alourdir le binaire par défaut) ?
3. Faut-il, à terme, migrer le tool-parsing Ollama actuel (via le shim OpenAI)
   vers le provider natif, ou garder les deux indéfiniment ? → recommandation :
   garder les deux, laisser l'utilisateur choisir via sa config.
4. Raccourci clavier exact pour le panneau "Model Info" (`Ctrl+M` proposé,
   à confirmer — vérifier qu'il n'entre pas en conflit avec un raccourci
   existant dans `src/tui/events.rs`).
5. Stocker `PerfMetrics` en JSON (`perf_metrics_json TEXT`, plus simple, une
   seule migration) ou en colonnes séparées (plus interrogeable en SQL, mais
   plus de colonnes à faire évoluer) — recommandation : JSON, cohérent avec
   le fait que ces métriques sont avant tout informatives/TUI et non
   utilisées dans des requêtes analytiques pour l'instant.
