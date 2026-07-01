# Plan d'intégration de `ollama-rs`

Statut : **Phases 1, 2 et 3 implémentées et testées** (le panneau "Model
Info" dans la TUI reste à faire, cf. tableau). **Phase 4 partiellement
implémentée** (voir détail ci-dessous).
Branche : `claude/ollama-rs-integration-8an4bc`
Dépendance : [`ollama-rs`](https://github.com/pepperoni21/ollama-rs) 0.3.5 (crates.io)

## 0. État d'implémentation (résumé honnête)

| Phase | Statut | Détail |
|---|---|---|
| 1 — Provider natif | ✅ Fait | `src/llm/provider/ollama.rs` (`OllamaProvider`), config, factory, tests unitaires. Testé avec `cargo test --features ollama` (418 tests, 0 échec) et sans la feature (403 tests, 0 échec). `cargo clippy --features ollama` et sans la feature : 0 warning. |
| 2 — Métriques TUI | ✅ Fait | `PerfMetrics` sur `LLMResponse`, propagé via `AgentResponse` → `DisplayMessage`/`Session` → `render.rs` (badge provider + tok/s en en-tête, ligne de métriques sous chaque réponse). Persisté en base (migration `20260701000001_provider_perf_metrics.sql`). |
| 3 — Gestion de modèles | ✅ Fait (sauf panneau Model Info TUI) | `src/llm/provider/ollama_models.rs` (list/pull/delete/show) + sous-commande CLI `crustly ollama list\|pull\|rm\|show` avec barre de progression terminal. **Dialog interactif dans la TUI** (`Ctrl+D` en mode Chat, `src/tui/ollama_download.rs` + `AppMode::ModelDownload`) : saisie du nom de modèle avec suggestions filtrées (modèles déjà installés + liste curatée), navigation ↑↓, `Tab` pour reprendre une suggestion, `Enter` pour lancer le pull, barre de progression live rendue dans l'interface, `Esc` annule le téléchargement en cours (`JoinHandle::abort()`). Le panneau "Model Info" dans la TUI (§5.4 point 3) n'est pas fait — `crustly ollama show` en CLI reste le seul accès. |
| 4 — Embeddings | 🟡 Partiel | `ollama_models::generate_embeddings()` + `crustly ollama embed <model> <text>`. **Pas de couche RAG/retrieval à brancher dessus : Crustly n'en a pas** (vérifié — aucune référence à "embedding" dans le code avant cette phase). La capacité brute est exposée pour un usage futur. |

Écarts connus par rapport au plan technique ci-dessous (à noter avant toute
implémentation ultérieure) :
- **Pas de retry automatique** sur `OllamaProvider::complete()`/`stream()` :
  `ollama-rs` est bâti sur `reqwest` 0.12, incompatible avec le
  `ProviderError::HttpError(reqwest 0.11)` du reste du crate, donc les
  erreurs réseau sont mappées en `ApiError{status:0,..}`, jamais retryable
  par `retry_with_backoff`. Voir le commentaire de module dans `ollama.rs`.
- Les métriques de performance (`PerfMetrics`) ne sont propagées que sur le
  chemin **non-streamé** (`complete()`). En streaming, les durées finales
  sont calculées mais pas encore attachées à un `StreamEvent` (pas de champ
  prévu dans ce type) — capturées en interne puis ignorées (`let _ =
  final_perf;` dans `ollama.rs`).
- `tokens_per_second` sur un message affiché redevient `None` après un
  rechargement de session depuis la base (la colonne `token_count` stocke
  input+output combinés, pas le nombre de tokens de sortie seul nécessaire
  au calcul).
- Dialog "Model Download" : la barre de progression suit la **couche
  courante** téléchargée par Ollama (`completed`/`total` de la réponse
  `/api/pull`), pas une estimation globale multi-couches agrégée — décrit
  comme option possible en §5.7.3 mais non implémenté (Ollama ne renvoie pas
  la taille totale de toutes les couches à l'avance). Pas non plus de
  confirmation "re-pull ?" si le modèle est déjà installé (§5.7.2 point 3) —
  le pull est relancé directement ; simplification acceptée pour rester
  cohérent avec `ollama pull` en CLI qui a le même comportement.

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

4. **Téléchargement de modèle interactif** : voir §5.7 — dialog dédié pour
   saisir/choisir un modèle et le télécharger sans quitter la TUI, avec barre
   de progression en temps réel.

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

### 5.7 Téléchargement de modèle LLM depuis la TUI (`pull` interactif)

Objectif : l'utilisateur ne doit **pas** avoir besoin de sortir de Crustly
vers un terminal pour faire `ollama pull <model>` — il choisit et télécharge
le modèle qu'il veut directement depuis la TUI, avec suivi de progression en
direct.

#### 5.7.1 Contrainte réelle de l'API Ollama (à documenter pour l'utilisateur)

Ollama **n'expose aucune API locale pour parcourir/rechercher le catalogue en
ligne** (la recherche façon "Ollama Library" n'existe que sur le site
ollama.com). `ollama-rs` ne peut donc pas offrir de recherche catalogue côté
API — seulement :

- `list_local_models()` (`GET /api/tags`) → modèles déjà installés localement
- `pull_model(name, allow_insecure)` (`POST /api/pull`) → télécharge un modèle
  **si son nom `repo:tag` est déjà connu** (ex. `llama3.2:3b`,
  `qwen2.5-coder:7b`, `mistral:latest`)

Le "choix du modèle" dans la TUI est donc une **saisie/sélection du nom de
modèle** (comme le ferait l'utilisateur en CLI avec `ollama pull <name>`),
assistée par une liste de suggestions, **pas** une recherche plein texte sur
le hub Ollama. Ce point sera documenté explicitement dans le README pour ne
pas créer d'attente erronée.

#### 5.7.2 Flux utilisateur (UX)

1. Raccourci dédié en mode `Chat` (proposé : `Ctrl+D` pour "Download" — à
   confirmer, doit être libre dans `src/tui/events.rs`) ou commande slash
   `/pull` dans la barre de saisie existante (cohérent avec le reste de la
   TUI si des commandes slash existent déjà — à vérifier à l'implémentation).
2. Ouverture d'un nouveau dialog **Model Download**, sur le modèle du
   `FilePicker` déjà existant (réutilise `tui-textarea`, déjà une dépendance
   du projet, pour le champ de saisie) :
   - Un champ texte pour taper librement `repo:tag` (ex. `deepseek-r1:14b`).
   - Une liste de suggestions au-dessus/à côté, préremplie avec :
     a) les modèles **déjà installés** (via `list_local_models()`, marqués
        "✅ installed" et non re-téléchargeables sans confirmation), et
     b) une liste **curatée statique** reprise du README existant
        (`qwen2.5-coder:7b`, `gemma3:12b`, `llama3.1:8b`, `mistral`, …,
        section "Recommended Local Models for Coding") embarquée en constante
        Rust (`const CURATED_MODELS: &[(&str, &str)]` = nom + courte
        description), filtrable en tapant (fuzzy-match simple, style
        `FilePicker`).
   - Navigation haut/bas pour sélectionner une suggestion, `Tab`/`Enter` pour
     la copier dans le champ de saisie, `Enter` à nouveau (ou sur champ
     libre) pour lancer le téléchargement.
   - `Esc` ferme le dialog sans rien télécharger.
3. Validation légère avant lancement : format `nom[:tag]` non vide ; si le
   modèle est déjà dans `list_local_models()`, demander confirmation
   ("Re-pull llama3.2:3b ? (y/n)") plutôt que de retélécharger silencieusement.
4. Lancement du pull ⇒ bascule vers un overlay de progression (même famille
   que `render_auto_exec_progress`) :

   ```
   ┌─ Downloading qwen2.5-coder:7b ───────────────────────────────┐
   │ pulling manifest                                              │
   │ pulling 8934d96d3f08... [████████████████░░░░░░░░]  64%      │
   │   2.9 GB / 4.5 GB · 38 MB/s · ETA 42s                         │
   │ verifying sha256 digest                                       │
   │                                                                │
   │                              (Esc annule le téléchargement)   │
   └────────────────────────────────────────────────────────────────┘
   ```

5. À la fin : message de succès dans le fil de chat/toast
   (`✅ qwen2.5-coder:7b downloaded — 4.5 GB in 1m58s`), et proposition
   optionnelle "utiliser ce modèle pour la session courante ?" (met à jour
   `session.model` + `providers.ollama.default_model` en mémoire, sans
   redémarrer l'app).
6. En cas d'échec (nom introuvable côté registre Ollama → 404, disque plein,
   réseau coupé) : message d'erreur clair dans l'overlay, dialog reste ouvert
   pour corriger et relancer.

#### 5.7.3 Détails techniques

- **`src/llm/provider/ollama_models.rs`** (§4.5) : `pull_model` prend un
  `tokio::sync::mpsc::UnboundedSender<PullProgressEvent>` et streame les
  évènements de progression natifs d'`ollama-rs` (`PullModelStatus { status,
  digest, total, completed }`) au fur et à mesure. Agrégation multi-couches :
  Ollama télécharge plusieurs "layers" (blobs) séquentiellement pour un même
  modèle ; l'overlay doit combiner `completed`/`total` de la couche courante
  ET une estimation globale (somme des tailles de couches déjà connues, si
  disponible dans la réponse `/api/pull`, sinon se limiter à la couche
  courante + un indicateur "couche X").
- **Annulation** : le pull tourne dans une `tokio::task::JoinHandle` dédiée ;
  `Esc` envoie un signal d'annulation via un `CancellationToken`
  (déjà utilisé ailleurs dans le projet ? à vérifier — sinon `tokio_util
  ::sync::CancellationToken`, `tokio-util` est déjà une dépendance du
  projet) qui interrompt proprement le stream HTTP en cours.
- **Concurrence** : un seul pull actif à la fois depuis la TUI (verrou
  applicatif simple, ex. `app.active_pull: Option<PullHandle>`) — lancer un
  second téléchargement pendant qu'un premier est en cours affiche un
  message plutôt que de les empiler.
- **Nouveau variant** `AppMode::ModelDownload` dans `src/tui/events.rs`
  (même famille que `FilePicker`/`ToolApproval`), plus un évènement applicatif
  `AppEvent::PullProgress(PullProgressEvent)` et `AppEvent::PullFinished { ok:
  bool, model: String }` traités dans la boucle d'évènements existante
  (à côté de `ToolApprovalRequested`).
- **Rendu** : nouvelle fonction `render_model_download(f, app, area)` dans
  `src/tui/components/dialogs/mod.rs`, suivant exactement le style
  `Clear` + `Block` + `Borders::ALL` déjà utilisé par
  `render_auto_exec_progress`.
- **Persistance** : aucune — l'historique des téléchargements n'a pas besoin
  d'être stocké en base ; seul l'état du modèle localement installé compte,
  et il est déjà source de vérité côté disque/Ollama (`list_local_models()`
  interrogé à chaque ouverture du dialog).
- **CLI équivalente conservée** : `crustly ollama pull <name>` (§4.5) reste
  disponible pour scripts/CI — le dialog TUI est une surface supplémentaire
  au-dessus de la même fonction `pull_model()`, pas une réimplémentation.

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
- **Phase 3 (gestion de modèles + téléchargement interactif)** : ✅
  `list`/`pull`/`rm`/`show` + sous-commande CLI, et **dialog "Model
  Download" dans la TUI** (§5.7, `Ctrl+D`) permettant à l'utilisateur de
  choisir/saisir un modèle et de le télécharger avec barre de progression
  en direct, sans quitter Crustly. Reste non fait : le panneau "Model Info"
  dans la TUI décrit en §5.4 point 3 (`crustly ollama show` en CLI reste
  le seul moyen de voir license/parameters/template/capabilities).
- **Phase 4** : ✅ (partiel) `generate_embeddings()` + `crustly ollama embed`
  exposés comme capacité brute — pas de couche RAG/recherche interne à y
  brancher, Crustly n'en a pas (vérifié, aucune référence à "embedding"
  dans le code avant cette phase).

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
6. ✅ Tranché et implémenté : `Ctrl+D` (mode Chat), libre dans
   `src/tui/events.rs` (`keys::is_model_download`). Pas de commande slash
   séparée — Crustly n'a pas de système de commandes slash dans sa barre de
   saisie, `Ctrl+D` reste donc la seule entrée.
7. ✅ Tranché et implémenté : annulation via `JoinHandle::abort()` sur la
   tâche tokio qui exécute `ollama_models::pull_model()` (stockée dans
   `App::model_download_task`), déclenchée par `Esc`. Pas de
   `CancellationToken` : `abort()` suffit puisque le stream HTTP tourne
   entièrement dans cette tâche.
8. La liste curatée de modèles suggérés (§5.7.2) doit-elle être codée en dur
   dans le binaire (simple, nécessite une recompilation pour la mettre à
   jour) ou chargée depuis un fichier de config/JSON embarqué modifiable par
   l'utilisateur (plus flexible, légère complexité en plus) — recommandation :
   codée en dur pour la v1, réévaluable si demande utilisateur. **Tranché :
   codée en dur** (`ollama_download::CURATED_MODELS`), cohérent avec la
   recommandation ci-dessus.
