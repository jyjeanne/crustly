# Gemma 4 26B A4B MoE
# Architecture interne

> Analyse technique de l'architecture Transformer, du routage MoE, de l'attention et des implications pour l'inférence locale.

---

# 1. Vue générale de l'architecture

Gemma 4 26B A4B utilise une architecture :

```
Decoder-only Transformer
+
Mixture of Experts (MoE)
+
Long Context Attention
+
Multimodal Encoder (selon variante)
```

Le pipeline global :

```
               Input Text

                    |
                    v

              Tokenizer

                    |
                    v

             Token Embeddings

                    |
                    v

        +-----------------------+
        | Transformer Layers    |
        |                       |
        |  Attention            |
        |       |               |
        |       v               |
        |  MoE Feed Forward     |
        |       |               |
        +-----------------------+

                    |
                    v

             Output Layer

                    |
                    v

              Next Token
```

---

# 2. Transformer Decoder Only

Gemma 4 appartient à la famille des modèles auto-régressifs.

Le modèle prédit le prochain token :

```
Input:

"function calculate"

Probabilités:

{
 "(": 0.72,
 "=": 0.12,
 "(": 0.08,
 ...
}

Choix:

"("
```

Puis il recommence :

```
function calculate(
```

devient l'entrée suivante.

---

# 3. Les blocs Transformer

Chaque couche contient généralement :

```
Input

 |

Layer Normalization

 |

Multi Head Attention

 |

Residual Connection

 |

Layer Normalization

 |

Feed Forward Network

 |

Residual Connection

 |

Output
```

---

# 4. Attention Mechanism

L'attention permet au modèle de savoir quels tokens sont importants.

Exemple :

```python
class User:
    def login(self):
        return self.password
```

Quand le modèle analyse :

```
self.password
```

l'attention peut relier :

```
password

↓

User attribute

↓

class User
```

---

# 5. Multi Head Attention

Au lieu d'une seule attention :

```
Attention
```

Gemma utilise plusieurs têtes :

```
             Attention

       +------+------+------+

       | Head | Head | Head |

       +------+------+------+

              |

          Combined

              |

          Output
```

Chaque tête apprend des relations différentes :

| Head | Exemple |
|-|-|
| 1 | Syntaxe |
| 2 | Dépendances |
| 3 | Structure |
| 4 | Relations longues |

---

# 6. Long Context Attention

Gemma 4 supporte :

```
256 000 tokens
```

Cela nécessite des optimisations.

Un contexte classique :

```
Token1 Token2 Token3 Token4
```

avec attention complète :

```
Chaque token regarde tous les autres
```

Complexité :

```
O(n²)
```

Pour 256k tokens cela devient impossible.

---

# 7. Sliding Window Attention

Gemma utilise une combinaison :

```
Local Attention

+

Global Attention
```

Schéma :

```
Token actuel

       |
       |
+------+------+------+
|      |      |      |
T-3   T-2    T-1     T

       |

Attention locale


+

Tokens importants globaux
```

Avantages :

- moins de calcul ;
- contexte long ;
- meilleure efficacité.

---

# 8. Feed Forward Network

Dans un Transformer classique :

```
Attention

   |

MLP

   |

Output
```

Le MLP contient énormément de paramètres.

Dans Gemma 4 :

```
Attention

   |

Router

   |

Experts

   |

Output
```

Le MLP devient un système MoE.

---

# 9. Architecture Mixture of Experts

Gemma 4 26B possède :

```
128 experts
```

Mais tous ne travaillent pas ensemble.

Pour chaque token :

```
Token

 |

Router

 |

Choix experts

 |

8 experts actifs

 |

Fusion

 |

Output
```

---

# 10. Le Router

Le router est un petit réseau qui décide :

"Quels experts sont les meilleurs pour ce token ?"

Exemple :

Token :

```
async function
```

Le router peut choisir :

```
Expert 12
Expert 33
Expert 48
Expert 91
...
```

Pour un token :

```
docker-compose.yml
```

il peut choisir d'autres experts.

---

# 11. Experts spécialisés

Un expert peut apprendre davantage :

- langage naturel ;
- code ;
- mathématiques ;
- raisonnement ;
- langues ;
- documentation.

Le modèle développe automatiquement ces spécialisations pendant l'entraînement.

---

# 12. Paramètres actifs

Gemma 4 26B :

Paramètres totaux :

```
≈ 26 milliards
```

Mais actifs :

```
≈ 3.8 milliards/token
```

Comparaison :

## Modèle dense

```
Token

 |

26B paramètres

 |

Calcul
```

## Gemma MoE

```
Token

 |

Router

 |

3.8B paramètres

 |

Calcul
```

---

# 13. Impact sur Ollama

Le nombre total de paramètres influence :

- taille du modèle ;
- stockage ;
- RAM nécessaire.

Les paramètres actifs influencent :

- vitesse ;
- tokens/seconde ;
- coût calcul.

Donc :

Gemma 4 26B peut être plus rapide qu'un dense 26B.

---

# 14. KV Cache

Pendant la génération, Ollama conserve une mémoire appelée :

```
Key Value Cache
```

Elle évite de recalculer les tokens précédents.

Sans KV Cache :

```
Token 1000

recalcule tokens 1-999
```

Avec :

```
Token 1000

utilise cache existant
```

---

# 15. Impact du contexte sur la mémoire

Le KV Cache augmente avec :

- taille contexte ;
- nombre couches ;
- taille batch.

Exemple :

```
num_ctx 8192

↓

faible mémoire


num_ctx 131072

↓

beaucoup plus mémoire
```

---

# 16. Architecture et RTX 3060

Configuration :

```
RTX 3060 12GB

+

34GB RAM

+

Ollama
```

Stratégie :

```
GPU

↓

Poids principaux


RAM

↓

Cache + contexte
```

---

# 17. Recommandation Crustly

Pour un agent de code :

```
num_ctx = 65536
```

est un bon compromis.

Pourquoi :

- assez grand pour un projet ;
- latence raisonnable ;
- consommation mémoire maîtrisée.

---

# 18. Résumé architecture

| Élément | Fonction |
|-|-|
| Transformer | Compréhension séquentielle |
| Attention | Relations entre tokens |
| MoE Router | Sélection experts |
| Experts | Capacités spécialisées |
| KV Cache | Mémoire génération |
| Sliding Attention | Long contexte |
| Tokenizer | Conversion texte/token |

---

# Conclusion

Gemma 4 26B A4B n'est pas simplement un modèle de 26 milliards de paramètres.

C'est un système hybride :

```
26B paramètres disponibles

        +

3.8B actifs par token

        +

Architecture MoE

        +

Long Context

        +

Tool Calling
```

Cette architecture explique pourquoi il peut fournir des performances élevées tout en restant utilisable localement.

---

# Chapitre suivant

```
03-mixture-of-experts.md
```

Le prochain chapitre détaillera :

- fonctionnement exact du routage ;
- experts partagés ;
- équilibrage des experts ;
- impact sur les performances ;
- réglages Ollama pour exploiter correctement le MoE.
