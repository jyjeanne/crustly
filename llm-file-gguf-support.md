# Spécification d'évaluation : chargement direct de modèles GGUF (sans Ollama)

Statut : **Évaluation — aucune implémentation engagée**
Date : 2026-07-21
Portée : évaluer l'intégration de tout ou partie de
[`cactus-compute/cactus`](https://github.com/cactus-compute/cactus) (ou d'une
alternative) pour permettre à Crustly de charger et exécuter un modèle depuis
un fichier `.gguf` local, **sans dépendre d'un serveur Ollama externe**, dans
l'objectif d'obtenir une application plus monolithique.

---

## 0. Résumé exécutif

**Recommandation : ne pas intégrer `cactus-compute/cactus`.** Trois blocages
indépendants, chacun suffisant à lui seul :

1. **Licence incompatible avec la distribution actuelle de Crustly.** Cactus
   est sous licence propriétaire (Cactus Compute, Inc.), gratuite seulement
   pour les particuliers, les organismes à but non lucratif, et les
   organisations dont le financement **et** le chiffre d'affaires sont tous
   deux inférieurs à 2 000 000 USD — avec **résiliation automatique** au
   franchissement de l'un des deux seuils. Intégrer ce moteur dans un binaire
   Crustly redistribué publiquement reporterait cette contrainte sur chaque
   utilisateur/redistributeur en aval, ce que Crustly (sous FSL-1.1-MIT, un
   modèle "source-available" qui redevient MIT après 2 ans, mais reste
   librement redistribuable dès maintenant) ne peut pas garantir de respecter.
2. **Depuis sa v1, Cactus ne charge plus directement des fichiers `.gguf`** :
   le moteur est passé d'un pipeline basé GGUF à un format propriétaire
   ("Cactus bundle") produit par `cactus convert`, avec une quantization
   maison (CQ2–CQ4). La documentation marketing continue de mentionner "tout
   modèle GGUF depuis HuggingFace", mais le mécanisme réel est une
   **conversion**, pas un chargement natif — l'objectif "lire un `.gguf`
   existant sans étape intermédiaire" n'est donc pas rempli tel quel.
3. **Cible plateforme désalignée.** Cactus est conçu et optimisé pour
   mobile/edge (iOS, Android, ARM NEON, Metal) ; ses bindings Rust
   (`bindings/rust/cactus.rs`) sont un wrapper FFI fin au-dessus d'une
   bibliothèque native précompilée par plateforme (toolchain CMake/Xcode/NDK
   requise), et ne sont **pas publiés sur crates.io**. Crustly cible des
   postes de développement Linux/macOS/Windows x86_64/arm64 en `cargo build`
   simple — le pipeline de build de Cactus n'a pas cette garantie.

Si l'objectif "monolithique, zéro dépendance externe, lire un `.gguf`
directement" reste souhaité, la voie techniquement correcte est un binding
Rust vers **llama.cpp** (ex. crate `llama-cpp-2`), qui lit nativement le
format GGUF de référence et est disponible sur crates.io sous licence MIT.
Mais c'est un projet d'ampleur différente d'une intégration de client HTTP
(comparable à l'intégration `ollama-rs` déjà faite) : c'est l'embarquement
d'un moteur d'inférence natif C++ dans le binaire, avec les coûts de build,
de taille binaire et de maintenance que cela implique (détaillés en §6).
**Recommandation secondaire : traiter cette option comme une ADR séparée,
précédée d'un spike technique isolé, avant tout engagement de développement.**

---

## 1. Contexte : pourquoi cette évaluation

Crustly supporte aujourd'hui l'inférence locale via deux chemins, tous deux
**des clients HTTP vers un processus externe** :

| Chemin | Fichier | Mécanisme |
|---|---|---|
| Ollama natif | `src/llm/provider/ollama.rs` | `ollama-rs` → HTTP vers `http://localhost:11434` (`/api/chat`, `/api/pull`, …) |
| Compatible OpenAI (LM Studio, Ollama, LocalAI) | `src/llm/provider/openai.rs` avec `base_url` custom | `async-openai`/HTTP vers un serveur OpenAI-compatible local |

Dans les deux cas, **Crustly ne charge jamais lui-même de poids de modèle** :
il délègue entièrement l'inférence (chargement du `.gguf`, quantization au
runtime, kernels CPU/GPU, gestion mémoire du KV-cache) à un processus tiers
déjà démarré. C'est ce qui rend l'intégration `ollama-rs` relativement légère
(voir `ollama-rs-integration-plan.md`, ~700 lignes de plan pour un client HTTP
typé) : aucun code d'inférence, juste du mapping requête/réponse.

