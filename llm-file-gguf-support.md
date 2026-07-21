# Spécification d'évaluation : chargement direct de modèles GGUF via `llama-cpp-2`

Statut : **Évaluation — aucune implémentation engagée**
Date : 2026-07-21
Portée : évaluer l'intégration de la crate
[`llama-cpp-2`](https://github.com/utilityai/llama-cpp-rs) (bindings Rust vers
`llama.cpp`) pour permettre à Crustly de charger et exécuter un modèle depuis
un fichier `.gguf` local, **sans dépendre d'un serveur Ollama externe**, dans
l'objectif d'obtenir une application plus monolithique. Bénéfices et coûts
évalués spécifiquement pour ce dépôt.

---

## 0. Résumé exécutif

`llama-cpp-2` est **techniquement le bon candidat** pour cet objectif : c'est
un binding Rust direct vers l'implémentation de référence du format GGUF,
publié sur crates.io, sous licence MIT OR Apache-2.0 (compatible avec la
licence FSL-1.1-MIT de Crustly), maintenu activement (mise à jour début
juillet 2026, 841 745 téléchargements cumulés, 615 ★ sur le dépôt source) et
couvrant nativement tout ce qu'il faut : chargement `.gguf` sans conversion,
backends CPU/CUDA/Metal/Vulkan, génération contrainte par grammaire
(utile pour le tool-calling), et support multimodal (`mtmd`).

