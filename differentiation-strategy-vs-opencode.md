# Stratégie de différenciation : Crustly face à OpenCode

Statut : **Étude stratégique — propositions à valider avant tout engagement**
Date : 2026-07-21
Portée : identifier des axes de différenciation crédibles et défendables pour
Crustly face à OpenCode (`opencode.ai`), l'agent de code IA open source
dominant du marché en 2026, et proposer une feuille de route priorisée.

---

## 0. Résumé exécutif

OpenCode a une échelle qu'aucune stratégie de rattrapage fonctionnel ne peut
combler : ~160 000 ★ GitHub, ~7,5 millions de développeurs actifs mensuels,
75+ providers, écosystème de plugins TypeScript, architecture client/serveur
multi-frontend (TUI, desktop, web, mobile), licence MIT. **Chercher à
égaler cette largeur fonctionnelle serait une stratégie perdante** pour une
équipe de la taille de celle de Crustly.

Cette étude identifie à la place **quatre axes de profondeur** où Crustly a
déjà un avantage structurel réel, vérifiable dans son propre code, et que
l'architecture d'OpenCode ne peut pas reproduire sans une refonte majeure :

1. **Efficacité ressources mesurable** — Rust natif, process unique, contre
   une stack TypeScript/Bun avec un serveur HTTP/SSE (Hono) qui tourne même
   pour un usage TUI purement local.
2. **Architecture véritablement monolithique, zéro démon** — pas de scission
   client/serveur interne, ce qui ouvre des cas d'usage (air-gapped, postes
   IT-restreints) qu'OpenCode ne sert pas nativement.
3. **Modèle de sécurité plus profond** — moteur de policies composables
   (`AndPolicy`, allowlist bash résistante au chaînage d'opérateurs shell,
   frontières de chemin) contre l'aveu explicite d'OpenCode : *"no sandbox,
   no rule engine, no hooks — just a channel and a human."*
4. **Plan Mode auditable et persisté** — approbation explicite avant
   exécution, tracée en base SQLite (`plan_tasks`, `compaction_records`),
   contre l'approbation "une fois par session, on ne redemande plus"
   d'OpenCode, plus permissive mais moins traçable.

**Recommandation** : positionner Crustly non pas comme "un autre agent de
code terminal", mais comme **l'agent de code pour les environnements
contraints en ressources, en réseau, ou en gouvernance** — développeurs sur
machines modestes, environnements air-gapped/réglementés, équipes qui ont
besoin d'un journal d'audit de ce que l'IA a fait. C'est un marché de niche
mais défendable, aligné avec ce que Crustly est déjà, plutôt qu'une course
à la parité fonctionnelle avec un projet 1000x plus large en communauté.

---

## 1. Profil concurrentiel : OpenCode

### 1.1 Échelle et modèle

- **~160 000 ★** sur GitHub début 2026, **~7,5 millions de développeurs**
  actifs mensuels revendiqués — l'agent de code terminal open source le
  plus populaire par une large marge.
- **Licence MIT**, gratuit, model-agnostic (l'utilisateur paie uniquement
  les tokens du provider choisi).
- **75+ providers** supportés (Anthropic, OpenAI, Gemini, Bedrock, Ollama,
  et bien d'autres).
- Positionnement confidentialité : pas de stockage du code/contexte côté
  infrastructure OpenCode, les prompts partent directement vers le
  provider configuré par l'utilisateur.

### 1.2 Architecture

Client/serveur : un serveur HTTP/WebSocket basé sur **Hono** orchestre
trois modules (Agent Engine, Session Manager, Tool Registry), consommé par
plusieurs frontends qui partagent le même backend :

- **TUI** — construit sur OpenTUI + SolidJS.
- **Desktop app** — Electron.
- **Web / mobile** — clients distants sur le même serveur.
- **SDK** — clients JavaScript et Python.

Ce choix permet le **partage de session par lien** entre coéquipiers — une
fonctionnalité forte, mais qui implique qu'un serveur HTTP tourne même pour
un usage 100% local et solo dans un terminal.

### 1.3 Fonctionnalités notables

- **LSP Client** intégré (diagnostics, hover, go-to-definition) via le
  Language Server Protocol.
- **Système de plugins** TypeScript/JavaScript avec 25+ hooks de cycle de
  vie.
- **Sub-agents** : agents définis en code ou chargés depuis Markdown,
  fusionnés dans un registre partagé, exécutés via un pipeline unifié de
  prompt/permission/session.
- **MCP Server** : support du Model Context Protocol pour connecter des
  serveurs d'outils externes.
- **AGENTS.md** : fichier d'instructions portable, avec repli sur
  `CLAUDE.md` si présent — facilite la migration depuis Claude Code.

### 1.4 Modèle de permissions et de sécurité — le point le plus important pour cette étude

Confirmé par une analyse d'architecture indépendante (comparaison
Claude Code / Codex / Cline / OpenCode) :

