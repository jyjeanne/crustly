# Gemma 4 26B A4B MoE
# Benchmarks

> Résultats d'évaluation officiels, transcrits depuis [ollama.com/library/gemma4:26b](https://ollama.com/library/gemma4:26b). Les scores sont ceux des versions instruction-tuned.

---

# 1. Famille Gemma 4 comparée

| Model | Total Params | Active Params | Context | Modalities |
|-|-|-|-|-|
| Gemma 4 E2B | 2.3B effective (5.1B w/ embeddings) | 2.3B | 128K | Text, Image, Audio |
| Gemma 4 E4B | 4.5B effective (8B w/ embeddings) | 4.5B | 128K | Text, Image, Audio |
| Gemma 4 12B | — | — | 128K | Text, Image |
| **Gemma 4 26B A4B (MoE)** | 25.2B | 3.8B | 256K | Text, Image |
| Gemma 4 31B (Dense) | 30.7B | 30.7B | 256K | Text, Image |

`E2B`/`E4B` = "effective parameters", optimisés pour déploiement sur appareils edge (laptop/mobile). Detailed per-variant architecture specs (layers, sliding window, vocab, encoder sizes) are in [`02-architecture.md`](02-architecture.md).

---

# 2. Benchmarks généraux

| Benchmark | Gemma 4 31B | Gemma 4 26B A4B | Gemma 4 E4B | Gemma 4 E2B | Gemma 3 27B (no think) |
|-|-|-|-|-|-|
| MMLU Pro | 85.2% | 82.6% | 69.4% | 60.0% | 67.6% |
| AIME 2026 no tools | 89.2% | 88.3% | 42.5% | 37.5% | 20.8% |
| LiveCodeBench v6 | 80.0% | 77.1% | 52.0% | 44.0% | 29.1% |
| Codeforces ELO | 2150 | 1718 | 940 | 633 | 110 |
| GPQA Diamond | 84.3% | 82.3% | 58.6% | 43.4% | 42.4% |
| Tau2 (average over 3) | 76.9% | 68.2% | 42.2% | 24.5% | 16.2% |
| HLE no tools | 19.5% | 8.7% | - | - | - |
| HLE with search | 26.5% | 17.2% | - | - | - |
| BigBench Extra Hard | 74.4% | 64.8% | 33.1% | 21.9% | 19.3% |
| MMMLU | 88.4% | 86.3% | 76.6% | 67.4% | 70.7% |

---

# 3. Vision

| Benchmark | Gemma 4 31B | Gemma 4 26B A4B | Gemma 4 E4B | Gemma 4 E2B | Gemma 3 27B (no think) |
|-|-|-|-|-|-|
| MMMU Pro | 76.9% | 73.8% | 52.6% | 44.2% | 49.7% |
| OmniDocBench 1.5 (avg edit distance, lower is better) | 0.131 | 0.149 | 0.181 | 0.290 | 0.365 |
| MATH-Vision | 85.6% | 82.4% | 59.5% | 52.4% | 46.0% |
| MedXPertQA MM | 61.3% | 58.1% | 28.7% | 23.5% | - |

---

# 4. Audio

Seules les variantes edge (E2B/E4B) supportent l'audio — voir [`README.md`](README.md#model-summary) et [`02-architecture.md`](02-architecture.md), Gemma 4 26B A4B (MoE) ne supporte que texte + image.

| Benchmark | Gemma 4 E4B | Gemma 4 E2B |
|-|-|-|
| CoVoST | 35.54 | 33.47 |
| FLEURS (lower is better) | 0.08 | 0.09 |

---

# 5. Long Context

| Benchmark | Gemma 4 31B | Gemma 4 26B A4B | Gemma 4 E4B | Gemma 4 E2B | Gemma 3 27B (no think) |
|-|-|-|-|-|-|
| MRCR v2 8 needle 128k (average) | 66.4% | 44.1% | 25.4% | 19.1% | 13.5% |

---

# 6. Lecture des résultats pour Crustly

Points clés pour le choix du modèle dans un contexte agentique :

- **Codeforces ELO (1718) et LiveCodeBench v6 (77.1%)** : Gemma 4 26B A4B reste largement au-dessus de Gemma 3 27B sur la génération de code compétitif, malgré ~3.8B paramètres actifs contre un modèle dense équivalent.
- **Tau2 (68.2%)** : Tau2 mesure des capacités agentiques (utilisation d'outils, suivi d'instructions multi-étapes) — pertinent directement pour l'usage Crustly.
- **MRCR v2 (44.1% à 128k)** : la dégradation en long contexte existe mais reste très supérieure à Gemma 3 27B (13.5%) — cohérent avec les optimisations d'attention décrites dans [`02-architecture.md`](02-architecture.md#6-long-context-attention).
- **Écart avec le 31B Dense** : sur presque tous les benchmarks, le 31B Dense dépasse le 26B A4B MoE de quelques points, ce qui est attendu (30.7B paramètres actifs contre 3.8B) — le compromis MoE reste favorable en local du fait du coût de calcul réduit (voir [`03-mixture-of-experts.md`](03-mixture-of-experts.md#11-différence-dense-vs-moe)).

---

# Chapitre suivant

```
21-troubleshooting.md
```

Ce chapitre détaillera les problèmes courants (routage MoE instable, sorties JSON invalides, performance dégradée) et leurs solutions.
