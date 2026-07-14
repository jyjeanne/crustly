# Gemma 4 26B A4B MoE
# Mixture of Experts (MoE)

> Documentation technique du système Mixture of Experts utilisé par Gemma 4 26B A4B. Ce document explique le routage des experts, l'architecture sparse, l'impact sur l'inférence locale et les implications pour Ollama et Crustly.

---

# 1. Introduction au Mixture of Experts

Le Mixture of Experts (MoE) est une architecture permettant à un modèle d'avoir un très grand nombre de paramètres tout en n'activant qu'une partie de ces paramètres pour chaque token généré.

Un modèle Transformer classique est dense :

```
Token
 |
 v
Tous les paramètres du modèle
 |
 v
Sortie
```

Un modèle MoE :

```
Token
 |
 v
Router
 |
 +------------+
 |            |
 v            v

Expert A    Expert B

 |            |

 +------------+

       |
       v

    Sortie
```

Seuls certains experts participent au calcul.

---

# 2. Pourquoi utiliser un MoE ?

Les modèles denses ont un problème :

Plus de paramètres =

- meilleure connaissance ;
- meilleur raisonnement ;
- meilleure compréhension ;

mais aussi :

- plus de VRAM ;
- plus de calcul ;
- plus de latence.

Le MoE cherche un compromis :

```
Grande capacité mémoire

+

Petit coût d'inférence
```

---

# 3. Gemma 4 26B A4B Architecture MoE

Gemma 4 26B utilise :

| Élément | Valeur |
|-|-|
| Paramètres totaux | 25.2B |
| Paramètres actifs | ~3.8B/token |
| Nombre experts | 128 |
| Experts actifs | 8 |
| Expert partagé | Oui |

Le modèle possède donc une grande réserve de connaissances mais n'utilise qu'une partie à chaque étape.

---

# 4. Architecture d'un bloc MoE

Un bloc Transformer classique :

```
Input

 |

Attention

 |

Feed Forward Network

 |

Output
```

Gemma 4 :

```
Input

 |

Attention

 |

Router

 |

+----------------+
|                |
v                v

Expert 1      Expert 2

Expert 3      Expert 4

...

Expert 128

+----------------+

 |

Expert Fusion

 |

Output
```

---

# 5. Le Router

Le router est un petit réseau neuronal spécialisé.

Son rôle :

> « Déterminer quels experts sont les plus pertinents pour chaque token. »

Exemple :

Entrée :

```
async function authenticate()
```

Le router peut sélectionner :

```
Expert 12
Expert 43
Expert 67
Expert 89
...
```

Alors qu'un autre token :

```
docker-compose.yml
```

peut activer :

```
Expert 4
Expert 18
Expert 55
Expert 101
...
```

---

# 6. Sélection Top-K Experts

Gemma 4 utilise une stratégie :

```
Top-K Routing
```

Processus :

```
Token

 |

Router calcule scores

 |

Classement experts

 |

Sélection meilleurs experts

 |

Calcul parallèle

 |

Fusion
```

Exemple :

Scores :

```json
{
  "expert_1": 0.02,
  "expert_25": 0.91,
  "expert_48": 0.84,
  "expert_90": 0.77
}
```

Les experts avec les meilleurs scores sont activés.

---

# 7. Experts spécialisés

Les experts ne sont pas assignés manuellement.

Pendant l'entraînement, ils développent naturellement des spécialisations.

Exemples possibles :

| Expert | Domaine appris |
|-|-|
| Expert A | Python |
| Expert B | JavaScript |
| Expert C | Raisonnement |
| Expert D | Documentation |
| Expert E | Mathématiques |
| Expert F | Langues |

Le modèle apprend automatiquement cette répartition.

---

# 8. Expert partagé (Shared Expert)

Gemma 4 utilise également un expert partagé.

Architecture :

```
             Token

               |

             Router

        +------+------+

        |             |

    Selected       Shared

    Experts        Expert

        |             |

        +------+------+

               |

             Output
```

Avantages :

- meilleure stabilité ;
- connaissances générales toujours disponibles ;
- moins de pertes lors du routage.

---

# 9. MoE et génération de code

Pour un assistant comme Crustly :

Question :

```
Refactor this authentication system
```

Le modèle peut activer :