> *"OpenCode uses Go channels to create a clean blocking-approval
> pattern... OpenCode has no sandbox, no rule engine, no hooks — just a
> channel and a human."*

Le mécanisme est volontairement minimal : l'agent demande une permission,
bloque jusqu'à réponse de l'utilisateur, et **mémorise l'approbation au
niveau de la session** — une fois autorisée, une action similaire ne
redemande plus. OpenCode parse les commandes shell avec `tree-sitter` (le
même moteur que Neovim) pour repérer les commandes à risque (`rm`, `mv`,
`chmod`, chemins hors projet) plus finement qu'un simple filtrage de
chaînes, mais **il n'y a ni moteur de règles composables, ni hooks
pré/post-exécution, ni sandbox OS**. C'est un choix de philosophie assumé
— *"qui détient les clés"*, et OpenCode les donne entièrement à
l'utilisateur, en misant sur l'inspectabilité plutôt que sur des couches de
sécurité intégrées.

C'est un écart concret, vérifiable, et directement opposable au modèle de
Crustly (§3.3).

---

## 2. Ce que Crustly ne doit pas chasser (anti-objectifs)

Avant de lister les axes à investir, il faut nommer explicitement ce qu'il ne
faut **pas** essayer de répliquer, car le rapport effort/impact y est
structurellement défavorable face à une communauté 1000x plus large :

| Ce qu'OpenCode a | Pourquoi ne pas courir après |
|---|---|
| 75+ providers | Crustly en couvre déjà les plus utilisés (Anthropic, OpenAI, Gemini, Bedrock, Ollama, Qwen, Azure) ; chaque provider supplémentaire est un coût de maintenance récurrent pour un bénéfice marginal decroissant |
| Multi-frontend (web/mobile/desktop) | Implique la même scission client/serveur que Crustly évite précisément dans son positionnement (§3.2) — les deux ne sont pas cumulables sans renoncer à l'avantage "process unique" |
| Écosystème de plugins tiers (25+ hooks) | Nécessite une communauté de contributeurs de plugins pour avoir de la valeur ; sans l'échelle d'OpenCode, un système de plugins Crustly resterait vide |
| Partage de session par lien | Suppose une infrastructure serveur/backend hébergé, contraire à l'architecture monolithique/offline-first visée (§3.2) |
| Nombre absolu d'étoiles/d'utilisateurs | Effet de réseau déjà consolidé ; non rattrapable par des choix techniques, seulement par un positionnement différent qui capte un segment qu'OpenCode ne sert pas bien |

Le risque principal d'ignorer cette liste : diluer l'effort d'ingénierie
sur des fonctionnalités où Crustly ne pourra jamais atteindre la qualité
ou l'adoption d'OpenCode, au détriment des axes où un avantage structurel
existe déjà.

---

## 3. Les quatre axes de différenciation

### 3.1 Efficacité ressources — mesurable, pas seulement revendiquée

**État actuel** : Crustly est écrit en Rust, compilé en binaire natif
(`cargo build --release` avec LTO fat, `codegen-units = 1`, `strip = true`,
`panic = "abort"` — `Cargo.toml` lignes 151-156), sans runtime interprété ni
machine virtuelle. OpenCode tourne sur Bun/Node avec un serveur HTTP/SSE
actif en permanence (Hono), même pour une session TUI locale solo.

**Ce qui manque aujourd'hui** : Crustly revendique déjà "performance,
memory efficiency, and reduced resource consumption" dans `CLAUDE.md`, mais
**aucun chiffre public ne le prouve** face à un concurrent nommé. C'est une
affirmation non vérifiée, donc peu convaincante pour un utilisateur qui
hésite entre les deux outils.

**Initiative proposée** : un **benchmark reproductible et publié** (détail
méthodologique en §5) comparant, sur la même machine, la même tâche :
temps de démarrage à froid, RSS mémoire au repos et pendant une session
active, taille du binaire/installation. Publier les résultats et le script
de mesure (pas juste les chiffres) pour que la comparaison soit vérifiable
par un tiers — la crédibilité vient de la reproductibilité, pas de
l'affirmation.

