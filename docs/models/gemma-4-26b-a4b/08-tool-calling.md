# Gemma 4 26B A4B MoE
# Tool Calling

> Documentation technique du système d'appel d'outils (Function Calling) pour intégration avec Ollama, agents IA et Crustly.

---

# 1. Introduction

Le Tool Calling permet au modèle Gemma 4 de demander l'exécution d'actions externes.

Un LLM classique :

```
Utilisateur
    |
    v
Modèle
    |
    v
Réponse texte
```

Un agent avec Tool Calling :

```
Utilisateur
    |
    v
Agent Crustly
    |
    v
Gemma 4
    |
    v
Décision d'utiliser un outil
    |
    v
Exécution outil
    |
    v
Résultat retourné au modèle
    |
    v
Réponse finale
```

---

# 2. Pourquoi utiliser Tool Calling avec Crustly

Pour un assistant de développement, le modèle ne doit pas seulement générer du texte.

Il doit pouvoir :

- lire un fichier ;
- rechercher dans un projet ;
- exécuter des tests ;
- lancer Git ;
- analyser des logs ;
- modifier du code ;
- appeler une API.

Le modèle décide quoi faire, mais Crustly contrôle comment l'action est exécutée.

---

# 3. Architecture générale

```
                 Crustly

                    |
                    |

              Agent Controller

                    |
                    |

              Gemma 4 LLM

                    |
        +-----------+-----------+
        |                       |
        v                       v

    Texte final             Tool Call


                                |
                                v

                         Tool Executor

                                |
                                v

                         Résultat outil

                                |
                                v

                         Retour LLM
```

---

# 4. Format conceptuel d'un outil

Un outil possède :

- un nom ;
- une description ;
- des paramètres ;
- un résultat.

Exemple :

```json
{
  "name": "read_file",
  "description": "Read content of a source file",
  "parameters": {
    "path": {
      "type": "string",
      "description": "File path"
    }
  }
}
```

---

# 5. Déclaration des outils

Exemple OpenAI compatible :

```json
{
  "tools": [
    {
      "type": "function",
      "function": {
        "name": "read_file",
        "description": "Read a file from the repository",
        "parameters": {
          "type": "object",
          "properties": {
            "path": {
              "type": "string"
            }
          },
          "required": [
            "path"
          ]
        }
      }
    }
  ]
}
```

---

# 6. Exemple complet avec Ollama API

Requête

```
POST http://localhost:11434/api/chat
Content-Type: application/json
```

```json
{
  "model": "gemma4:26b",
  "messages": [
    {
      "role": "system",
      "content": "You are a senior software engineer."
    },
    {
      "role": "user",
      "content": "Analyze src/main.py and find possible bugs."
    }
  ],
  "tools": [
    {
      "type": "function",
      "function": {
        "name": "read_file",
        "description": "Read source file",
        "parameters": {
          "type": "object",
          "properties": {
            "path": {
              "type": "string"
            }
          }
        }
      }
    }
  ]
}
```

---

# 7. Réponse Tool Call

Le modèle peut répondre :

```json
{
  "message": {
    "role": "assistant",
    "tool_calls": [
      {
        "function": {
          "name": "read_file",
          "arguments": {
            "path": "src/main.py"
          }
        }
      }
    ]
  }
}
```

Le modèle ne lit pas réellement le fichier.

Il demande :

```
Crustly,
exécute read_file("src/main.py")
```

---

# 8. Exécution du Tool

Crustly exécute la fonction :

```python
def read_file(path):

    with open(path, "r") as file:
        return file.read()
```

Résultat :

```python
class Application:

    def start():
        ...
```

---

# 9. Retour du résultat au modèle

Le résultat est envoyé avec un message :

```json
{
  "role": "tool",
  "content": "class Application:\n..."
}
```

Le modèle continue :

```
Analyse du fichier

↓

Détection problème

↓

Proposition correction
```

---

# 10. Outils recommandés pour Crustly