**Mais ce n'est pas une intégration "légère".** Contrairement à l'intégration
`ollama-rs` déjà livrée dans ce dépôt (un client HTTP typé vers un serveur
qui fait tout le travail d'inférence), `llama-cpp-2` embarque le **moteur
d'inférence natif C++ lui-même** dans le binaire de Crustly : compilation
CMake d'un sous-module C++ à chaque build, gestion mémoire/concurrence du
modèle dans le process TUI, ré-implémentation du cycle de vie et du
tool-calling que le daemon Ollama gère aujourd'hui gratuitement. C'est un
changement de nature du projet (client réseau → moteur d'inférence embarqué),
pas une extension incrémentale.

**Recommandation : faisable et alignée techniquement, mais à traiter comme un
projet séparé avec spike de validation avant tout engagement**, pas comme un
ajout de provider ordinaire. Voir §7 pour le calcul coût/bénéfice détaillé et
§9 pour la marche à suivre si retenue.

---

## 1. Contexte : pourquoi cette évaluation

Crustly supporte aujourd'hui l'inférence locale via deux chemins, tous deux
**des clients HTTP vers un processus externe déjà démarré** :

| Chemin | Fichier | Mécanisme |
|---|---|---|
| Ollama natif | `src/llm/provider/ollama.rs` | `ollama-rs` → HTTP vers `http://localhost:11434` (`/api/chat`, `/api/pull`, …) |
| Compatible OpenAI (LM Studio, Ollama, LocalAI) | `src/llm/provider/openai.rs` avec `base_url` custom | `async-openai`/HTTP vers un serveur OpenAI-compatible local |

Dans les deux cas, **Crustly ne charge jamais lui-même de poids de modèle** :
il délègue entièrement l'inférence (lecture du `.gguf`, dé-quantization au
runtime, kernels CPU/GPU, gestion mémoire du KV-cache) à un processus tiers.
C'est ce qui rend l'intégration `ollama-rs` relativement légère (voir
`ollama-rs-integration-plan.md`, ~700 lignes de plan pour un client HTTP
typé) : aucun code d'inférence, uniquement du mapping requête/réponse.

L'idée évaluée ici change de nature : **faire tourner l'inférence dans le
process Crustly lui-même**, en lisant un fichier `.gguf` directement depuis
le disque, sans dépendre d'aucun serveur externe démarré au préalable.
Objectif déclaré : une distribution plus monolithique (un seul binaire, une
seule installation, pas de service tiers à gérer).

Le format GGUF est un conteneur binaire (métadonnées + tenseurs quantifiés).
Le "lire" ne suffit pas : il faut aussi **exécuter** le modèle — désérialiser
les tenseurs, dé-quantifier, faire tourner les kernels matmul/attention,
gérer le KV-cache et la fenêtre de contexte, tokenizer le texte. C'est un
moteur d'inférence complet, pas un parseur de fichier. `llama-cpp-2` fournit
exactement ce moteur en embarquant `llama.cpp` (la référence C++ du
domaine) via FFI.

---

## 2. `llama-cpp-2` : ce que c'est

### 2.1 Composition et provenance

Le dépôt [`utilityai/llama-cpp-rs`](https://github.com/utilityai/llama-cpp-rs)
publie deux crates complémentaires sur crates.io :

- **`llama-cpp-2`** — API Rust de haut niveau (modèle, contexte, batch,
  sampling, tokenisation).
- **`llama-cpp-sys-2`** — bindings FFI bruts, générés par `bindgen`, qui
  compilent le sous-module Git vendoré de `llama.cpp` via la crate `cmake`
  au moment du `cargo build`.

Design assumé par les mainteneurs : rester "aussi proche que possible des
bindings bruts" et "aussi à jour que possible avec `llama.cpp`" —
**explicitement pas suivi en SemVer strict** (versionnage actuel :
`llama-cpp-2` v0.1.151, `llama-cpp-sys-2` v0.1.152). C'est un point à noter
pour la maintenance long terme (§6.3).

### 2.2 Licence

`MIT OR Apache-2.0` — pleinement compatible avec la licence FSL-1.1-MIT de
Crustly (`Cargo.toml` ligne 7), sans aucune des contraintes commerciales
qu'aurait posées une dépendance sous licence propriétaire. Aucun blocage
juridique de ce côté.

### 2.3 Maturité

- 841 745 téléchargements cumulés sur crates.io, 430 790 sur la période
  récente (signal d'usage actif, pas seulement historique).
- 615 ★ / 218 forks sur GitHub, 151 versions publiées, dernière mise à jour
  début juillet 2026 — mis à jour au fil des sorties de `llama.cpp`.
- 33 issues ouvertes, 12 PR en cours — niveau d'activité de mainteneur sain
  pour une crate de cette taille.
- Créée en janvier 2024, donc ~2,5 ans d'historique au moment de cette
  évaluation.

### 2.4 Fonctionnalités couvertes (features Cargo)

Extrait du `Cargo.toml` de `llama-cpp-2` :

```toml
[features]
default = ["openmp", "android-shared-stdcxx", "common"]
cuda = ["llama-cpp-sys-2/cuda"]
metal = ["llama-cpp-sys-2/metal"]
vulkan = ["llama-cpp-sys-2/vulkan"]
rocm = ["llama-cpp-sys-2/rocm"]
opencl = ["llama-cpp-sys-2/opencl"]
mkl = ["llama-cpp-sys-2/mkl"]
openmp = ["llama-cpp-sys-2/openmp"]
mtmd = ["llama-cpp-sys-2/mtmd"]                     # multimodal (vision)
llguidance = ["dep:llguidance", "dep:toktrie"]       # génération contrainte par grammaire/JSON
sampler = []
dynamic-link = ["llama-cpp-sys-2/dynamic-link"]
system-ggml = ["llama-cpp-sys-2/system-ggml"]
```

Deux features méritent une mention particulière dans le contexte de
Crustly :

- **`mtmd`** (multimodal) : correspondrait à `Provider::supports_vision()`
  dans le trait actuel (`src/llm/provider/trait.rs` ligne 42) — utile car
  Ollama/OpenAI-compat gèrent déjà la vision aujourd'hui, une intégration
  GGUF native sans cette feature serait une régression fonctionnelle.
- **`llguidance`** (via `llguidance`/`toktrie`, la bibliothèque de génération
  contrainte par grammaire) : permettrait de forcer la sortie du modèle à
  respecter un schéma JSON — une alternative potentiellement **plus fiable**
  que l'heuristique actuelle de récupération de tool calls imprimés en texte
  brut (`maybe_tool_call_json`/`tool_call_from_content` dans `ollama.rs`
  lignes 607-655, et le `ToolCallParser` dédié de `qwen.rs`). C'est un
  bénéfice réel et spécifique à cette crate, pas juste une parité avec
  l'existant.

### 2.5 Exemples fournis (repère pour l'API)

Le dépôt fournit des exemples couvrant : `simple` (chargement modèle +
complétion), `embeddings`, `mtmd` (multimodal), `reranker`, `llguidance.rs`
(génération contrainte), `usage.rs`. L'exemple `simple` démontre le
chargement direct d'un `.gguf` quantifié (`Q4_K_M`, `Q6_K`) téléchargé depuis
HuggingFace — confirmant le chargement natif visé par cette évaluation, sans
étape de conversion.

---

## 3. Alignement avec l'architecture Crustly actuelle

### 3.1 Le trait `Provider` s'y prête sans modification

`src/llm/provider/trait.rs` (lignes 18-65) définit une interface
agnostique du transport : `complete()`, `stream()`,
`supports_streaming/tools/vision()`, `name()`, `default_model()`,
`context_window()`, `calculate_cost()`. Un provider GGUF natif
(`src/llm/provider/gguf.rs`, nouveau fichier) implémenterait ce même trait,
câblé dans `factory.rs` exactement comme `try_create_ollama()` l'est
aujourd'hui — **aucune modification du trait n'est nécessaire**, contrainte
de compatibilité déjà respectée par construction.

`calculate_cost()` retournerait simplement `0.0` (pas d'appel API payant,
comme c'est déjà implicitement le cas pour un modèle local).

### 3.2 Écart de schéma de configuration à combler

`ProviderConfig` générique (`src/config/mod.rs` ligne 346) ne connaît qu'un
`default_model: String` — un nom résolu par un serveur distant, jamais un
chemin de fichier local. Aucun champ existant pour un chemin `.gguf`, un
nombre de threads CPU, un nombre de couches déportées sur GPU
(`n_gpu_layers`), ou une taille de contexte propre à un fichier modèle donné.
Il faudrait un nouveau type de configuration, sur le modèle
d'`OllamaModelConfig` (`src/config/mod.rs` ligne 499, réglages par modèle) ou
de `QwenProviderConfig` (ligne 366, précédent le plus proche pour un
déploiement local avec réglages bas niveau) :

```rust
pub struct GgufProviderConfig {
    pub model_path: PathBuf,       // chemin vers le fichier .gguf
    pub n_ctx: Option<u32>,        // taille de contexte (par défaut celle du modèle)
    pub n_threads: Option<u32>,    // threads CPU pour le décodage
    pub n_gpu_layers: Option<u32>, // couches déportées sur GPU (0 = CPU only)
    // pas de champ "quantization" : intrinsèque au fichier .gguf choisi
}
```

### 3.3 Crabrace n'a aucun rôle ici

Le registre `crabrace.rs` (§0003 des ADR) modélise des providers
**joignables sur le réseau** (API cloud ou daemon local avec surface HTTP).
Un moteur GGUF en process n'a pas de tel point d'entrée à enregistrer — il
contournerait Crabrace entièrement, comme le fait déjà `QwenProvider` pour
ses aspects spécifiques. Pas d'implication architecturale au-delà de ce
constat.

### 3.4 Précédent de feature flag à suivre

`ollama = ["dep:ollama-rs", "dep:schemars"]` dans `Cargo.toml`, gaté de façon
cohérente dans `src/llm/provider/mod.rs` (`#[cfg(feature = "ollama")]`) et
`factory.rs` (avec un stub no-op quand la feature est désactivée). Une
nouvelle feature `gguf` suivrait exactement ce patron :

```toml
[dependencies]
llama-cpp-2 = { version = "0.1", optional = true, features = ["openmp"] }

[features]
gguf = ["dep:llama-cpp-2"]
all-llm = ["openai", "aws-bedrock", "ollama", "gguf"]
```

Optionnelle par défaut, cohérent avec la volonté actuelle de ne pas alourdir
le binaire par défaut (déjà la politique retenue pour `ollama`, cf. point
ouvert §9 de `ollama-rs-integration-plan.md`).

---

## 4. Bénéfices

1. **Chargement `.gguf` natif, sans conversion.** Contrairement à toute
   solution qui imposerait un format propriétaire intermédiaire,
   `llama-cpp-2` lit directement le format GGUF de référence — n'importe
   quel fichier `.gguf` déjà téléchargé (HuggingFace, `ollama pull` puis
   export, etc.) est utilisable tel quel.
2. **Zéro dépendance de service externe.** Plus besoin qu'Ollama (ou un
   autre serveur) tourne en arrière-plan ; un utilisateur avec un fichier
   `.gguf` sur disque peut lancer Crustly directement, y compris en
   environnement air-gapped/restreint où l'installation d'un daemon tiers
   n'est pas possible.
3. **Distribution monolithique.** Un seul binaire à installer (au prix d'une
   taille sensiblement plus grande, voir §5.1).
4. **Contrôle fin du cycle de vie du modèle** dans le process Crustly
   (chargement, déchargement, paramètres d'inférence bas niveau) sans passer
   par l'API HTTP d'un tiers.
5. **Backends matériels larges** : CPU (avec OpenMP), CUDA, Metal (macOS),
   Vulkan, ROCm, OpenCL, MKL — couvre l'essentiel du matériel visé par les
   utilisateurs desktop de Crustly (Linux/macOS/Windows, x86_64/arm64),
   contrairement à un moteur mobile-first limité à ARM NEON/Metal.
6. **Génération contrainte par grammaire (`llguidance`)** — un gain
   fonctionnel *au-delà* de la simple parité avec Ollama : une sortie JSON
   garantie valide au niveau du décodage token-par-token serait plus fiable
   que l'heuristique actuelle de récupération de tool calls textuels
   (`ollama.rs` lignes 607-655), pour peu que le travail d'intégration soit
   fait (§5.2).
7. **Support multimodal (`mtmd`)** disponible nativement, évitant une
   régression de fonctionnalité par rapport à Ollama/OpenAI-compat qui
   supportent déjà la vision.
8. **Latence de démarrage** potentiellement meilleure pour un usage
   ponctuel/CLI (pas de round-trip HTTP, pas de "cold start" d'un service
   externe) — à mesurer concrètement, le chargement mmap d'un gros `.gguf`
   restant coûteux dans tous les cas.

---

## 5. Coûts

### 5.1 Build et packaging

- `llama-cpp-sys-2` compile le sous-module Git vendoré de `llama.cpp` via la
  crate `cmake` à chaque `cargo build` (à moins d'utiliser `system-ggml` avec
  une lib système préinstallée) — nécessite **CMake et un compilateur C++**
  sur toute machine qui build Crustly avec la feature `gguf` activée. Rompt
  la promesse actuelle de `CLAUDE.md` ("Development build: `cargo build`",
  sans toolchain externe) pour quiconque active cette feature.
- **Windows** : le `build.rs` distingue explicitement MSVC (`.lib`) et
  MinGW/GNU (`.a`), avec des drapeaux spécifiques (`/O2`, `/DNDEBUG`, `/FS`,
  désactivation de `TrackFileAccess`) — signale un chemin de build testé
  mais qui ajoute une exigence d'outillage (Visual Studio Build Tools avec
  charge de travail C++, ou toolchain MinGW) absente aujourd'hui du poste de
  développement Crustly type.
- **CUDA/Vulkan/ROCm/MKL** ne sont pas auto-détectés : CUDA nécessite un
  toolkit installé, Vulkan nécessite la variable d'environnement
  `VULKAN_SDK`, etc. — chaque backend GPU est une case de configuration
  manuelle supplémentaire pour l'utilisateur qui build depuis les sources.
- Temps de compilation à froid fortement augmenté (compilation C++ native
  de `llama.cpp`, en plus du Rust).
- Taille du binaire final en hausse significative (moteur natif + kernels
  backend embarqués), sans compter le poids des fichiers `.gguf`
  eux-mêmes (plusieurs Go par modèle, à gérer côté disque indépendamment
  du mécanisme de chargement).

### 5.2 Nouveau code applicatif

- Nouveau provider (`src/llm/provider/gguf.rs`) : chargement du modèle,
  création du contexte d'inférence, tokenisation, décodage par batch,
  sampling (température/top-p/top-k, feature `sampler`), reconstruction de
  `StreamEvent` token par token — sur le même patron que `stream()` dans
  `ollama.rs` (ligne 543), qui est déjà le meilleur précédent en place pour
  un backend local "from scratch" (accumulation des deltas, construction
  manuelle de `MessageStart`/`ContentBlockDelta`/`MessageStop`).
- **Tool/function calling à ré-implémenter** : `llama-cpp-2` ne fait aucune
  hypothèse sur le format de sortie d'un modèle — contrairement à Ollama qui
  gère aujourd'hui le tool-calling côté serveur. Deux options :
  - reprendre l'approche heuristique déjà en place (`maybe_tool_call_json`/
    `tool_call_from_content` dans `ollama.rs`, `ToolCallParser` de
    `qwen.rs`) — coût de portage modéré, fiabilité identique à l'existant ;
  - investir dans `llguidance` (génération contrainte par grammaire) pour
    une fiabilité supérieure — coût d'intégration plus élevé (apprentissage
    de l'API `llguidance`/`toktrie`, définition d'une grammaire JSON Schema
    à partir des `Tool.input_schema` déjà génériques dans
    `provider/types.rs`), mais bénéfice net réel (§4 point 6).
- Gestion mémoire/concurrence : un modèle chargé occupe la RAM pendant toute
  la durée de vie du process Crustly (ou nécessite une logique de
  déchargement maison, ré-implémentant le `keep_alive` qu'Ollama gère
  aujourd'hui) ; le décodage CPU doit être isolé dans une tâche
  `tokio::task::spawn_blocking` pour ne pas geler la boucle d'événements TUI
  pendant l'inférence — un risque absent aujourd'hui puisque l'inférence
  tourne toujours dans un process séparé (Ollama).
- **Gestion de catalogue/téléchargement de modèles** : avec un fichier
  `.gguf` brut, la responsabilité de trouver, télécharger et vérifier
  l'intégrité (checksum) du fichier revient entièrement à Crustly — Ollama
  gère aujourd'hui cela via `ollama pull <repo:tag>`, déjà exposé dans
  Crustly (`crustly ollama pull`, dialog TUI `Ctrl+D`, cf.
  `ollama-rs-integration-plan.md` §5.7). Un provider GGUF natif perd cette
  capacité sauf à la ré-implémenter (résolution d'URL HuggingFace,
  téléchargement avec barre de progression, vérification de checksum).

### 5.3 Maintenance long terme

- **Pas de SemVer strict** (assumé par les mainteneurs de `llama-cpp-rs`,
  qui priorisent le suivi de `llama.cpp` en amont) : des mises à jour de
  version peuvent introduire des changements d'API non annoncés par un bump
  majeur classique — nécessite un pin de version plus prudent et une revue
  de changelog à chaque mise à jour, contrairement aux autres dépendances du
  projet qui suivent SemVer.
- Nouvelle surface de sécurité : exécution de code natif C++ dans le même
  process que le reste de l'application, sur un fichier binaire fourni par
  l'utilisateur (`.gguf`) — à contraster avec le modèle actuel où Ollama
  tourne en process séparé (isolation naturelle des crashs/fuites mémoire du
  moteur d'inférence vis-à-vis du reste de Crustly : TUI, DB, exécution des
  tools).
- Dépendance C++ vendorée à suivre hors de l'écosystème `cargo update`
  standard (mises à jour de sécurité/performance de `llama.cpp` en amont).
- **Tests CI plus difficiles qu'avec Ollama** : le plan d'intégration
  `ollama-rs` note déjà qu'un test contre un vrai serveur Ollama est "non
  exécutable en CI" (test manuel local). Avec un moteur embarqué, il n'y a
  même plus de serveur à mocker/stubber facilement pour les tests
  d'intégration légers — il faudrait soit invoquer un vrai `.gguf` (poids de
  plusieurs centaines de Mo minimum à héberger/télécharger en CI), soit se
  limiter à des tests unitaires sur le mapping de types (comme
  aujourd'hui), sans jamais exercer le chemin d'inférence réel en CI.

### 5.4 Ce qui est perdu par rapport à Ollama

- **Partage de modèle entre plusieurs process** : Ollama charge un modèle
  une fois et sert plusieurs clients depuis le même processus serveur ; un
  moteur embarqué dans Crustly rechargerait le modèle à chaque instance de
  Crustly lancée (RAM et temps de démarrage multipliés par le nombre de
  sessions concurrentes).
- **Déchargement automatique par inactivité** (`keep_alive` d'Ollama) — à
  réimplémenter côté Crustly si souhaité.
- **Catalogue de modèles nommés** (`llama3.2:3b`, etc.) — avec un `.gguf`
  brut, l'utilisateur gère lui-même ses fichiers, sans le confort d'un nom
  court résolu automatiquement.

---

## 6. Estimation d'effort (ordre de grandeur)

À titre de comparaison, l'intégration `ollama-rs` (client HTTP typé,
documentée dans `ollama-rs-integration-plan.md`) a représenté 4 phases
livrées progressivement, sans aucun code d'inférence à écrire. Un provider
GGUF natif via `llama-cpp-2` couvre un périmètre fonctionnel comparable
**plus** un moteur d'inférence embarqué à intégrer, tester et maintenir :
raisonnablement, **plusieurs semaines** de travail pour un MVP correct
(build multi-plateforme, chargement modèle, complétion, streaming,
tool-calling basique par heuristique texte), et significativement plus si
`llguidance` est intégré dès le MVP plutôt que dans une itération
ultérieure. Contre quelques jours pour un client HTTP vers un serveur déjà
bâti et testé par une large communauté (Ollama).

Le rapport effort/bénéfice n'est positif que si "zéro dépendance de service
externe" est une **exigence produit dure** (usage air-gapped, distribution à
des utilisateurs sans droits d'installation de service tiers) — pas une
simple préférence de simplicité perçue, largement déjà couverte
aujourd'hui par Ollama qui s'installe et démarre automatiquement en
arrière-plan sur les trois plateformes cibles.

---

## 7. Tableau de synthèse

| Critère | Statu quo (Ollama natif + compat OpenAI) | `llama-cpp-2` (moteur embarqué) |
|---|---|---|
| Charge un `.gguf` sans conversion | Délégué à Ollama (oui, côté serveur) | ✅ natif, in-process |
| Dépendance à un service externe | Oui (daemon Ollama) | ❌ aucune |
| Licence | — | ✅ MIT OR Apache-2.0, compatible FSL-1.1-MIT |
| Toolchain de build | `cargo build` seul | CMake + compilateur C++ requis |
| Tool-calling | Géré côté serveur Ollama (+ heuristique texte côté Crustly) | À ré-implémenter (heuristique ou `llguidance`) |
| Gestion catalogue/téléchargement modèles | ✅ déjà livré (`crustly ollama pull`, TUI) | À ré-implémenter |
| Partage modèle multi-process | ✅ (un seul chargement, N clients) | ❌ (un chargement par instance Crustly) |
| Isolation crash/mémoire du moteur d'inférence | ✅ (process séparé) | ❌ (même process que la TUI/DB) |
| Effort d'intégration | Déjà fait | Plusieurs semaines (MVP) |
| Testabilité CI | Limitée (test manuel documenté) | Plus limitée encore (pas de serveur à mocker) |

---

## 8. Questions ouvertes avant toute décision

1. Le besoin "monolithique, pas de service externe" est-il une **contrainte
   produit ferme** (usage air-gapped, distribution à des utilisateurs sans
   droits d'installation de service tiers) ou une **préférence de confort** ?
   La réponse change complètement le calcul coût/bénéfice du §6.
2. Quel périmètre de backends est requis au lancement : CPU seulement (plus
   simple à builder/tester, plus lent) ou CPU+GPU (CUDA/Metal, complexité de
   build et matrice de test multipliées) ?
3. Le tool/function calling (utilisé massivement par les 21+ tools de
   Crustly) est-il requis dès le MVP GGUF natif, ou une première version
   "chat simple, pas de tools" est-elle acceptable pour valider l'intérêt
   avant d'investir dans le parsing de tool calls sur un moteur brut ?
4. Faut-il viser `llguidance` (génération contrainte) dès le MVP, ou
   commencer par l'heuristique texte déjà éprouvée dans `ollama.rs`/`qwen.rs`
   et migrer plus tard si le besoin de fiabilité le justifie ?
5. Le binaire par défaut de Crustly doit-il rester sans la feature `gguf`
   (comme c'est le cas pour `ollama` aujourd'hui — optionnelle, pas dans
   `default`), ou cette capacité est-elle jugée assez centrale pour changer
   cette politique ?
6. Qui gère la vérification d'intégrité (checksum) et la provenance des
   fichiers `.gguf` fournis par l'utilisateur, sachant qu'ils sont exécutés
   par du code natif dans le même process que le reste de l'application ?

---

## 9. Suite à donner

Ce document est un **plan d'évaluation**, pas une décision.

- **Aucune décision** (statu quo) → rien à faire, ce document reste comme
  trace de l'évaluation.
- **Décision d'intégrer `llama-cpp-2`** → recommandé de commencer par un
  **spike technique isolé** (hors branche principale) : charger un petit
  `.gguf` (ex. modèle 1-3B), obtenir une complétion et un streaming
  fonctionnels, mesurer le temps de build à froid et la taille du binaire
  réels sur les trois plateformes cibles, avant tout engagement sur le plan
  complet. Documenter ensuite le choix dans une ADR dédiée
  `docs/architecture/decisions/0005-<titre>.md` (Context/Decision/
  Consequences, cf. gabarit `0000-adr-template.md`) qui renverra vers ce
  fichier pour le détail — même articulation que l'ADR `0003` vers
  `docs/guides/CRABRACE_INTEGRATION.md`. L'implémentation suivrait ensuite le
  même triptyque que l'intégration Ollama : plan d'intégration détaillé →
  plan de test → guide utilisateur (`docs/guides/`), sur le modèle de
  `ollama-rs-integration-plan.md` + `ollama-local-llm-test-plan.md` +
  `docs/guides/OLLAMA_GUIDE.md`.

---

## Sources consultées

- [utilityai/llama-cpp-rs](https://github.com/utilityai/llama-cpp-rs) — README, structure du dépôt, exemples
- [llama-cpp-2 sur crates.io](https://crates.io/crates/llama-cpp-2) — version, licence, statistiques de téléchargement
- `llama-cpp-2/Cargo.toml` et `llama-cpp-sys-2/build.rs` (dépôt `utilityai/llama-cpp-rs`) — features Cargo et logique de build CMake/Windows
- `ollama-rs-integration-plan.md` (ce dépôt) — référence de comparaison d'effort et patrons d'implémentation (streaming, tool-calling heuristique, feature flags)
- `docs/architecture/decisions/0003-crabrace-provider-registry.md` — architecture de découverte de providers existante
- `src/llm/provider/trait.rs`, `src/llm/provider/ollama.rs`, `src/config/mod.rs`, `Cargo.toml` (ce dépôt) — interface `Provider`, conventions de feature flags et écarts de schéma de configuration actuels
