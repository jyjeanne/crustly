# Gemma 4 26B A4B MoE
# Structured JSON Output

> Documentation technique des sorties JSON structurées avec Gemma 4, Ollama et Crustly. Ce chapitre explique comment forcer le modèle à produire des réponses machine-readable, valider les sorties et construire des agents fiables.

---

# 1. Introduction

Un LLM produit naturellement du texte :

```
The bug is caused by a missing null check.
You should update the function.
```

Ce format est adapté aux humains mais difficile à exploiter par un agent.

Un agent comme Crustly nécessite des sorties structurées :

```json
{
  "action": "modify_file",
  "file": "src/auth.py",
  "changes": [
    "Add null validation"
  ]
}
```

Le JSON permet :

- validation automatique ;
- exécution d'actions ;
- orchestration d'agents ;
- communication entre composants.

---

# 2. Architecture JSON avec Crustly

Architecture recommandée :

```
                 User

                  |

               Crustly

                  |

            Gemma 4 26B

                  |

          Structured JSON

                  |

        JSON Validator

                  |

     +------------+------------+

     |                         |

 Execute Action          Request Fix
```

Le modèle propose une action.

Crustly décide si elle est valide.

---

# 3. Modes de sortie JSON

Gemma 4 peut être utilisé avec plusieurs niveaux de contrainte.

## Mode libre

Le modèle répond normalement :

```json
{
  "content": "Here is the explanation..."
}
```

---

## Mode JSON demandé

Le prompt impose :

```
Return only valid JSON.
```

Exemple :

```json
{
  "bug": "NullPointerException",
  "solution": "Add validation"
}
```

---

## Mode JSON Schema

Le format est imposé :

```json
{
  "type": "object",
  "properties": {
    "action": {
      "type": "string"
    }
  }
}
```

C'est le mode recommandé pour les agents.

---

# 4. Requête Ollama JSON simple

Endpoint :

```
POST http://localhost:11434/api/chat
```

Exemple :

```json
{
  "model": "gemma4:26b",
  "messages": [
    {
      "role": "system",
      "content": "Return only JSON."
    },
    {
      "role": "user",
      "content": "Analyze this error."
    }
  ],
  "format": "json"
}
```

---

# 5. Réponse Ollama

Exemple :

```json
{
  "model": "gemma4:26b",
  "message": {
    "role": "assistant",
    "content": {
      "error": "Missing import",
      "severity": "medium",
      "solution": "Add required module"
    }
  },
  "done": true
}
```

---

# 6. JSON Schema

Pour un agent de programmation :

```json
{
  "type": "object",
  "required": [
    "action",
    "target",
    "reason"
  ],
  "properties": {
    "action": {
      "type": "string",
      "enum": [
        "read",
        "modify",
        "create",
        "delete"
      ]
    },
    "target": {
      "type": "string"
    },
    "reason": {
      "type": "string"
    }
  }
}
```

---

# 7. Exemple Agent Crustly

Demande :

```
Fix authentication bug.
```

Réponse Gemma :

```json
{
  "action": "modify",
  "target": "src/auth/login.py",
  "reason": "Missing password validation",
  "changes": [
    {
      "line": 42,
      "operation": "insert",
      "code": "validate_password(password)"
    }
  ]
}
```

---

# 8. Génération de patch JSON

Format recommandé :

```json
{
  "patch": {
    "file": "src/api/user.py",
    "operations": [
      {
        "type": "replace",
        "start_line": 20,
        "end_line": 25,
        "content": "new code"
      }
    ]
  }
}
```

Avantages :

- traçable ;
- réversible ;
- compatible Git.

---

# 9. Analyse de projet JSON

Exemple :

```json
{
  "project": {
    "language": "Python",
    "framework": "FastAPI",
    "files_analyzed": 245
  },
  "issues": [
    {
      "type": "security",
      "file": "auth.py",
      "priority": "high"
    }
  ]
}
```

