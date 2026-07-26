# Spécification d'évaluation : chargement direct de modèles GGUF via `llama-cpp-2`

Statut : **Évaluation approfondie — aucune implémentation engagée**
Date : 2026-07-21
Portée : étude complète des **bénéfices** d'intégrer la crate
[`llama-cpp-2`](https://github.com/utilityai/llama-cpp-rs) (bindings Rust vers
`llama.cpp`) pour permettre à Crustly de charger et exécuter un modèle depuis
un fichier `.gguf` local, **sans dépendre d'un serveur Ollama externe**, afin
d'obtenir une application plus monolithique — avec un cadre de décision
explicite Go/No-Go en fin de document.

---

## 0. Résumé exécutif

`llama-cpp-2` est **techniquement le bon candidat** pour cet objectif :
binding Rust direct vers l'implémentation de référence du format GGUF,
publié sur crates.io, licence MIT OR Apache-2.0 (compatible avec la licence
FSL-1.1-MIT de Crustly), maintenu activement (841 745 téléchargements
cumulés, mise à jour début juillet 2026, 615 ★).

Sept bénéfices concrets et spécifiques à Crustly ressortent de cette étude
(détail en §4) :

1. **Alignement direct avec le positionnement produit** de Crustly
   ("performance, memory efficiency, and reduced resource consumption" —
   `CLAUDE.md`) : le daemon Ollama consomme ~1 Go de RAM au repos même sans
   modèle chargé ([issue confirmée par les mainteneurs
   d'Ollama](https://github.com/ollama/ollama/issues/7168)), plus l'empreinte
   complète du modèle tant que `keep_alive` ne l'a pas déchargé — un second
   processus permanent qui va à l'encontre de l'argument de vente principal
   de Crustly face à des outils plus lourds.
2. **Élimination d'une dépendance de service externe**, seul chemin viable
   dans les environnements air-gapped, les postes contraints par une
   politique IT interdisant les daemons tiers, et les pipelines CI/CD
   éphémères.
3. **Fiabilité du tool-calling supérieure** via génération contrainte par
   grammaire (GBNF/`llguidance`) — élimine structurellement les erreurs de
   syntaxe JSON que l'heuristique actuelle de récupération de texte
   (`ollama.rs` lignes 607-655) ne fait que *rattraper après coup*.
4. **Support multimodal natif** (`mtmd`), sans régression face à
   Ollama/OpenAI-compat.
5. **Onboarding utilisateur simplifié** : un seul binaire, une seule
   commande, pas de "d'abord installer et démarrer Ollama".
6. **Différenciation compétitive** face aux autres assistants IA terminal,
   qui délèguent tous l'inférence locale à un daemon externe.
7. **Contrôle total du cycle de vie du modèle**, sans dépendre du
   comportement/des choix de configuration d'un processus tiers.

Ces bénéfices sont réels mais **conditionnels** : ils ne se matérialisent
que si les coûts identifiés en §5 (toolchain de build C++/CMake, ré-
implémentation du tool-calling et de la gestion de modèles, isolation
process perdue, plusieurs semaines d'effort) sont acceptés. Le §6 fournit un
cadre de décision explicite pour trancher Go/No-Go selon les priorités
produit réelles de Crustly.

**Recommandation** : **GO conditionnel** — voir la grille de décision §6.
L'intégration est justifiée si au moins un des critères "durs" (contrainte
air-gapped/IT, ou fiabilité du tool-calling jugée insuffisante aujourd'hui)
s'applique ; sinon, le rapport effort/bénéfice penche pour un **NO-GO à ce
stade**, avec réévaluation possible si l'un de ces critères apparaît plus
tard.

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
typé) : aucun code d'inférence, uniquement du mapping requête/réponse — mais
c'est aussi précisément ce qui *coûte* en ressources et en dépendance
externe, matière du §4.

L'idée évaluée ici change de nature : **faire tourner l'inférence dans le
process Crustly lui-même**, en lisant un fichier `.gguf` directement depuis
le disque, sans dépendre d'aucun serveur externe démarré au préalable.

---

## 2. `llama-cpp-2` : ce que c'est

### 2.1 Composition et provenance

Le dépôt [`utilityai/llama-cpp-rs`](https://github.com/utilityai/llama-cpp-rs)
publie deux crates sur crates.io : **`llama-cpp-2`** (API Rust de haut
niveau) et **`llama-cpp-sys-2`** (bindings FFI bruts, `bindgen`, compile le
sous-module vendoré `llama.cpp` via la crate `cmake` au moment du `cargo
build`). Design assumé : rester proche des bindings bruts et "aussi à jour
que possible avec `llama.cpp`" — **explicitement pas suivi en SemVer
strict** (versionnage actuel : `llama-cpp-2` v0.1.151).

### 2.2 Licence

`MIT OR Apache-2.0` — pleinement compatible avec la licence FSL-1.1-MIT de
Crustly (`Cargo.toml` ligne 7). Aucun blocage juridique.

### 2.3 Maturité

841 745 téléchargements cumulés sur crates.io (430 790 récents), 615 ★ / 218
forks, 151 versions publiées, dernière mise à jour début juillet 2026, 33
issues ouvertes / 12 PR en cours — niveau d'activité sain, ~2,5 ans
d'historique.

### 2.4 Fonctionnalités couvertes (features Cargo)

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

Exemples fournis dans le dépôt : `simple` (chargement `.gguf` quantifié
`Q4_K_M`/`Q6_K` depuis HuggingFace + complétion), `embeddings`, `mtmd`,
`reranker`, `llguidance.rs` (génération contrainte), `usage.rs` — confirment
un chargement natif sans étape de conversion.

---

## 3. Alignement avec l'architecture Crustly actuelle

`src/llm/provider/trait.rs` (lignes 18-65) définit une interface
agnostique du transport : `complete()`, `stream()`,
`supports_streaming/tools/vision()`, `name()`, `default_model()`,
`context_window()`, `calculate_cost()`. Un provider GGUF natif
implémenterait ce même trait, câblé dans `factory.rs` comme
`try_create_ollama()` l'est déjà — **aucune modification du trait requise**.

`ProviderConfig` générique (`src/config/mod.rs` ligne 346) ne connaît qu'un
`default_model: String` (nom résolu à distance), pas un chemin de fichier —
un nouveau `GgufProviderConfig` (`model_path`, `n_ctx`, `n_threads`,
`n_gpu_layers`) serait nécessaire, sur le modèle d'`OllamaModelConfig`
(ligne 499) ou `QwenProviderConfig` (ligne 366). Le registre `crabrace.rs`
n'a aucun rôle ici (il modélise des providers joignables sur le réseau) — un
moteur GGUF en process le contournerait entièrement.

---

## 4. Étude complète des bénéfices

### 4.1 Alignement avec le positionnement produit de Crustly

C'est le bénéfice le plus spécifique à ce projet précis, et le plus souvent
absent d'une évaluation générique de `llama-cpp-2`. `CLAUDE.md` définit
Crustly comme : *"a high-performance terminal AI assistant... with focus on
performance, memory efficiency, and reduced resource consumption"*. Or,
l'usage local actuel repose entièrement sur un **second processus
permanent** (le daemon Ollama) :

- Un [rapport de bug confirmé côté mainteneurs d'Ollama](https://github.com/ollama/ollama/issues/7168)
  documente une empreinte mémoire du processus Ollama d'**environ 1 Go au
  repos**, y compris sans aucun modèle chargé, du fait des "embedded
  runners". À cela s'ajoute l'empreinte complète du modèle en mémoire tant
  que `keep_alive` ne l'a pas déchargé (plusieurs Go de plus selon la
  taille/quantization du modèle).
- Ce coût est **permanent et indépendant de l'usage réel de Crustly** : le
  daemon tourne dès qu'il est démarré (souvent au login, par défaut sur les
  installateurs officiels), que Crustly soit lancé ou non.
- Un moteur GGUF embarqué n'a **aucune empreinte mémoire en dehors des
  sessions Crustly actives** : le modèle n'est chargé que pendant la durée
  de vie du process qui en a besoin, et disparaît avec lui — cohérent avec
  l'argument produit "reduced resource consumption" que Crustly met en avant
  face à des concurrents plus lourds.
- Bénéfice secondaire : un seul processus à surveiller/tracer/profiler
  (`cargo build --features profiling` couvre déjà Crustly, mais pas le
  comportement du daemon Ollama externe, hors de portée des outils de
  profiling du projet).

**Niveau de confiance : élevé** (chiffre sourcé sur un rapport public
maintenu par l'éditeur d'Ollama lui-même, pas une estimation).

### 4.2 Élimination de la dépendance à un service externe

- **Environnements air-gapped / réseaux isolés** : un utilisateur avec un
  fichier `.gguf` déjà sur disque peut lancer Crustly sans jamais avoir eu
  besoin d'installer, configurer, ni faire tourner de service réseau, même
  local. C'est un cas d'usage qu'Ollama ne couvre pas nativement (il faut
  quand même installer et démarrer le daemon au préalable, même sans accès
  Internet pour le `pull`).
- **Postes contraints par une politique IT** : dans de nombreux
  environnements d'entreprise, l'installation ou l'exécution de services
  d'arrière-plan (daemons, ports d'écoute même en local) est soumise à
  autorisation ou bloquée par la politique de sécurité du poste. Un binaire
  unique sans port d'écoute local évite cette classe de blocage.