L'idée évaluée ici est différente en nature : **faire tourner l'inférence
dans le process Crustly lui-même**, en lisant un fichier `.gguf` directement
depuis le disque, sans dépendre d'Ollama (ni d'aucun autre serveur externe)
étant démarré au préalable. Objectif déclaré : une distribution plus
monolithique (un seul binaire, une seule installation, pas de service tiers à
gérer).

---

## 2. Ce qu'implique réellement "charger un GGUF sans Ollama"

Le format GGUF est un conteneur binaire (métadonnées + tenseurs quantifiés).
Le "lire" ne suffit pas : il faut aussi **exécuter** le modèle, ce qui
nécessite un moteur d'inférence complet :

- désérialisation GGUF (métadonnées, vocabulaire, tenseurs)
- dé-quantization / kernels de calcul (matmul, attention) pour chaque format
  de quantization supporté (Q4_K_M, Q5_K_S, Q8_0, …)
- gestion du KV-cache et de la fenêtre de contexte
- backends d'exécution (CPU SIMD — AVX2/NEON —, et idéalement GPU — CUDA,
  Metal, Vulkan — pour des débits utilisables)
- tokenizer(s) compatibles (BPE, SentencePiece, selon la famille de modèle)
- gestion mémoire (mmap du fichier modèle, allocation du KV-cache)

C'est un sous-système d'une toute autre échelle que celui écrit pour
`ollama.rs` (client HTTP typé). Concrètement, cela revient à embarquer
l'équivalent de **llama.cpp** (ou un moteur propriétaire qui en tient lieu)
dans le binaire de Crustly. Aucun projet sérieux dans cet espace ne réécrit ce
moteur en Rust pur pour cet usage — l'écosystème mûr (`llama.cpp`, `candle`,
`mistral.rs`) s'appuie soit sur du C++/CUDA existant via FFI, soit sur des
kernels dédiés très lourds à maintenir.

---

## 3. Analyse de `cactus-compute/cactus`

### 3.1 Nature du projet

Cactus est un moteur d'inférence "hybride edge-cloud", **conçu pour mobile et
wearables** ("Tiny AI for tiny devices") : iOS, Android, macOS, iPad, Vision
Pro. Il expose des bindings Swift, Kotlin, Flutter, React Native, Python,
Rust, C/C++, avec une API façon OpenAI (`cactus_init`, `cactus_complete`) et
un mode serveur (`cactus serve`, HTTP compatible OpenAI). Projet actif et
soutenu (Y Combinator, ~5.5k ★, releases régulières jusqu'à v2.0.1 début
juillet 2026).

### 3.2 Moteur d'inférence : plus du tout basé sur llama.cpp/GGUF

Point le plus important pour cette évaluation : **la v1 de Cactus a
abandonné GGUF pour un format propriétaire**, dans un objectif de
performance sur device mobile :

- `cactus-graph` : graphe de calcul "zero-copy"
- `cactus-kernels` : kernels SIMD ARM NEON écrits sur mesure par appareil
- `cactus-quants` : quantization "par rotation" maison, 1 à 4 bits (CQ2, CQ3,
  CQ2.54, CQ3.26, CQ4)