```
Expert sécurité

+

Expert architecture

+

Expert Python

+

Expert tests

+

Expert documentation
```

Ce mécanisme est particulièrement adapté aux tâches complexes.

---

# 10. MoE et raisonnement

Lors d'une tâche difficile :

```
Analyse bug complexe
```

Le routeur peut favoriser des experts associés à :

- logique ;
- planification ;
- analyse ;
- résolution de problèmes.

---

# 11. Différence Dense vs MoE

## Modèle Dense

Exemple :

```
Llama 70B
```

Chaque token :

```
70B paramètres actifs
```

Avantages :

- comportement homogène ;
- simplicité.

Inconvénients :

- très lourd.

---

## Modèle MoE

Exemple :

```
Gemma 4 26B A4B
```

Chaque token :

```
~3.8B paramètres actifs
```

Avantages :

- rapide ;
- efficace ;
- grande capacité.

---

# 12. Impact sur la VRAM

La mémoire nécessaire dépend de :

- poids du modèle ;
- quantification ;
- contexte ;
- KV Cache.

Exemple :

```
Modèle complet

26B paramètres

        |

Quantification Q4

        |

GPU + RAM
```

Le calcul actif est inférieur mais les poids restent nécessaires.

---

# 13. Quantification MoE

La quantification réduit la taille des poids.

Formats courants :

| Format | Qualité | Mémoire |
|-|-|-|
| FP16 | Maximum | Très élevée |
| Q8 | Très haute | Haute |
| Q6 | Excellente | Moyenne |
| Q5 | Très bonne | Réduite |
| Q4 | Bon compromis | Faible |

Pour RTX 3060 :

```
Q4_K_M
```

est généralement le meilleur compromis.

---

# 14. Impact sur Ollama

Ollama utilise :

- GGUF ;
- llama.cpp backend ;
- offload GPU.

Configuration :

```bash
ollama run gemma4:26b
```

Ollama gère automatiquement :

- chargement experts ;
- allocation mémoire ;
- GPU layers.

---

# 15. Optimisation Crustly

Pour un agent logiciel :

Recommandation :

```
Model: gemma4:26b
Context: 65536
Temperature: 0.1-0.2
Top_k: 20
Top_p: 0.9
```

---

# 16. Problèmes possibles du MoE

## Routage instable

Symptômes :

- réponses incohérentes ;
- changement brutal de style.

Solution :

Réduire :

```
temperature
```

---

## Experts sous-utilisés

Pendant l'entraînement, certains experts peuvent recevoir trop peu de tokens.

Solution :

Techniques :

- load balancing loss ;
- expert capacity control.

---

# 17. MoE et agents autonomes

Le MoE est particulièrement adapté aux agents.

Un agent doit alterner :

```
Comprendre

↓

Planifier

↓

Coder

↓

Tester

↓

Documenter
```

Chaque étape peut solliciter des capacités différentes.

---

# 18. Architecture recommandée Crustly

```
              User

               |

             Crustly

               |

        Agent Controller

               |

          Gemma 4 MoE

               |

     +---------+---------+

     |                   |

  Tools              Reasoning

     |

Filesystem / Git / Terminal
```

---

# 19. Résumé technique

| Concept | Description |
|-|-|
| MoE | Plusieurs réseaux spécialisés |
| Router | Sélection experts |
| Top-K | Nombre experts actifs |
| Shared Expert | Connaissance générale |
| Sparse Activation | Calcul réduit |
| Active Parameters | Paramètres utilisés |
| Total Parameters | Capacité totale |

---

# Conclusion

Gemma 4 26B A4B n'est pas un simple modèle de 25.2 milliards de paramètres.

Son architecture MoE lui permet d'obtenir :

- une grande capacité ;
- une meilleure efficacité ;
- une excellente adaptation aux agents ;
- une consommation adaptée au calcul local.

Pour Crustly, le MoE est particulièrement intéressant car il permet de combiner :

```
Grande intelligence

+

Coût local raisonnable

+

Capacité agentique
```

---

# Chapitre suivant

```
04-tokenizer.md
```

Ce chapitre détaillera :

- le tokenizer Gemma 4 ;
- vocabulaire ;
- tokens spéciaux ;
- encodage ChatML ;
- impact sur le contexte ;
- optimisation des prompts.