- **CI/CD et conteneurs éphémères** : dans un pipeline ou une image Docker
  jetable, démarrer et attendre qu'un second processus (Ollama) soit prêt
  avant de lancer Crustly ajoute de la complexité d'orchestration
  (health-check, ordre de démarrage, gestion du cycle de vie du conteneur
  compagnon). Un binaire unique élimine cette coordination.
- **Pas de "port déjà utilisé" / conflit réseau local** : un moteur en
  process n'ouvre aucun port TCP local, contrairement au daemon Ollama
  (`:11434`) — supprime une classe entière de bugs de support
  ("Ollama unreachable at http://localhost:11434", déjà un message d'erreur
  documenté dans le plan d'intégration `ollama-rs`).

**Niveau de confiance : élevé** pour les cas air-gapped/IT-restreint (besoin
binaire, pas de solution de contournement) ; **moyen** pour le confort
général (Ollama s'installe et démarre déjà automatiquement sur la plupart
des postes de développement standards).

### 4.3 Fiabilité du tool-calling via génération contrainte par grammaire

Crustly s'appuie massivement sur le tool-calling (21+ tools). Aujourd'hui,
avec les modèles locaux via Ollama, la fiabilité de ce mécanisme dépend
entièrement du template de chat du modèle et de sa capacité à respecter le
format `tool_calls` attendu — au point que Crustly a dû ajouter une
**heuristique de secours** : `maybe_tool_call_json`/`tool_call_from_content`
(`ollama.rs` lignes 607-655) qui détecte un tool call imprimé en JSON brut
dans le flux de texte quand le modèle ne peuple pas le champ natif, et
`qwen.rs` a son propre `ToolCallParser` pour deux formats différents
("hermes"/"openai"). L'existence même de ces deux mécanismes de
récupération témoigne d'un problème réel et déjà rencontré en pratique, pas
hypothétique.

`llama-cpp-2` expose la génération contrainte par grammaire de `llama.cpp`
(GBNF, feature `llguidance`) : le format de sortie du modèle peut être
**restreint au niveau du vocabulaire à chaque étape de décodage**, de sorte
que le modèle ne peut physiquement pas produire un token qui violerait le
schéma JSON attendu. Documentation `llama.cpp` : cela *"elimine
effectivement les erreurs de syntaxe dans les sorties structurées"* — un
changement de nature par rapport à une heuristique de récupération après
coup, qui ne peut que *deviner* qu'un texte ressemble à un tool call raté.
Limite honnête à noter : la contrainte de grammaire garantit la
**syntaxe**, pas l'épuisement du budget de tokens avant la fin d'une
structure JSON valide (cas limite documenté par la communauté `llama.cpp`)
— un filet de sécurité reste donc utile même avec la grammaire activée,
mais le taux d'échec structurel de base change de catégorie (de "arrive
régulièrement selon le modèle" à "cas limite rare de troncature").

**Niveau de confiance : moyen-élevé** — le mécanisme est documenté et
largement utilisé dans l'écosystème `llama.cpp` (`llama-cpp-python`,
`node-llama-cpp` l'exposent aussi), mais son intégration dans Crustly
resterait à construire (mapping des `Tool.input_schema` déjà génériques de
`provider/types.rs` vers une grammaire GBNF) — un coût, détaillé en §5, mais
qui débloque un bénéfice qu'aucune des solutions HTTP actuelles
(Ollama/OpenAI-compat) ne peut offrir, celles-ci étant limitées à ce que le
serveur distant expose.

### 4.4 Support multimodal natif (`mtmd`)

La feature `mtmd` de `llama-cpp-2` couvre l'inférence multimodale
(vision), ce qui correspondrait à `Provider::supports_vision()`
(`trait.rs` ligne 42). Bénéfice de parité stricte avec Ollama/OpenAI-compat,
qui gèrent déjà la vision aujourd'hui — un moteur GGUF natif sans cette
feature serait une régression ; avec elle, aucune perte de capacité pour
l'utilisateur qui migrerait vers ce chemin.

### 4.5 Contrôle total du cycle de vie du modèle

Avec un moteur embarqué, Crustly décide directement :

- **du moment exact du chargement/déchargement** du modèle, sans dépendre
  du réglage `keep_alive` d'un daemon externe (actuellement configurable
  côté Crustly via `providers.ollama.keep_alive`, mais toujours appliqué
  *par Ollama*, avec ses propres heuristiques de gestion mémoire globale
  entre plusieurs modèles/clients) ;
