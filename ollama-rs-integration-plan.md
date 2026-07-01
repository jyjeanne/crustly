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

L'intérêt de `ollama-rs` est donc de débloquer la **gestion de modèles** (pull
depuis Crustly, liste des modèles installés, suppression) et les
**embeddings**, tout en gardant une implémentation plus fidèle au protocole
natif qu'un mapping OpenAI approximatif.

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
5. Tous les tests existants (`cargo test`) doivent continuer à passer.

## 4. Ce qui va changer / être ajouté

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
  `ContentBlock::ToolUse` pour les tool calls et `TokenUsage` depuis
  `prompt_eval_count` / `eval_count`
- `stream()` : consommer le stream natif d'`ollama-rs` et le traduire vers
  `StreamEvent` (`MessageStart` → `ContentBlockDelta` → `MessageStop`), en
  suivant le même séquencement que celui déjà présent dans `openai.rs`
  (accumulation des fragments de tool calls avant de les émettre)
- `supports_vision()` : réutiliser la même liste de motifs de noms de modèles
  que `openai.rs` (`llava`, `vision`, `minicpm-v`, etc.), potentiellement
  extraite dans une fonction partagée `super::model_hints::is_vision_model()`
  pour éviter la duplication entre les deux providers
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

- `list_models()`, `pull_model(name, on_progress)`, `delete_model(name)`

Exposés via une sous-commande CLI `crustly ollama <list|pull|rm>` dans
`src/cli/mod.rs` (nouveau, additif — n'affecte pas les commandes existantes).
Cette partie est **optionnelle** pour une v1 : elle peut être livrée dans une
seconde phase.

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
    modèles + embeddings)

## 5. Ce qui NE change PAS

- `OpenAIProvider`, `AnthropicProvider`, `QwenProvider`, `AzureOpenAIProvider` :
  aucune modification de comportement.
- Le trait `Provider` : signature inchangée.
- `crabrace` (registry de providers) : pas de dépendance croisée avec ce plan
  sauf si l'on souhaite y déclarer Ollama comme provider connu (à évaluer
  séparément, hors scope v1).
- Comportement par défaut de `create_provider()` sans config `ollama` :
  identique bit-à-bit à l'existant.

## 6. Plan de test

1. Tests unitaires (offline, sans dépendre d'un serveur Ollama réel) :
   - construction du provider, mapping requête/réponse, mapping d'erreurs —
     même style que les tests déjà présents dans `openai.rs`/`qwen.rs`.
2. Tests de non-régression : `cargo test --no-default-features` et
   `cargo test --features all-llm` doivent tous les deux passer.
3. Test manuel local (nécessite un Ollama réel, non exécutable en CI) :
   - `ollama pull llama3.2` puis validation chat/stream/tool-call/pull via
     `providers.ollama`.
4. Vérifier que la configuration existante (`providers.openai.base_url =
   "http://localhost:11434/v1"`) continue de fonctionner à l'identique en
   parallèle (non-régression LM Studio/Ollama-compat).

## 7. Phasage proposé

- **Phase 1 (MVP)** : dépendance optionnelle + `OllamaProvider` (chat,
  streaming, tool calls, vision) + config + factory + doc. Livrable testable
  isolément, sans toucher aux autres providers.
- **Phase 2** : gestion de modèles (`list`/`pull`/`rm`) + sous-commande CLI.
- **Phase 3** : embeddings natifs Ollama, exposés à la couche RAG/recherche
  interne si Crustly en a une (à confirmer selon le code existant).

## 8. Points ouverts à trancher avant implémentation

1. Nom de la variable d'environnement : `OLLAMA_HOST` (convention officielle
   Ollama) vs `OLLAMA_BASE_URL` (cohérence avec `OPENAI_BASE_URL`/`QWEN_BASE_URL`
   internes) — recommandation : accepter les deux, `OLLAMA_HOST` prioritaire.
2. Faut-il activer `ollama` dans la feature `default` ou la garder strictement
   optionnelle (recommandé, pour ne pas alourdir le binaire par défaut) ?
3. Faut-il, à terme, migrer le tool-parsing Ollama actuel (via le shim OpenAI)
   vers le provider natif, ou garder les deux indéfiniment ? → recommandation :
   garder les deux, laisser l'utilisateur choisir via sa config.
