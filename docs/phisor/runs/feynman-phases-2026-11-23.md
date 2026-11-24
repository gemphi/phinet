# Feynman's run — crates/phiano/data/person/murphy.txt

held out: 3176 sentences · the aligned arrows' seed: 7 (the opening's; the closing's is seed ^ 1)

| arm | mode | scored | opening | closing | both ends | both ≥ better end + 2 | disagree | probabilities added | amplitudes aligned | difference | aligned ≥ probabilities |
|---|---|---|---|---|---|---|---|---|---|---|---|
| hash | 16 | 3176 | 56.1% | 54.3% | 53.6% | miss by 4.5 | 3045 | 55.4% | 54.4% | -1.0 | miss by 1.0 |
| hash | 32 | 3176 | 53.7% | 52.5% | 52.1% | miss by 3.6 | 1485 | 51.9% | 52.0% | +0.1 | pass |
| hash | 64 | 3176 | 52.7% | 52.3% | 52.6% | miss by 2.1 | 889 | 52.0% | 51.9% | -0.1 | miss by 0.1 |
| given | 16 | 3176 | 56.5% | 54.4% | 56.5% | miss by 2.0 | 2957 | 56.6% | 56.8% | +0.2 | pass |
| given | 32 | 3176 | 56.5% | 54.4% | 56.5% | miss by 2.0 | 2957 | 56.6% | 56.8% | +0.2 | pass |
| given | 64 | 3176 | 56.5% | 54.4% | 56.5% | miss by 2.0 | 2957 | 56.6% | 56.8% | +0.2 | pass |
| learned | 16 | 3176 | 56.1% | 55.0% | 56.6% | miss by 1.5 | 3006 | 55.7% | 56.6% | +0.9 | pass |
| learned | 32 | 3176 | 57.1% | 52.9% | 56.4% | miss by 2.7 | 2777 | 54.9% | 54.9% | +0.0 | pass |
| learned | 64 | 3176 | 54.8% | 53.1% | 53.7% | miss by 3.1 | 2195 | 53.4% | 53.4% | +0.0 | pass |

## learned phases

Each kind's closing arrow turned by one of sixteen phases, kept when more training sentences peak at their own both-ends kind (no label read).

| arm | mode | training read as their own kind, phase 0 → learned | passes | kinds turned | a quarter turn or more | probabilities added | amplitudes aligned | learned phases | learned ≥ probabilities |
|---|---|---|---|---|---|---|---|---|---|
| hash | 16 | 10401 → 10516 of 14777 | 3 | 14 | 0 | 55.4% | 54.4% | **54.5%** | miss by 0.9 |
| hash | 32 | 8105 → 11095 of 14777 | 3 | 5 | 1 | 51.9% | 52.0% | **53.1%** | pass |
| hash | 64 | 10684 → 12722 of 14777 | 3 | 5 | 1 | 52.0% | 51.9% | **52.6%** | pass |
| given | 16 | 12725 → 12726 of 14777 | 2 | 1 | 0 | 56.6% | 56.8% | **56.8%** | pass |
| given | 32 | 12725 → 12726 of 14777 | 2 | 1 | 0 | 56.6% | 56.8% | **56.8%** | pass |
| given | 64 | 12725 → 12726 of 14777 | 2 | 1 | 0 | 56.6% | 56.8% | **56.8%** | pass |
| learned | 16 | 12803 → 12874 of 14777 | 4 | 17 | 1 | 55.7% | 56.6% | **56.7%** | pass |
| learned | 32 | 10779 → 12043 of 14777 | 3 | 15 | 1 | 54.9% | 54.9% | **56.1%** | pass |
| learned | 64 | 9005 → 11952 of 14777 | 4 | 15 | 2 | 53.4% | 53.4% | **54.1%** | pass |