- **des paramètres d'inférence bas niveau** (nombre de threads CPU, nombre
  de couches déportées sur GPU, taille de contexte) sans passer par les
  valeurs par défaut ou les Modelfiles d'Ollama ;
- **de la coordination avec le reste de l'application** : par exemple,
  libérer explicitement la mémoire du modèle pendant une opération
  Crustly gourmande en RAM (compaction de contexte, traitement d'un gros
  fichier), une coordination impossible avec un processus externe qui ne
  connaît pas l'état interne de Crustly.

Ce niveau de contrôle n'est valorisable que si le produit a un besoin réel de
cette finesse — sinon c'est un bénéfice théorique. À rapprocher de la
question ouverte §7.2.

### 4.6 Onboarding utilisateur simplifié

Aujourd'hui, l'utilisateur qui veut du local doit : installer Ollama
(`curl -fsSL https://ollama.com/install.sh | sh`), vérifier qu'il tourne,
puis `ollama pull <modèle>`, puis configurer Crustly pour pointer dessus
(`docs/guides/OLLAMA_GUIDE.md`) — un parcours en plusieurs étapes sur deux
outils distincts. Avec un moteur GGUF natif, le parcours se réduirait à :
télécharger un fichier `.gguf` (HuggingFace ou autre source) et pointer
Crustly dessus (`crustly run --model /chemin/vers/modele.gguf` ou équivalent
config) — un seul outil, un chemin de configuration. Bénéfice direct pour
l'acquisition de nouveaux utilisateurs qui essaient Crustly pour la première
fois sans vouloir gérer un service tiers.