Le flux normal est `cactus convert <modèle HuggingFace>` → bundle Cactus
propriétaire (poids quantifiés + graphe d'exécution), ou `cactus download`
pour récupérer un bundle déjà converti. La documentation/marketing continue
d'affirmer que "tout modèle GGUF sur HuggingFace" est supporté "via
migration", mais cela décrit une **conversion vers le format propriétaire**,
pas un chargement natif d'un `.gguf` existant tel quel — ce qui ne correspond
pas à l'objectif "lire directement un fichier `.gguf`" évalué ici. Les
sources publiques sur ce point sont partiellement contradictoires (pages
marketing vs. documentation technique) ; à re-vérifier auprès du mainteneur
avant toute décision si cette piste était malgré tout retenue.

**Conséquence directe** : intégrer Cactus n'apporte pas "charger un `.gguf`
sans conversion" — il faudrait de toute façon une étape de conversion
propriétaire, comparable en complexité opérationnelle à `ollama pull`
aujourd'hui, mais avec un outil et un format supplémentaires à maintenir.

### 3.3 Licence — blocage principal

Licence custom "Cactus Compute, Inc." (ni MIT/Apache, ni copyleft
classique — source-available à conditions commerciales) :

- Gratuit pour : particuliers (usage personnel/éducatif/recherche/non
  commercial), établissements d'enseignement, associations 501(c)(3), et
  organisations dont le **financement total < 2 M USD ET le chiffre
  d'affaires annuel brut < 2 M USD** (les deux conditions, cumulatives).
- **Résiliation automatique** : si une organisation qui remplissait ces
  critères dépasse l'un des deux seuils, l'autorisation s'arrête
  immédiatement et une licence commerciale doit être obtenue sous 30 jours.
- Toute entité hors de ces catégories (y compris dès le départ) doit obtenir
  une licence commerciale séparée.

**Pourquoi c'est bloquant pour Crustly spécifiquement** : Crustly est
lui-même distribué sous FSL-1.1-MIT (Functional Source License) — un modèle
de licence "à conditions" mais qui reste librement utilisable/redistribuable
dès aujourd'hui, et qui redevient MIT pur après 2 ans par version. Crustly ne
contrôle pas qui télécharge/recompile/redistribue son binaire ni la taille de
l'organisation de chaque utilisateur final. Embarquer une dépendance dont la
licence dépend de la taille financière de *l'utilisateur final de Crustly*
(pas de Crustly lui-même) transfère une contrainte de conformité
imprévisible et non vérifiable à toute la chaîne de distribution — un risque
qu'un outil dev "cargo install"-able / packagé (Homebrew, binaires GitHub
Releases, etc.) ne peut pas absorber proprement sans segmenter sa
distribution (build "avec Cactus" réservé à certains utilisateurs vs. build
public sans). C'est une complexité de packaging et un risque juridique que le
gain fonctionnel ne justifie pas.

### 3.4 Bindings Rust et intégration technique

- `bindings/rust/` ne contient que `cactus.rs` (un fichier unique avec
  attributs `#[link(...)]`) + un README expliquant de le copier dans son
  projet et de pointer `build.rs` vers le répertoire de build de la
  bibliothèque native précompilée. **Pas de crate publié sur crates.io.**
- La bibliothèque native (`cactus-engine`) doit être compilée/liée par
  plateforme cible (toolchain CMake/Xcode/NDK selon iOS/Android/macOS) —
  aucune garantie de portabilité "out of the box" vers Linux x86_64/Windows,
  les plateformes desktop réellement ciblées par Crustly (`cargo build`,
  `cargo build --release` sans dépendance externe, tel que documenté dans
  `CLAUDE.md`).
- API exposée : `cactus_init`, `cactus_complete`, fonctions de streaming
  transcription — orientée mobile-app-embarquée, pas serveur/CLI desktop.

### 3.5 Maturité et stabilité d'API

Projet actif, mais le changement de format de modèle entre versions majeures
(GGUF → propriétaire à la v1, refonte "complète" du moteur d'inférence selon
les notes de version) signale une **API/format encore mouvants** — un risque
de maintenance supplémentaire si Crustly devait suivre ces changements.

---

## 4. Alternative techniquement alignée : `llama-cpp-2` (bindings Rust vers llama.cpp)

Si l'objectif reste "lire un `.gguf` sans conversion, sans serveur externe",
la voie correcte est un binding Rust direct vers **llama.cpp**, la
implémentation de référence du format GGUF :