**Effort** : faible-moyen (quelques jours : script de mesure + exécution +
rédaction). **Impact** : élevé si les chiffres sont favorables (matérialise
une affirmation déjà faite mais jamais prouvée) ; risque de réputation si
les chiffres sont défavorables (voir §6).

### 3.2 Architecture monolithique, zéro démon

**État actuel** : Crustly n'a pas de scission client/serveur interne — la
TUI, l'orchestration LLM, la base SQLite et l'exécution des tools tournent
dans le même process. OpenCode, même en usage 100% local et solo,
maintient un serveur HTTP/WebSocket actif en arrière-plan.

**Ce que cela ouvre pour Crustly** : des cas d'usage qu'OpenCode ne sert
pas nativement — environnements air-gapped, postes contraints par une
politique IT qui bloque les daemons/ports d'écoute locaux, pipelines
CI/CD éphémères où démarrer et attendre un serveur compagnon ajoute de la
complexité d'orchestration. C'est directement lié à l'évaluation déjà faite
dans `llm-file-gguf-support.md` (chargement direct de modèles `.gguf` sans
dépendre d'un serveur Ollama externe) : les deux initiatives se renforcent
mutuellement — si l'inférence locale devient elle aussi monolithique (pas
de daemon Ollama), Crustly devient l'un des seuls agents de code du segment
à être **intégralement** zéro-démon, LLM local compris.