**Niveau de confiance : moyen** — dépend de la disponibilité de fichiers
`.gguf` prêts à l'emploi (avec Ollama, `ollama pull` gère la découverte et
le téléchargement ; avec un `.gguf` brut, l'utilisateur doit trouver et
télécharger le bon fichier lui-même, sauf si Crustly ré-implémente cette
brique — coût déjà noté en §5).

### 4.7 Différenciation compétitive

Les assistants IA terminal comparables (Aider, et d'autres outils du même
segment) délèguent tous l'inférence locale à un backend externe
(Ollama, LM Studio, vLLM). Un chargement GGUF natif ferait de Crustly l'un
des rares outils du segment à offrir une **véritable exécution
monolithique** de bout en bout, y compris en local — un argument de
positionnement produit distinct, cohérent avec l'identité "performance et
efficacité mémoire" déjà revendiquée par le projet.

**Niveau de confiance : qualitatif** — bénéfice stratégique, non mesurable
directement, à valider par la stratégie produit plutôt que par une métrique
technique.

### 4.8 Tableau récapitulatif des bénéfices

| # | Bénéfice | Nature | Niveau de confiance | Dépend de |
|---|---|---|---|---|
| 1 | Réduction de l'empreinte ressources (pas de daemon permanent ~1 Go+) | Quantifiable, sourcé | Élevé | — |
| 2 | Fonctionne air-gapped / IT-restreint / CI éphémère | Fonctionnel, binaire (marche ou pas) | Élevé (cas dur) / Moyen (confort général) | Besoin produit réel (§6) |
| 3 | Tool-calling plus fiable (grammaire GBNF) | Qualitatif, mécanisme documenté | Moyen-élevé | Effort d'intégration (§5.2) |
| 4 | Parité multimodale (`mtmd`) | Fonctionnel | Élevé | Activation de la feature |
| 5 | Contrôle fin du cycle de vie du modèle | Qualitatif | Moyen | Besoin produit réel |
| 6 | Onboarding simplifié (un seul outil) | UX | Moyen | Solution de téléchargement de `.gguf` (à construire) |
| 7 | Différenciation compétitive | Stratégique | Qualitatif | Positionnement produit voulu |

---

## 5. Coûts (résumé — détail dans les sections précédentes de ce document avant réécriture, condensé ici pour équilibrer l'étude)

### 5.1 Build et packaging

