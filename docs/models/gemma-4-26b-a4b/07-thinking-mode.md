# Gemma 4 26B A4B MoE
# Thinking Mode

> Documentation technique du mode de raisonnement (thinking) de Gemma 4, basée sur la documentation officielle Ollama ([ollama.com/library/gemma4:26b](https://ollama.com/library/gemma4:26b)).

---

# 1. Introduction

Gemma 4 sépare la réflexion interne du modèle de sa réponse finale, comme introduit conceptuellement dans [`01-introduction.md`](01-introduction.md#12-gemma-4-et-raisonnement).

Ce chapitre documente le mécanisme concret : les tokens de contrôle, le format de sortie, et son comportement en conversation multi-tours.

---

# 2. Activer/désactiver le thinking

Le thinking est contrôlé par un token spécial placé au début du system prompt :

```
<|think|>
```

- **Activer** : inclure `<|think|>` au début du system prompt.
- **Désactiver** : retirer ce token du system prompt.

---

# 3. Format de sortie

## Thinking activé

Quand le thinking est activé, le modèle produit d'abord son raisonnement interne, puis la réponse finale, selon cette structure :

```
<|channel>thought\n[Internal reasoning]<channel|>
```

suivi de la réponse finale.

## Thinking désactivé

Pour tous les modèles de la famille **sauf E2B et E4B**, si le thinking est désactivé, le modèle génère toujours les balises, mais avec un bloc de réflexion vide :

```
<|channel>thought\n<channel|>[Final answer]
```

> Les variantes E2B/E4B n'émettent pas ces balises quand le thinking est désactivé.

---

# 4. Gestion en conversation multi-tours

Règle importante documentée par Google/Ollama :

> Dans une conversation multi-tours, la sortie historique du modèle ne doit contenir que la réponse finale. Les réflexions ("thoughts") des tours précédents ne doivent jamais être réinjectées avant le tour utilisateur suivant.

Schéma :

```
Tour 1

  User -> Assistant (thought + answer)

Historique conservé pour Tour 2:

  User -> Assistant (answer only, thought stripped)

Tour 2

  User -> Assistant (new thought + answer)
```

---

# 5. Ce qu'Ollama gère automatiquement

La documentation officielle précise :

> "Ollama already handles the complexities of the chat template for you."

En pratique, Ollama applique lui-même le chat template natif du modèle (y compris l'insertion/retrait du token `<|think|>` et le parsing des balises `<|channel|>`) à partir du paramètre `think` de l'API native `/api/chat` — l'appelant n'a pas besoin de manipuler ces tokens manuellement.

---

# 6. Correspondance avec l'implémentation Crustly

Le provider Ollama natif de Crustly (`src/llm/provider/ollama.rs`) traite déjà le thinking de façon générique, sans connaissance spécifique de Gemma 4 :

```rust
let think = request.thinking.as_ref().map(|t| match t.budget_tokens {
    0..=2_000 => ThinkType::Low,
    2_001..=8_000 => ThinkType::Medium,
    _ => ThinkType::High,
});
```

Ce `ThinkType` est transmis tel quel à `ollama-rs`, qui l'envoie comme paramètre `think` de l'API native. C'est exactement le mécanisme que la documentation officielle décrit comme géré côté Ollama — **aucune modification de code n'était nécessaire** pour supporter le thinking mode de Gemma 4 : le modèle bénéficie du même chemin générique déjà utilisé pour DeepSeek-R1 et QwQ-32B.

Côté TUI, l'utilisateur peut afficher/masquer le panneau de réflexion avec la touche `t` (voir le comportement documenté dans `CLAUDE.md` sous "Streaming Architecture").

---

# 7. Bonnes pratiques

- Ne jamais réinjecter les blocs `thought` précédents dans l'historique envoyé au modèle.
- Pour un agent (Crustly), préférer un budget de thinking proportionné à la complexité de la tâche (`budget_tokens` bas pour des actions simples, élevé pour un debugging complexe ou une planification multi-étapes).
- Le thinking désactivé n'élimine pas la latence des balises vides sur les variantes non-edge (E2B/E4B) — c'est un comportement du modèle, pas un bug d'intégration.

---

# Chapitre suivant

```
08-tool-calling.md
```

Déjà rédigé — voir [`08-tool-calling.md`](08-tool-calling.md) pour le Function Calling.