| Critère | `llama-cpp-2` / `llama_cpp` (crates.io) | Cactus |
|---|---|---|
| Lit un `.gguf` existant sans conversion | ✅ natif | ❌ conversion requise |
| Licence | MIT (llama.cpp) — compatible FSL-1.1-MIT | ❌ propriétaire, seuils financiers |
| Publié sur crates.io | ✅ | ❌ (copier-coller de fichier) |
| Backends | CPU (AVX2/NEON), CUDA, Metal, Vulkan | ARM NEON / Metal (mobile-first) |
| Cible plateforme | Linux/macOS/Windows desktop — aligné Crustly | iOS/Android/mobile-first |
| Maturité écosystème | Très large (des dizaines de milliers de projets s'appuient sur llama.cpp) | Plus jeune, API en mouvement |

Ce chemin est **faisable**, mais reste un projet d'ampleur nettement
supérieure à l'intégration `ollama-rs` existante, pour les raisons du §5.

---

## 5. Bénéfices réels d'un chargement GGUF natif (indépendamment du fournisseur)

1. **Zéro dépendance de service externe** : plus besoin qu'Ollama tourne en
   arrière-plan ; un utilisateur avec juste un fichier `.gguf` sur disque
   peut lancer Crustly directement.
2. **Distribution monolithique** : un seul binaire à installer (au prix d'une
   taille de binaire nettement plus grande, voir §6).
3. **Contrôle fin du cycle de vie du modèle** dans le process Crustly
   (chargement, déchargement, paramètres d'inférence) sans passer par l'API
   HTTP d'un tiers.
4. **Latence de démarrage** potentiellement meilleure pour un usage
   ponctuel/CLI (pas de round-trip HTTP, pas de "cold start" du service
   Ollama) — à mesurer, pas garanti (le chargement mmap d'un gros `.gguf`
   reste coûteux dans tous les cas).

Bénéfices **non obtenus** par rapport à Ollama tel qu'utilisé aujourd'hui :

- **Partage de modèle entre plusieurs process** : Ollama charge un modèle une
  fois et sert plusieurs clients (Crustly + autre outil) depuis le même
  processus ; un moteur embarqué dans Crustly rechargerait le modèle à
  chaque instance de Crustly lancée (RAM et temps de démarrage multipliés
  par le nombre de sessions concurrentes).
- **Gestion de catalogue/téléchargement de modèles** (`ollama pull
  <nom:tag>`, déjà exposé dans Crustly via `crustly ollama pull` et le
  dialog TUI `Ctrl+D`, cf. `ollama-rs-integration-plan.md` §5.7) : avec un
  fichier `.gguf` brut, cette responsabilité (trouver, télécharger, vérifier
  l'intégrité du fichier) reviendrait entièrement à Crustly.
- **Déchargement automatique par inactivité** (`keep_alive` d'Ollama) —
  géré aujourd'hui par le daemon, à réimplémenter côté Crustly.

---

## 6. Coûts d'une intégration native (`llama-cpp-2` ou équivalent)

### 6.1 Build et packaging

- Compilation C++/CUDA/Metal de llama.cpp à intégrer au pipeline
  `cargo build` (via `cmake`/`bindgen`), ou distribution de binaires
  précompilés par plateforme+backend (Linux CPU, Linux CUDA, macOS Metal,
  Windows CPU, Windows CUDA, …) — matrice de build significativement plus
  large que celle actuelle (`cargo build`/`cargo build --release`, aucune
  toolchain externe requise aujourd'hui).
- Temps de compilation à froid fortement augmenté (compilation C++ native en
  plus du Rust).
- Taille du binaire final en forte hausse (moteur natif + éventuels kernels
  CUDA embarqués), sans compter le poids des fichiers `.gguf` eux-mêmes
  (plusieurs GB par modèle, à gérer côté disque quel que soit le mécanisme
  de chargement).

### 6.2 Nouveau code applicatif

- Nouveau provider `src/llm/provider/gguf.rs` (ou équivalent) implémentant le
  trait `Provider` existant (`src/llm/provider/trait.rs`, lignes 18-65) :
  `complete()`, `stream()`, mapping vers
  `LLMRequest`/`LLMResponse`/`StreamEvent`. Câblage dans `factory.rs` sur le
  même modèle que `try_create_ollama()`.
- **Écart de schéma de configuration à combler** : `ProviderConfig`
  générique (`src/config/mod.rs` ligne 346) ne connaît qu'un
  `default_model: String` (un nom résolu par un serveur distant), jamais un
  chemin de fichier. Aucun champ existant pour un chemin `.gguf`, un niveau
  de quantization, un nombre de threads CPU, ou un nombre de couches
  déportées sur GPU (`n_gpu_layers`) — il faudrait un nouveau
  `GgufProviderConfig` sur le modèle de `QwenProviderConfig` (ligne 366, déjà
  le précédent le plus proche pour un déploiement "local" avec réglages bas
  niveau) ou d'`OllamaModelConfig` (ligne 499, réglages par modèle).
- **Contrairement à `ollama.rs`, ce provider doit aussi gérer** :
  découverte et validation des fichiers `.gguf` locaux, chargement/
  déchargement en mémoire (mmap), paramètres d'inférence bas niveau (taille
  du contexte, threads CPU, offload GPU partiel/total), tokenizer embarqué,
  et — point sensible — le **tool/function calling**, qu'Ollama gère
  aujourd'hui côté serveur (avec, déjà, une hétérogénéité notable côté
  Crustly : `ollama.rs` lignes 607-655 contient une heuristique de secours
  — `maybe_tool_call_json`/`tool_call_from_content` — qui détecte un tool
  call imprimé en JSON brut dans le flux de texte quand le template du
  modèle ne peuple pas le champ natif `tool_calls`, et `qwen.rs` a son
  propre `ToolCallParser` pour les formats "hermes"/"openai"). Un moteur
  GGUF brut, exécutant des modèles open-weight arbitraires, aura le même
  besoin de parsing de secours — indépendant du transport (daemon Ollama vs.
  moteur en process), donc ce n'est pas un coût nouveau propre à
  l'intégration native, mais confirme que le "vrai" travail de
  tool-calling reste à faire même après avoir résolu le chargement du
  `.gguf`.
- Gestion mémoire/concurrence : un modèle chargé dans le process Crustly
  occupe la RAM pendant toute la durée de vie de l'app (ou nécessite une
  logique de déchargement maison) ; à coordonner avec le reste de
  l'application (TUI, DB, tools) qui tourne dans le même process — risque de
  pression mémoire ou de blocage du thread si l'inférence CPU n'est pas
  correctement isolée dans une tâche `tokio::task::spawn_blocking`.
- **Précédent de feature flag à suivre** : `ollama = ["dep:ollama-rs",
  "dep:schemars"]` dans `Cargo.toml`, gaté de façon cohérente dans
  `src/llm/provider/mod.rs` (`#[cfg(feature = "ollama")]`) et `factory.rs`
  (y compris un stub no-op quand la feature est désactivée). Une nouvelle
  feature `gguf`/`local-inference` suivrait exactement ce patron et
  s'ajouterait à `all-llm`.
- **Crabrace n'a pas de rôle ici** : le registre `crabrace.rs` modélise des
  providers *joignables sur le réseau* (API cloud ou daemon local avec
  surface HTTP) ; un moteur GGUF en process n'a pas de tel point d'entrée à
  enregistrer — il contournerait Crabrace entièrement, comme le fait déjà
  `QwenProvider` pour ses aspects spécifiques.

### 6.3 Maintenance long terme

- Nouvelle surface de sécurité : parsing de fichiers binaires tiers
  (`.gguf`) fournis par l'utilisateur, exécution de code natif C++ dans le
  process — à contraster avec le modèle actuel où Ollama tourne en process
  séparé (isolation naturelle des crashs/fuites mémoire du moteur
  d'inférence).
- Suivi des mises à jour de sécurité/performance de llama.cpp (dépendance
  externe C++ à vendorer/mettre à jour manuellement, hors de l'écosystème
  `cargo update`).
- Tests CI : nécessite des runners avec un vrai modèle `.gguf` téléchargé
  (même problème que documenté pour Ollama en §7 du plan d'intégration
  `ollama-rs` — "test manuel local, non exécutable en CI" — mais en pire, car
  il n'y a même plus de serveur à mocker/stubber facilement : c'est le moteur
  d'inférence lui-même qu'il faudrait invoquer).

### 6.4 Estimation d'effort (ordre de grandeur)

À titre de comparaison, l'intégration `ollama-rs` (client HTTP typé,
documentée dans `ollama-rs-integration-plan.md`) a représenté 4 phases livrées
progressivement. Un provider GGUF natif via `llama-cpp-2` couvre un
périmètre fonctionnel comparable **plus** un moteur d'inférence embarqué :
raisonnablement, **plusieurs semaines** de travail pour un MVP correct
(chargement modèle, complétion, streaming, tool-calling basique), contre
quelques jours pour un client HTTP vers un serveur déjà bâti et testé par une
communauté large (Ollama). Le rapport effort/bénéfice n'est positif que si
l'objectif "zéro dépendance de service externe" est une exigence dure (ex.
distribution air-gapped, contrainte produit explicite) — pas une simple
préférence de simplicité perçue.

---

## 7. Tableau de synthèse

| Option | Charge un `.gguf` sans conversion | Licence compatible | Effort | Recommandation |
|---|---|---|---|---|
| **Statu quo** (Ollama natif + compat OpenAI) | Non (délégué à Ollama) | ✅ | — | ✅ Conserver |
| **Cactus** (`cactus-compute/cactus`) | ❌ conversion propriétaire requise | ❌ seuils financiers, résiliation auto | Moyen-élevé (FFI + toolchain native par plateforme) | ❌ Ne pas intégrer |
| **`llama-cpp-2`** (bindings vers llama.cpp) | ✅ | ✅ MIT | Élevé (moteur natif embarqué) | ⚠️ Envisageable en ADR séparée + spike, seulement si "zéro dépendance externe" devient une exigence produit explicite |

---

## 8. Questions ouvertes avant toute décision

1. Le besoin "monolithique, pas de service externe" est-il une **contrainte
   produit ferme** (ex. usage air-gapped, distribution à des utilisateurs
   sans droits d'installation de service tiers) ou une **préférence de
   confort** ? La réponse change complètement le calcul coût/bénéfice du §6.
2. Si retenu, quel périmètre de backends est requis au lancement : CPU
   seulement (plus simple, plus lent) ou CPU+GPU (CUDA/Metal, complexité de
   build multipliée) ?
3. Le tool/function calling (utilisé massivement par les 21+ tools de
   Crustly) est-il requis dès le MVP GGUF natif, ou une première version
   "chat simple, pas de tools" est-elle acceptable pour valider l'intérêt
   avant d'investir dans le parsing de tool calls sur un moteur brut ?
4. Faut-il reconsidérer **Cactus spécifiquement** si l'équipe Cactus confirme
   un chargement GGUF natif (sans conversion) dans une version future, et
   clarifie une licence permissive pour usage embarqué en outil dev
   redistribué publiquement ? (à re-vérifier périodiquement, pas d'action
   immédiate).

---

## 9. Suite à donner

Ce document est un **plan d'évaluation**, pas une décision. Si une direction
est retenue à l'issue des questions du §8 :

- **Aucune décision** (statu quo) → rien à faire, ce document reste comme
  trace de l'évaluation.
- **Décision d'intégrer un moteur GGUF natif** (`llama-cpp-2` ou équivalent)
  → documenter le choix dans une ADR dédiée
  `docs/architecture/decisions/0005-<titre>.md` (Context/Decision/
  Consequences, cf. gabarit `0000-adr-template.md`), qui renverra vers ce
  fichier pour le détail — même articulation que l'ADR `0003` vers
  `docs/guides/CRABRACE_INTEGRATION.md`. Le détail d'implémentation
  suivrait ensuite le même triptyque que l'intégration Ollama :
  plan d'intégration (ce document en tient lieu) → plan de test → guide
  utilisateur (`docs/guides/`), sur le modèle de
  `ollama-rs-integration-plan.md` + `ollama-local-llm-test-plan.md` +
  `docs/guides/OLLAMA_GUIDE.md`.

---

## Sources consultées

- [cactus-compute/cactus](https://github.com/cactus-compute/cactus) — README, structure du repo, bindings Rust
- [cactuscompute.com/docs](https://docs.cactuscompute.com/v2.0.1/) — documentation produit
- Licence Cactus Compute, Inc. (fichier `LICENSE` du dépôt)
- `ollama-rs-integration-plan.md` (ce dépôt) — référence de comparaison d'effort
- `docs/architecture/decisions/0003-crabrace-provider-registry.md` — architecture de découverte de providers existante
- `src/llm/provider/trait.rs`, `Cargo.toml` (ce dépôt) — interface `Provider` et conventions de feature flags actuelles
- Recherche web sur `llama-cpp-2`/`llama_cpp` (crates.io) comme alternative