CMake + compilateur C++ requis pour compiler le sous-module vendoré
`llama.cpp` à chaque `cargo build` avec la feature activée — rompt la
promesse actuelle "juste `cargo build`" de `CLAUDE.md`. Windows nécessite
Visual Studio Build Tools (C++) ou MinGW ; CUDA/Vulkan/ROCm/MKL demandent une
configuration manuelle (toolkit installé, `VULKAN_SDK`, etc.). Temps de
build à froid et taille du binaire final en hausse significative.

### 5.2 Nouveau code applicatif

Nouveau provider (`src/llm/provider/gguf.rs`), nouveau `GgufProviderConfig`
(§3), ré-implémentation du tool-calling (heuristique texte reprise de
`ollama.rs`/`qwen.rs`, ou investissement dans `llguidance` pour le bénéfice
§4.3), gestion mémoire/concurrence (`tokio::task::spawn_blocking` pour ne
pas geler la boucle TUI), et ré-implémentation de la découverte/du
téléchargement de modèles (aujourd'hui gérés par `ollama pull`).

### 5.3 Maintenance long terme

Pas de SemVer strict côté `llama-cpp-2` (suit `llama.cpp` en amont, pin de
version à surveiller plus attentivement), nouvelle surface de sécurité
(exécution de code natif C++ sur un fichier `.gguf` fourni par
l'utilisateur, dans le même process que la TUI/DB/tools — perte de
l'isolation crash/mémoire qu'offre aujourd'hui le process séparé Ollama),
dépendance C++ vendorée hors `cargo update`, tests CI plus difficiles
(aucun serveur à mocker, il faudrait invoquer un vrai `.gguf`).

### 5.4 Ce qui est perdu par rapport à Ollama

Partage de modèle entre plusieurs process (un chargement par instance
Crustly au lieu d'un chargement partagé), déchargement automatique par
inactivité à réimplémenter, catalogue de modèles nommés
(`llama3.2:3b` → nom court) remplacé par la gestion manuelle de fichiers.

### 5.5 Estimation d'effort

Plusieurs semaines pour un MVP correct (build multi-plateforme, chargement,
complétion, streaming, tool-calling basique), contre quelques jours pour un
client HTTP vers un serveur déjà bâti (Ollama). Effort significativement
plus élevé si `llguidance` est intégré dès le MVP plutôt que dans une
itération ultérieure.

---

## 6. Cadre de décision Go/No-Go

Grille de critères à évaluer contre les priorités produit réelles de
Crustly — chaque critère "dur" coché suffit à justifier un GO ; en leur
absence, le rapport effort/bénéfice ne le justifie pas encore.

| Critère | Type | Statut aujourd'hui | Si "oui" → |
|---|---|---|---|
| Un usage air-gapped / sans accès réseau local est un cas d'usage cible explicite | Dur | À confirmer par le produit | **GO** — aucune alternative HTTP ne couvre ce cas |
| Des utilisateurs cibles sont sur des postes où l'installation/exécution d'un daemon est bloquée par la politique IT | Dur | À confirmer par le produit | **GO** — même raison |
| Le taux d'échec actuel du tool-calling avec les modèles locaux (heuristique texte) est mesuré comme un point de friction utilisateur significatif | Dur si mesuré | Non mesuré à ce jour (aucune télémétrie de taux d'échec identifiée dans le code) | **GO**, avec priorité sur `llguidance` dès le MVP |
| La réduction de l'empreinte mémoire "au repos" est un objectif produit chiffré (ex. cible RAM totale documentée) | Souple | Aligné avec le positionnement `CLAUDE.md`, mais pas de cible chiffrée trouvée dans le dépôt | Renforce un GO, ne le déclenche pas seul |
| L'équipe a la capacité de maintenir une dépendance C++ vendorée hors SemVer strict sur le long terme | Prérequis | À évaluer (taille équipe, expertise C++/CMake disponible) | Condition **bloquante** si "non", indépendamment des bénéfices |
| Un budget de plusieurs semaines dédiées est disponible à court terme sans repousser une autre priorité critique | Prérequis | À arbitrer par la roadmap | Condition bloquante si "non" |

**Lecture recommandée de la grille** :
1. Vérifier d'abord les deux **prérequis** (capacité de maintenance C++,
   budget de temps) — s'ils ne sont pas remplis, la question est **prématurée**
   quel que soit le score des bénéfices : mieux vaut la rouvrir plus tard.
2. Si les prérequis sont remplis, vérifier les critères **durs** : un seul
   suffit à justifier un GO, car ce sont des besoins qu'aucune alternative
   HTTP actuelle (Ollama, OpenAI-compat) ne peut satisfaire.
3. Si aucun critère dur n'est confirmé, les bénéfices restent réels (§4) mais
   **discrétionnaires** : un NO-GO à ce stade est raisonnable, avec
   réévaluation dès qu'un critère dur apparaît (ex. une demande utilisateur
   documentée pour un usage air-gapped).