## Filesystem

| Outil | Fonction |
|-|-|
| read_file | Lire fichier |
| write_file | Modifier fichier |
| list_directory | Explorer projet |
| search_code | Recherche texte |

---

## Git

| Outil | Fonction |
|-|-|
| git_status | Etat dépôt |
| git_diff | Voir modifications |
| git_commit | Créer commit |

---

## Terminal

| Outil | Fonction |
|-|-|
| execute_command | Exécuter commande |
| run_tests | Lancer tests |
| install_package | Installer dépendance |

---

# 11. Exemple workflow développeur

Utilisateur :

```
Corrige le bug dans l'API utilisateur
```

Agent :

1. Recherche fichiers API

Tool :

```json
{
  "name": "search_code",
  "arguments": {
    "query": "UserController"
  }
}
```

---

Agent :

2. Lecture fichier trouvé

Tool :

```json
{
  "name": "read_file",
  "arguments": {
    "path": "src/controllers/user.py"
  }
}
```

---

Agent :

3. Modification

Tool :

```json
{
  "name": "write_file",
  "arguments": {
    "path": "src/controllers/user.py",
    "content": "..."
  }
}
```

---

Agent :

4. Validation

Tool :

```json
{
  "name": "run_tests"
}
```

---

# 12. Gestion des erreurs

Un outil doit toujours retourner une erreur structurée.

Exemple :

```json
{
  "success": false,
  "error": {
    "type": "FILE_NOT_FOUND",
    "message": "src/test.py does not exist"
  }
}
```

Le modèle peut alors corriger sa stratégie.

---

# 13. Bonnes pratiques

## Toujours fournir une description claire

Mauvais :

```
execute()
```

Bon :

```
execute_shell_command:
Execute a safe shell command inside the project directory.
```

---

## Limiter les permissions

Ne jamais exposer :

- suppression système ;
- accès root ;
- secrets ;
- clés API.

---

## Ajouter des validations

Avant :

```
write_file()
```

Faire :

```
validate_patch()

↓

apply_change()

↓

run_tests()
```

---

# 14. Paramètres recommandés Gemma 4

Pour Tool Calling :

```json
{
  "temperature": 0.1,
  "top_p": 0.9,
  "top_k": 20,
  "repeat_penalty": 1.05
}
```

> Note : Google/Ollama documentent une configuration standardisée différente (`temperature=1.0, top_p=0.95, top_k=64`) comme recommandation générale "pour tous les cas d'usage", y compris agentique (voir [`README.md`](README.md#recommended-parameters)). Les valeurs basses ci-dessus sont une recommandation Crustly, pas une valeur officielle du fournisseur, à utiliser si l'on privilégie la déterminisme du tool calling au détriment du comportement par défaut du modèle.

Pourquoi :

- moins d'hallucinations ;
- appels plus cohérents ;
- meilleure stabilité.

---

# 15. Prompt système recommandé

```
You are an autonomous software engineer.

Rules:

- Always inspect files before modifying.
- Use tools when information is missing.
- Never invent file contents.
- Validate modifications with tests.
- Explain only after completing actions.
```

---

# 16. Sécurité Agent

Architecture recommandée :

```
Gemma 4

   |
   v

Crustly Permission Layer

   |
   +-- Allowed tools
   |
   +-- Sandbox
   |
   +-- Logs
   |
   +-- Approval system
```

Le modèle ne doit jamais avoir un accès direct au système.

---

# 17. Résumé

Le Tool Calling transforme Gemma 4 26B en véritable moteur agentique.

Avec Crustly :

```
Gemma 4
+
Ollama
+
Tool Calling
+
Sandbox
+
Git
+
Filesystem

=

Assistant développeur local autonome
```

---

# Chapitre suivant

```
09-json-output.md
```

Ce chapitre détaillera :

- sorties structurées ;
- JSON Schema ;
- validation automatique ;
- génération de patchs ;
- intégration avec les agents Crustly.