---

# 10. Sortie JSON pour Tool Calling

Structure :

```json
{
  "tool": "read_file",
  "arguments": {
    "path": "src/main.py"
  }
}
```

Crustly transforme :

```json
{
  "name": "read_file",
  "arguments": {
    "path": "src/main.py"
  }
}
```

en appel réel.

---

# 11. Validation côté application

Toujours valider.

Exemple Python :

```python
import json
from jsonschema import validate

data = json.loads(response)

validate(
    instance=data,
    schema=my_schema
)
```

---

# 12. Gestion des erreurs JSON

Problème :

```
Here is the JSON:

{
  "action": "modify"
}
```

Le texte avant le JSON casse le parser.

Solution :

Prompt :

```
Output ONLY JSON.
No markdown.
No explanation.
```

---

# 13. Prompt système recommandé

```
You are a software engineering agent.

Rules:

- Output only valid JSON.
- Follow the provided schema.
- Never add explanations outside JSON.
- Never invent missing information.
- Ask for clarification using the "request_info" action.
```

---

# 14. Paramètres Gemma recommandés

Pour JSON :

```json
{
  "temperature": 0.1,
  "top_p": 0.9,
  "top_k": 20,
  "repeat_penalty": 1.05
}
```

> Note : Google/Ollama documentent une configuration standardisée différente (`temperature=1.0, top_p=0.95, top_k=64`) comme recommandation générale "pour tous les cas d'usage" (voir [`README.md`](README.md#recommended-parameters)). Les valeurs basses ci-dessus sont une recommandation Crustly, pas une valeur officielle du fournisseur, à utiliser si l'on privilégie la conformité stricte du JSON au détriment du comportement par défaut du modèle.

Pourquoi :

- moins de variations ;
- moins d'erreurs syntaxiques ;
- meilleure conformité.

---

# 15. Actions agent recommandées

Schema Crustly :

```json
{
  "action": [
    "inspect_project",
    "read_file",
    "search_code",
    "modify_file",
    "create_file",
    "run_test",
    "commit_change"
  ]
}
```

---

# 16. Exemple workflow complet

Etape 1

Utilisateur :

```
Improve API security.
```

Etape 2

Gemma :

```json
{
  "action": "inspect_project"
}
```

Etape 3

Crustly exécute.

Retour :

```json
{
  "files": [
    "auth.py",
    "api.py"
  ]
}
```

Etape 4

Gemma :

```json
{
  "action": "modify_file",
  "target": "auth.py"
}
```

---

# 17. JSON Streaming

Avec streaming :

```
{
"action":
```

puis :

```
{
"action":"modify_file"
```

puis :

```json
{
  "action": "modify_file",
  "target": "main.py"
}
```

Le client doit reconstruire le JSON avant validation.

---

# 18. Bonnes pratiques

Toujours :

- utiliser un schema ;
- valider côté application ;
- journaliser les actions ;
- conserver l'historique ;
- demander confirmation pour actions dangereuses.

---

Ne jamais :

- exécuter directement un JSON généré ;
- donner accès root ;
- accepter une suppression sans validation.

---

# 19. JSON vs Tool Calling

| Besoin | Solution |
|-|-|
| Réponse structurée | JSON Output |
| Action externe | Tool Calling |
| Agent autonome | Les deux |
| Patch code | JSON Schema |
| Terminal | Tool Calling |

---

# 20. Résumé

Les sorties JSON transforment Gemma 4 en composant fiable pour un agent logiciel.

Architecture recommandée :

```
Gemma 4

+

JSON Schema

+

Validator

+

Tool Executor

+

Crustly Controller

=

Agent développeur local sécurisé
```

---

# Chapitre suivant

```
10-ollama-api.md
```

Ce chapitre détaillera :

- API REST Ollama complète ;
- endpoints ;
- paramètres ;
- requêtes curl ;
- Python SDK ;
- JavaScript ;
- gestion streaming.