**Conclusion de cette étude** : sur la base des seules informations
disponibles dans ce dépôt à la date de rédaction (aucune mention d'objectif
air-gapped, aucune contrainte IT documentée, aucune télémétrie de taux
d'échec de tool-calling), **aucun critère dur n'est aujourd'hui confirmé** —
le score penche pour un **NO-GO conditionnel**, à réévaluer dès que l'un des
trois critères durs du tableau reçoit une réponse positive du produit. Le
bénéfice §4.1 (alignement ressources) reste valable comme argument
d'appoint, mais insuffisant seul pour justifier plusieurs semaines
d'ingénierie et une nouvelle dépendance C++ à maintenir hors SemVer.

---

## 7. Questions ouvertes pour trancher

1. Un usage air-gapped ou fortement restreint par une politique IT est-il un
   cas d'usage cible **documenté**, ou une hypothèse de cette étude ?
   (Critère dur #1/#2 du §6 — à confirmer avant toute décision.)
2. Existe-t-il une mesure, même informelle, du taux d'échec du tool-calling
   avec les modèles locaux aujourd'hui (retours utilisateurs, issues
   GitHub) ? Cela transformerait le bénéfice §4.3 en critère dur.
3. Quel périmètre de backends est requis au lancement : CPU seulement (plus
   simple) ou CPU+GPU (CUDA/Metal, complexité de build multipliée) ?
4. Faut-il viser `llguidance` dès le MVP, ou commencer par l'heuristique
   texte déjà éprouvée et migrer plus tard ?
5. L'équipe dispose-t-elle de l'expertise CMake/C++ nécessaire pour
   maintenir cette dépendance sur la durée (prérequis bloquant du §6) ?
6. Qui gère la vérification d'intégrité (checksum) et la provenance des
   fichiers `.gguf` fournis par l'utilisateur, exécutés par du code natif
   dans le même process que le reste de l'application ?

---

## 8. Suite à donner

- **NO-GO / réévaluation ultérieure** (conclusion actuelle, §6) → conserver
  ce document comme référence, le ressortir dès qu'un critère dur du §6
  reçoit une réponse positive.
- **GO** (si un critère dur est confirmé) → commencer par un **spike
  technique isolé** (hors branche principale) : charger un petit `.gguf`
  (1-3B), obtenir complétion + streaming fonctionnels, mesurer temps de
  build à froid et taille du binaire réels sur les trois plateformes
  cibles. Documenter ensuite le choix dans une ADR dédiée
  `docs/architecture/decisions/0005-<titre>.md` (gabarit
  `0000-adr-template.md`) renvoyant vers ce fichier pour le détail — même
  articulation que l'ADR `0003` vers `docs/guides/CRABRACE_INTEGRATION.md`.
  Implémentation ensuite selon le même triptyque que l'intégration Ollama :
  plan d'intégration détaillé → plan de test → guide utilisateur
  (`docs/guides/`).

---

## Sources consultées

- [utilityai/llama-cpp-rs](https://github.com/utilityai/llama-cpp-rs) — README, structure du dépôt, exemples
- [llama-cpp-2 sur crates.io](https://crates.io/crates/llama-cpp-2) — version, licence, statistiques de téléchargement
- `llama-cpp-2/Cargo.toml` (dépôt `utilityai/llama-cpp-rs`) — features Cargo
- [ollama/ollama issue #7168](https://github.com/ollama/ollama/issues/7168) — empreinte mémoire du daemon Ollama au repos (~1 Go, "embedded runners")
- Documentation `llama.cpp` sur les grammaires GBNF et la génération contrainte (structured output / function calling)
- `ollama-rs-integration-plan.md` (ce dépôt) — référence de comparaison d'effort et patrons d'implémentation (streaming, tool-calling heuristique, feature flags)
- `docs/architecture/decisions/0003-crabrace-provider-registry.md` — architecture de découverte de providers existante
- `src/llm/provider/trait.rs`, `src/llm/provider/ollama.rs`, `src/config/mod.rs`, `Cargo.toml` (ce dépôt) — interface `Provider`, conventions de feature flags et écarts de schéma de configuration actuels