**Initiative proposée** : formaliser ce positionnement dans la
documentation ("Crustly ne démarre jamais de serveur, jamais de port
d'écoute local — tout tourne dans un seul process") et le relier
explicitement à l'évaluation GGUF déjà réalisée comme prochaine étape
naturelle pour fermer complètement cette boucle. Voir `llm-file-gguf-
support.md` §6 pour le cadre de décision Go/No-Go correspondant.

**Effort** : faible pour la partie documentation/positionnement (déjà
vrai aujourd'hui) ; élevé si le chargement GGUF direct est effectivement
implémenté (déjà chiffré à plusieurs semaines dans le document lié).
**Impact** : moyen à court terme (documentation), potentiellement élevé à
moyen terme si combiné au GGUF direct — un vrai argument de niche
défendable.

### 3.3 Modèle de sécurité — l'écart le plus net et le plus prouvable

**État actuel, vérifié dans le code** :

- `src/llm/tools/sandbox.rs` définit un trait `PermissionPolicy` composable
  (`evaluate(tool_name, inputs) -> PolicyDecision`), avec trois issues
  distinctes — `Allow` (permis mais toujours soumis au prompt d'approbation
  normal), `Trusted` (allowlisté explicitement, aucun prompt), `Deny`
  (bloqué, avec message) — une distinction intentionnelle documentée en
  commentaire (lignes 12-19) pour éviter qu'une politique par défaut
  "permissive" ne supprime silencieusement toute demande d'approbation.
- Une **allowlist bash** (`security.allow_bash` dans `config.toml`,
  `src/config/mod.rs` ligne 56) est testée contre le contournement par
  **chaînage d'opérateurs shell** — le test
  `bash_allowlist_never_trusts_shell_operator_chaining`
  (`sandbox.rs` ligne 705) vérifie explicitement qu'une commande
  allowlistée ne devient pas une porte dérobée si on la chaîne avec `&&`,
  `;`, `|` pour exécuter autre chose derrière.
- Trois modes d'approbation documentés dans `CLAUDE.md` (Interactive par
  défaut, AutoPlan pour les actions à faible risque, FullAuto), construits
  sur ce même moteur de policies composables (`AndPolicy` pour chaîner
  plusieurs règles).
- Enforcement des frontières de chemin (symlinks et échappements `../../`
  bloqués), vérifié avant toute opération fichier.

**Comparaison directe avec OpenCode** : la citation de §1.4 — *"no sandbox,
no rule engine, no hooks — just a channel and a human"* — décrit
précisément ce que Crustly **a** et qu'OpenCode **n'a pas** : un moteur de
règles composables, une distinction formelle entre "permis avec prompt" et
"allowlisté sans prompt", et des tests dédiés à des classes de
contournement connues (chaînage d'opérateurs). Ce n'est pas une
affirmation marketing à construire — c'est déjà implémenté et testé
aujourd'hui dans ce dépôt.

**Initiative proposée** : documenter cet écart explicitement (page
"Sécurité" comparative dans le README ou un guide dédié), en citant le
comportement réel d'OpenCode (sourcé, pas une attaque non fondée) face au
moteur de policies de Crustly. C'est l'axe où Crustly a le **moins** de
travail à faire pour matérialiser un avantage réel — il s'agit surtout de
le rendre visible.

**Effort** : très faible (documentation d'un état déjà existant).
**Impact** : élevé pour le segment "équipes/entreprises soucieuses de
sécurité", qui est probablement le public le plus réceptif à un
positionnement de niche face à un OpenCode généraliste.

### 3.4 Plan Mode auditable et persisté

**État actuel** : `src/plan/mod.rs` définit un `PlanDocument` complet
(tâches, contexte, risques identifiés, stratégie de test, stack technique,
statut) qui vit délibérément à la racine du crate plutôt que sous `tui/`
pour rester indépendant de l'UI (ADR 0004, `docs/architecture/decisions/
0004-plan-mode-read-only-with-approval-gating.md`) — un choix
architectural qui garantit que la planification passe par une porte
d'approbation explicite avant exécution, tracée en base SQLite
(`plan_tasks`, `compaction_records` pour l'historique de compaction du
contexte).

**Comparaison avec OpenCode** : les sub-agents d'OpenCode sont orientés
délégation de tâches entre agents, avec une approbation "une fois par
session" qui ne redemande plus ensuite (§1.4) — un modèle optimisé pour la
fluidité, pas pour la traçabilité. Crustly, à l'inverse, peut répondre à la
question *"qu'est-ce que l'IA a fait exactement, et qui l'a approuvé ?"*
avec un enregistrement persisté, pas seulement un log de session éphémère.

**Initiative proposée** : formaliser un **export d'audit** du Plan Mode
(ex. `crustly plan export --session <id>` produisant un rapport lisible :
tâches proposées, statut d'approbation, actions exécutées, horodatage) —
transforme un avantage architectural déjà présent en fonctionnalité
utilisateur concrète et démontrable, utile en particulier pour des équipes
sous obligation de conformité (revue de code assistée par IA avec preuve
d'approbation humaine).

**Effort** : moyen (le modèle de données existe déjà ; il s'agit
principalement d'une commande d'export et d'un format de rapport).
**Impact** : moyen-élevé pour le segment entreprise/réglementé, différenciant
car structurellement absent du modèle de session d'OpenCode.

---

## 4. Feuille de route priorisée

| # | Initiative | Axe | Effort | Impact | Dépendances |
|---|---|---|---|---|---|
| 1 | Documenter l'écart de modèle de sécurité (§3.3) | Sécurité | Très faible | Élevé | Aucune — état déjà existant |
| 2 | Script de benchmark reproductible (démarrage, RSS, taille binaire) vs OpenCode | Ressources | Faible-moyen | Élevé (si favorable) | Aucune |
| 3 | Formaliser le positionnement "zéro démon" dans la doc | Architecture | Faible | Moyen | Aucune |
| 4 | Export d'audit Plan Mode (`crustly plan export`) | Auditabilité | Moyen | Moyen-élevé | `src/plan/mod.rs` existant |
| 5 | Décision Go/No-Go sur le chargement GGUF direct | Architecture | Élevé (plusieurs semaines si GO) | Élevé à moyen terme | Voir `llm-file-gguf-support.md` §6 |

**Ordre recommandé** : 1 → 2 → 3 avant 4 et 5. Les trois premières
initiatives ne nécessitent aucun développement de fonctionnalité — seulement
de la mesure et de la documentation d'un état déjà réel — et peuvent être
livrées en quelques jours à quelques semaines cumulées, avec un rapport
effort/impact nettement meilleur que les initiatives 4 et 5, qui restent
pertinentes mais plus coûteuses.

---

## 5. Plan de benchmark (initiative #2, détail méthodologique)

Objectif : produire une comparaison **reproductible par un tiers**, pas une
affirmation. Métriques et méthode :

1. **Temps de démarrage à froid** : `hyperfine` (déjà l'outil standard pour
   ce type de mesure) sur `crustly --version`/`--help` vs l'équivalent
   OpenCode, plusieurs runs, même machine, même charge système.
2. **Empreinte mémoire au repos** : RSS mesuré via `/usr/bin/time -v` (Linux)
   ou équivalent, juste après démarrage, avant toute interaction — pour
   Crustly (process unique) vs OpenCode (process serveur + process client,
   les deux comptabilisés puisque c'est le coût réel pour l'utilisateur).
3. **Empreinte mémoire en session active** : même mesure après une tâche de
   complexité comparable (ex. lecture + édition d'un fichier, un appel
   d'outil), pour capturer l'usage réaliste et pas seulement l'idle.
4. **Taille de l'installation** : taille du binaire Crustly (`cargo build
   --release`, éventuellement `--profile release-small`) vs la taille totale
   de l'installation OpenCode (runtime Bun/Node inclus si nécessaire pour
   fonctionner).
5. **Publication** : script de mesure versionné dans le dépôt (reproductible
   par quiconque, sur sa propre machine), résultats bruts + méthodologie
   documentée, pas seulement un tableau de chiffres sans contexte — la
   crédibilité de ce benchmark dépend entièrement de sa vérifiabilité.

**Risque à anticiper** (voir aussi §6) : si le résultat n'est pas
favorable sur une métrique donnée (ex. si OpenCode a par ailleurs optimisé
son propre démarrage), publier quand même les chiffres bruts avec la
méthodologie plutôt que de ne publier que les métriques favorables — un
benchmark sélectif perd toute crédibilité dès qu'il est vérifié par un
tiers, ce qui est probable vu la popularité d'OpenCode.

---

## 6. Risques et limites honnêtes de cette stratégie

- **Effet de réseau non rattrapable par la technique seule** : même avec un
  avantage réel sur les quatre axes ci-dessus, l'écosystème (plugins,
  intégrations, effet de communauté) d'OpenCode continuera de peser lourd
  dans le choix de nombreux utilisateurs, indépendamment du mérite
  technique. Cette stratégie vise un segment plus restreint et spécifique,
  pas un basculement massif d'utilisateurs OpenCode vers Crustly.
- **OpenCode peut réagir** : rien n'empêche OpenCode d'ajouter un mode
  sandbox plus strict ou un mode "process unique" à l'avenir — ces axes de
  différenciation sont des avantages actuels, pas des garanties
  permanentes. Le benchmark (§5) devrait être revu périodiquement, pas
  publié une fois pour toutes.
- **Capacité de l'équipe** : les initiatives 4 et 5 de la feuille de route
  (§4) demandent un effort d'ingénierie réel (semaines) ; à ne lancer que
  si le budget correspondant est confirmé disponible sans repousser une
  autre priorité (même logique de prérequis que dans `llm-file-gguf-
  support.md` §6).
- **Le benchmark peut se retourner** : si Crustly perd sur une métrique
  mesurée (ex. temps de démarrage à froid dominé par un facteur imprévu),
  le publier quand même reste la bonne décision (transparence >
  sélectivité), mais cela doit être anticipé avant de s'engager
  publiquement sur ce terrain.

---

## 7. Suite à donner

1. Valider ou amender la liste d'axes (§3) et d'anti-objectifs (§2) avec
   les priorités produit réelles — cette étude propose une lecture du code
   existant, pas une décision finale.
2. Lancer les initiatives 1-3 de la feuille de route (§4), qui ne
   nécessitent aucun développement de fonctionnalité nouvelle.
3. Pour les initiatives 4-5, traiter chacune comme un projet séparé avec
   son propre cadrage (sur le modèle de `llm-file-gguf-support.md` pour la
   #5, qui existe déjà).

---

## Sources consultées

- Recherche web sur `opencode.ai` — échelle, architecture (Hono/OpenTUI/
  SolidJS/Electron), fonctionnalités (LSP, plugins, sub-agents, MCP,
  partage de session), licence
- Analyse comparative indépendante des modèles de permission (Claude Code
  vs Codex vs Cline vs OpenCode) — citation sur l'absence de
  sandbox/moteur de règles/hooks côté OpenCode
- `src/llm/tools/sandbox.rs`, `src/llm/tools/cache.rs`, `src/plan/mod.rs`,
  `src/config/mod.rs`, `Cargo.toml` (ce dépôt) — vérification directe des
  mécanismes de sécurité, du modèle Plan, et des réglages de build
- `docs/architecture/decisions/0004-plan-mode-read-only-with-approval-
  gating.md` — rationale du Plan Mode
- `docs/architecture/PERFORMANCE_PLAN.md` (ce dépôt) — travaux de
  performance déjà engagés, notamment sur l'inférence locale
- `llm-file-gguf-support.md` (ce dépôt) — évaluation liée sur le chargement
  GGUF direct, référencée comme initiative complémentaire à l'axe
  "architecture monolithique"
