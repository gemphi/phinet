# the kinds of intention — crates/phiano/data/person/murphy.txt

18822 sentences: train 14777 · dev 869 · held out 3176 (every answer from the Key among them)

held out by function: explain 1641, instruct 98, answer 1437 · by mark (the units' sentences; the Key has none): stop 1128, question 187, exclamation 3, line 421

the word code, grown as far as it pays: 2849 words coded, 53 classes at depth 7, learned in 0.7s
  words, depth 1: 1 splits kept, 4 passes, gains 8090 353 151 2
  words, depth 2: 2 splits kept, 4 passes, gains 15812 1346 241 2
  words, depth 3: 4 splits kept, 9 passes, gains 10807 5736 1078 157 93 300 349 22 3
  words, depth 4: 8 splits kept, 5 passes, gains 15587 644 87 31 12
  words, depth 5: 16 splits kept, 6 passes, gains 13411 1323 651 97 21 1
  words, depth 6: 18 splits kept, 7 passes, gains 9861 973 353 266 72 42 3
  words, depth 7: 3 splits kept, 8 passes, gains 7873 656 166 32 38 21 12 0
  words, depth 8: 0 splits kept, 6 passes, gains 455 22 6 21 1 0

the learned classes at mode 16, most told members first:

- c0 (155 words): not b what something as c somebody d i'll put which who also why
- c1 (305 words): be go me get work want them home him come used us live eat
- c2 (431 words): time etc now people here more day well things holiday money years ing late
- c3 (190 words): going one verb away verbs using perfect most happened enough looking able goes s
- c4 (72 words): the this my her your these two his each our three their another those
- c5 (59 words): a very i'm it's an some how too write you're really being 25 15
- c6 (45 words): i we he she they i've i'd additional you've we've you'll you'd they've we'd
- c7 (106 words): you it there sarah joe lisa she's paul compare tom kate he's amy jane
- c8 (66 words): can don't has didn't will would can't could should doesn't couldn't must never always
- c9 (219 words): was like say had been were know think got need went said isn't better
- c10 (248 words): long good much lot past present new few nice tired back hard old right
- c11 (416 words): sentences car job house same party other friend hotel correct phone sentence room end
- c12 (150 words): have do use if when see did complete take tell find leave does seen
- c13 (234 words): is or are that out up unit no all any where last doing before
- c14 (92 words): to in of for at on with about by from after units into without
- c15 (61 words): and but so than because ago appendix unless over clauses formal slowly asks under

## a story in mode 16: structure only

- sarah is in her car
  - hash: c11 c10 c5 c5 c4
  - given: c0 c3 c6 c1 c0
  - learned: c7 c13 c14 c4 c11
- she is on her way to work
  - hash: c0 c10 c15 c5 c15 c11 c1
  - given: c1 c3 c6 c1 c0 c6 c0
  - learned: c6 c13 c14 c4 c11 c14 c1
- she's driving to work
  - hash: c12 c7 c11 c1
  - given: c2 c2 c6 c0
  - learned: c7 c10 c14 c1
- this means she is driving now at the time of speaking
  - hash: c14 c14 c0 c10 c7 c6 c3 c1 c13 c7 c0
  - given: c1 c2 c1 c3 c2 c5 c6 c8 c0 c6 c2
  - learned: c4 c13 c6 c13 c10 c2 c14 c4 c2 c14 c2
- am is are ing is the present continuous
  - hash: c12 c10 c4 c2 c10 c1 c11 c15
  - given: c3 c3 c3 c2 c3 c8 c0 c2
  - learned: c9 c13 c13 c2 c13 c4 c10 c2
- i am i'm driving
  - hash: c9 c12 c15 c7
  - given: c1 c3 c0 c2
  - learned: c6 c9 c5 c10

## every arm, mode and template, kinds grown as far as they pay, on the held-out sentences

| arm | mode | template | templates | kinds | depth | function purity | mark purity | river MI (bits) | river z |
|---|---|---|---|---|---|---|---|---|---|
| hash · river | 16 | open | 256 | 6 | 3 | 51.7% | 64.9% | 0.043 | 22.2 |
| hash · inside | 16 | open | 256 | 51 | 7 | 56.3% | 67.3% | 0.826 | 17.9 |
| hash · river | 16 | close | 256 | 6 | 4 | 51.7% | 64.9% | 0.011 | 3.5 |
| hash · inside | 16 | close | 256 | 53 | 7 | 55.9% | 65.7% | 0.682 | 1.6 |
| hash · river | 16 | both | 2717 | 16 | 4 | 53.1% | 64.9% | 0.060 | 0.9 |
| hash · inside | 16 | both | 2717 | 45 | 6 | 53.6% | 65.5% | 0.551 | 3.8 |
| hash · river | 16 | grown | 4664 | 18 | 5 | 52.6% | 64.9% | 0.080 | 1.5 |
| hash · inside | 16 | grown | 4664 | 37 | 6 | 54.5% | 66.1% | 0.469 | 10.3 |
| hash · river | 32 | open | 929 | 10 | 4 | 51.7% | 64.9% | 0.062 | 13.5 |
| hash · inside | 32 | open | 929 | 53 | 7 | 57.0% | 65.6% | 0.886 | 17.0 |
| hash · river | 32 | close | 998 | 10 | 4 | 52.1% | 64.9% | 0.030 | 3.5 |
| hash · inside | 32 | close | 998 | 55 | 6 | 55.3% | 65.1% | 0.777 | 2.0 |
| hash · river | 32 | both | 2945 | 16 | 4 | 52.4% | 64.9% | 0.061 | 0.5 |
| hash · inside | 32 | both | 2945 | 32 | 5 | 52.1% | 64.9% | 0.230 | 2.4 |
| hash · river | 32 | grown | 4455 | 17 | 5 | 51.8% | 64.9% | 0.078 | 2.7 |
| hash · inside | 32 | grown | 4455 | 42 | 6 | 55.8% | 65.6% | 0.564 | 9.5 |
| hash · river | 64 | open | 1694 | 14 | 4 | 52.2% | 64.9% | 0.093 | 11.2 |
| hash · inside | 64 | open | 1694 | 57 | 7 | 54.8% | 67.9% | 0.965 | 15.1 |
| hash · river | 64 | close | 2263 | 16 | 4 | 52.9% | 64.9% | 0.064 | 1.5 |
| hash · inside | 64 | close | 2263 | 58 | 6 | 54.9% | 65.1% | 0.834 | 1.9 |
| hash · river | 64 | both | 2049 | 13 | 4 | 51.8% | 64.9% | 0.032 | 1.9 |
| hash · inside | 64 | both | 2049 | 32 | 5 | 52.6% | 65.7% | 0.148 | 3.7 |
| hash · river | 64 | grown | 4252 | 19 | 5 | 53.3% | 64.9% | 0.126 | 6.9 |
| hash · inside | 64 | grown | 4252 | 49 | 6 | 55.0% | 66.8% | 0.745 | 11.2 |
| given · river | 16 | open | 89 | 6 | 3 | 53.1% | 64.9% | 0.088 | 62.7 |
| given · inside | 16 | open | 89 | 32 | 6 | 56.9% | 70.6% | 0.530 | 36.3 |
| given · river | 16 | close | 86 | 6 | 3 | 51.8% | 64.9% | 0.035 | 20.1 |
| given · inside | 16 | close | 86 | 29 | 7 | 54.2% | 65.7% | 0.274 | 13.6 |
| given · river | 16 | both | 452 | 8 | 4 | 51.8% | 65.2% | 0.071 | 23.4 |
| given · inside | 16 | both | 452 | 44 | 8 | 56.5% | 69.2% | 0.676 | 20.4 |
| given · river | 16 | grown | 4127 | 16 | 4 | 53.2% | 64.9% | 0.085 | 5.6 |
| given · inside | 16 | grown | 4127 | 38 | 7 | 55.5% | 69.5% | 0.479 | 9.5 |
| learned · river | 16 | open | 234 | 7 | 4 | 52.8% | 65.0% | 0.102 | 48.3 |
| learned · inside | 16 | open | 234 | 45 | 7 | 58.0% | 66.8% | 0.749 | 28.9 |
| learned · river | 16 | close | 231 | 7 | 4 | 52.5% | 64.9% | 0.025 | 8.5 |
| learned · inside | 16 | close | 231 | 42 | 6 | 55.2% | 65.4% | 0.434 | 7.1 |
| learned · river | 16 | both | 1239 | 10 | 4 | 52.4% | 64.9% | 0.037 | 5.5 |
| learned · inside | 16 | both | 1239 | 48 | 7 | 56.6% | 65.7% | 0.702 | 11.9 |
| learned · river | 16 | grown | 4498 | 20 | 5 | 52.1% | 64.9% | 0.125 | 5.3 |
| learned · inside | 16 | grown | 4498 | 40 | 6 | 55.1% | 65.3% | 0.592 | 13.8 |
| learned · river | 32 | open | 597 | 10 | 4 | 52.3% | 64.9% | 0.126 | 35.0 |
| learned · inside | 32 | open | 597 | 50 | 7 | 57.2% | 68.3% | 0.888 | 28.0 |
| learned · river | 32 | close | 633 | 8 | 3 | 52.0% | 65.3% | 0.038 | 11.2 |
| learned · inside | 32 | close | 633 | 52 | 6 | 55.8% | 65.3% | 0.706 | 6.1 |
| learned · river | 32 | both | 2020 | 16 | 4 | 51.9% | 64.9% | 0.085 | 5.3 |
| learned · inside | 32 | both | 2020 | 50 | 7 | 56.4% | 65.8% | 0.684 | 12.7 |
| learned · river | 32 | grown | 4507 | 20 | 5 | 53.1% | 64.9% | 0.126 | 5.4 |
| learned · inside | 32 | grown | 4507 | 43 | 6 | 56.4% | 67.6% | 0.667 | 16.1 |
| learned · river | 64 | open | 886 | 11 | 4 | 53.3% | 64.9% | 0.165 | 41.0 |
| learned · inside | 64 | open | 886 | 54 | 7 | 58.4% | 68.2% | 1.020 | 26.8 |
| learned · river | 64 | close | 1049 | 10 | 4 | 52.0% | 64.9% | 0.036 | 5.0 |
| learned · inside | 64 | close | 1049 | 52 | 6 | 55.2% | 65.6% | 0.737 | 5.7 |
| learned · river | 64 | both | 2393 | 15 | 4 | 51.8% | 64.9% | 0.068 | 3.9 |
| learned · inside | 64 | both | 2393 | 48 | 6 | 53.7% | 66.3% | 0.499 | 6.2 |
| learned · river | 64 | grown | 4489 | 19 | 5 | 52.9% | 64.9% | 0.119 | 6.1 |
| learned · inside | 64 | grown | 4489 | 50 | 6 | 55.4% | 68.0% | 0.876 | 16.0 |

the majority baselines on the held-out sentences: function 51.7%, mark 64.9%

## the bridge to the first slice: 64 kinds fixed against kinds that pay, mode 16

| arm | mode | template | templates | kinds | depth | function purity | mark purity | river MI (bits) | river z |
|---|---|---|---|---|---|---|---|---|---|
| hash | 16 | open | 256 | 37 | 6 | 55.0% | 65.9% | 0.425 | 13.9 |
| hash | 16 | grown | 4664 | 64 | 6 | 55.3% | 64.9% | 1.030 | 2.2 |
| given | 16 | open | 89 | 37 | 6 | 56.7% | 70.4% | 0.533 | 34.2 |
| given | 16 | grown | 4127 | 64 | 6 | 56.2% | 65.0% | 0.995 | 5.3 |
| learned | 16 | open | 234 | 55 | 6 | 56.9% | 66.9% | 0.544 | 25.9 |
| learned | 16 | grown | 4498 | 64 | 6 | 55.1% | 64.9% | 1.067 | 4.7 |

## hash, mode 16, the opening, read inside: what each label is sensed by

| label | kinds that sense it (chosen on dev) | held out | sensed | hit | F1 |
|---|---|---|---|---|---|
| function: explain | 50 | 1641 | 3167 | 1637 | 68.1% |
| function: instruct | 1 | 98 | 9 | 2 | 3.7% |
| function: answer | 0 | 1437 | 0 | 0 | 0.0% |
| mark: stop | 41 | 1128 | 1467 | 1019 | 78.5% |
| mark: question | 4 | 187 | 109 | 53 | 35.8% |
| mark: exclamation | 0 | 3 | 0 | 0 | 0.0% |
| mark: line | 6 | 421 | 163 | 75 | 25.7% |

family resemblance: 57 pieces over the kinds (shuffled kinds: 158.6, z -18.3); the widest shared feature holds 76.7% of a kind's variations (shuffled: 37.6%, z 22.7)
  kinds, depth 1: 1 splits kept, 6 passes, gains 2400 703 219 66 11 0
  kinds, depth 2: 2 splits kept, 9 passes, gains 2681 1157 521 139 66 60 19 64 0
  kinds, depth 3: 4 splits kept, 7 passes, gains 4498 1505 541 131 37 110 0
  kinds, depth 4: 8 splits kept, 7 passes, gains 7217 2021 855 1315 100 144 2
  kinds, depth 5: 16 splits kept, 5 passes, gains 9543 1842 625 564 0
  kinds, depth 6: 18 splits kept, 5 passes, gains 8159 1137 343 17 0
  kinds, depth 7: 1 splits kept, 5 passes, gains 3374 512 368 96 2
  kinds, depth 8: 0 splits kept, 1 passes, gains 0
  kinds: 51 at depth 7

## given, mode 16, the opening, read inside: what each label is sensed by

| label | kinds that sense it (chosen on dev) | held out | sensed | hit | F1 |
|---|---|---|---|---|---|
| function: explain | 31 | 1641 | 3164 | 1640 | 68.3% |
| function: instruct | 0 | 98 | 0 | 0 | 0.0% |
| function: answer | 0 | 1437 | 0 | 0 | 0.0% |
| mark: stop | 25 | 1128 | 1555 | 1086 | 81.0% |
| mark: question | 2 | 187 | 91 | 74 | 53.2% |
| mark: exclamation | 0 | 3 | 0 | 0 | 0.0% |
| mark: line | 4 | 421 | 92 | 43 | 16.8% |

family resemblance: 31 pieces over the kinds (shuffled kinds: 57.6, z -7.6); the widest shared feature holds 82.9% of a kind's variations (shuffled: 50.3%, z 9.4)
  kinds, depth 1: 1 splits kept, 4 passes, gains 3193 737 83 0
  kinds, depth 2: 2 splits kept, 3 passes, gains 3990 339 0
  kinds, depth 3: 4 splits kept, 4 passes, gains 7649 593 44 0
  kinds, depth 4: 8 splits kept, 5 passes, gains 6879 933 32 171 0
  kinds, depth 5: 14 splits kept, 4 passes, gains 7713 1435 12 0
  kinds, depth 6: 5 splits kept, 4 passes, gains 2842 27 33 0
  kinds, depth 7: 0 splits kept, 4 passes, gains 561 172 17 0
  kinds: 35 at depth 6

## learned, mode 16, the opening, read inside: what each label is sensed by

| label | kinds that sense it (chosen on dev) | held out | sensed | hit | F1 |
|---|---|---|---|---|---|
| function: explain | 42 | 1641 | 3107 | 1622 | 68.3% |
| function: instruct | 2 | 98 | 67 | 28 | 33.9% |
| function: answer | 0 | 1437 | 0 | 0 | 0.0% |
| mark: stop | 32 | 1128 | 1425 | 980 | 76.8% |
| mark: question | 5 | 187 | 144 | 57 | 34.4% |
| mark: exclamation | 0 | 3 | 0 | 0 | 0.0% |
| mark: line | 7 | 421 | 169 | 59 | 20.0% |

family resemblance: 51 pieces over the kinds (shuffled kinds: 129.1, z -14.5); the widest shared feature holds 77.1% of a kind's variations (shuffled: 40.2%, z 19.0)
  kinds, depth 1: 1 splits kept, 6 passes, gains 5012 1389 489 95 7 0
  kinds, depth 2: 2 splits kept, 8 passes, gains 5680 1405 169 60 26 472 53 0
  kinds, depth 3: 4 splits kept, 8 passes, gains 4296 1630 414 522 96 46 195 2
  kinds, depth 4: 8 splits kept, 4 passes, gains 9945 2017 99 0
  kinds, depth 5: 15 splits kept, 4 passes, gains 8358 1552 124 0
  kinds, depth 6: 14 splits kept, 5 passes, gains 6401 620 72 604 3
  kinds, depth 7: 2 splits kept, 3 passes, gains 4071 115 3
  kinds, depth 8: 0 splits kept, 3 passes, gains 270 38 0
  kinds: 47 at depth 7

## learned, mode 16, the opening, read inside: the kinds

| kind | variations | widest shared | pieces | held out | explain · instruct · answer | stop · question · excl. · line | commonest variation | an example |
|---|---|---|---|---|---|---|---|---|
| 6 | 2 | 2 | 1 | 300 | 137 · 2 · 161 | 114 · 4 · 0 · 21 | `c6 c8 …` ×299 | i don't go away very often |
| 1 | 2 | 2 | 1 | 287 | 162 · 4 · 121 | 134 · 0 · 0 · 32 | `c6 c9 …` ×286 | i need to eat something soon |
| 3 | 4 | 4 | 1 | 149 | 69 · 0 · 80 | 55 · 0 · 0 · 14 | `c4 c11 …` ×144 | the cafe opens at 7 |
| 12 | 3 | 2 | 2 | 136 | 72 · 1 · 63 | 63 · 2 · 0 · 8 | `c7 c9 …` ×131 | it stopped raining for a while but now it's raining again |
| 15 | 6 | 3 | 2 | 127 | 50 · 1 · 76 | 32 · 7 · 1 · 11 | `c5 c10 …` ×68 | i'm living with some friends until i find a my parents live |
| 20 | 4 | 3 | 2 | 125 | 81 · 2 · 42 | 43 · 12 · 0 · 28 | `c0 c13 …` ×67 | b i am doing something i started doing it and i haven't |
| 40 | 6 | 5 | 1 | 124 | 77 · 2 · 45 | 45 · 18 · 1 · 15 | `c5 c3 …` ×38 | a study this example situation |
| 17 | 10 | 7 | 2 | 118 | 43 · 2 · 73 | 28 · 1 · 0 · 16 | `c14 c4 …` ×62 | in the same way we say |
| 4 | 7 | 7 | 1 | 111 | 49 · 3 · 59 | 45 · 0 · 1 · 6 | `c4 c2 …` ×81 | this is the second time |
| 39 | 21 | 6 | 2 | 111 | 65 · 2 · 44 | 27 · 18 · 0 · 22 | `c13 c7 …` ×28 | tell me about jack |
| 37 | 2 | 2 | 1 | 110 | 73 · 2 · 35 | 59 · 5 · 0 · 11 | `c7 c8 …` ×109 | it doesn't rain very much in summer |
| 46 | 2 | 2 | 1 | 107 | 55 · 3 · 49 | 14 · 35 · 0 · 9 | `c12 c7 …` ×99 | did you go out |
| 14 | 23 | 8 | 1 | 89 | 51 · 0 · 38 | 31 · 4 · 0 · 16 | `c7 c1 …` ×21 | 9 the river amazon |
| 18 | 10 | 10 | 1 | 89 | 34 · 5 · 50 | 18 · 8 · 0 · 13 | `c13 c5 …` ×19 | getting becoming changing improving |
| 24 | 3 | 3 | 1 | 88 | 49 · 2 · 37 | 20 · 15 · 0 · 16 | `c0 c12 …` ×58 | c see hear smell taste look feel |
| 44 | 9 | 4 | 1 | 85 | 42 · 12 · 31 | 28 · 8 · 0 · 18 | `c0 c14 …` ×28 | what's all that noise |
| 36 | 11 | 10 | 1 | 81 | 28 · 0 · 53 | 16 · 10 · 0 · 2 | `c8 c7 …` ×33 | don't you have anything else to do |
| 19 | 11 | 11 | 1 | 77 | 39 · 1 · 37 | 23 · 0 · 0 · 17 | `c2 c14 …` ×14 | andrew started evening classes recently |
| 29 | 3 | 2 | 2 | 76 | 64 · 1 · 11 | 40 · 0 · 0 · 25 | `c6 c12 …` ×72 | we use the continuous for things happening at we use the simple |
| 45 | 10 | 7 | 1 | 73 | 29 · 0 · 44 | 14 · 3 · 0 · 12 | `c7 c12 …` ×19 | but perhaps she isn't learning italian at the time of speaking |
| 38 | 11 | 10 | 2 | 64 | 37 · 1 · 26 | 24 · 3 · 0 · 11 | `c1 c14 …` ×29 | drive s work s do es etc |
| 10 | 3 | 3 | 1 | 57 | 26 · 0 · 31 | 17 · 7 · 0 · 2 | `c5 c5 …` ×27 | a i'm not very active |
| 42 | 16 | 10 | 1 | 55 | 26 · 0 · 29 | 10 · 1 · 0 · 15 | `c14 c13 …` ×15 | after i hope we generally use the present |
| 32 | 2 | 2 | 1 | 50 | 13 · 26 · 11 | 26 · 6 · 0 · 7 | `c12 c4 …` ×48 | use the present continuous |
| 23 | 4 | 3 | 1 | 49 | 16 · 11 · 22 | 14 · 2 · 0 · 11 | `c0 c4 …` ×23 | put the verb into the correct form present continuous or present simple |
| 13 | 2 | 1 | 2 | 46 | 36 · 1 · 9 | 22 · 4 · 0 · 11 | `c7 c13 …` ×44 | sally is still here |
| 2 | 1 | 1 | 1 | 44 | 35 · 0 · 9 | 32 · 0 · 0 · 3 | `c6 c1 …` ×44 | i get up at 8 o'clock every morning |
| 35 | 1 | 1 | 1 | 41 | 21 · 0 · 20 | 13 · 2 · 0 · 6 | `c12 c6 …` ×41 | when i arrive not when i will arrive |
| 16 | 2 | 1 | 2 | 35 | 13 · 6 · 16 | 16 · 1 · 0 · 2 | `c5 c11 …` ×27 | a friend of yours is saving money to go on a world |
| 5 | 5 | 5 | 1 | 32 | 25 · 0 · 7 | 20 · 0 · 0 · 5 | `c6 c13 …` ×16 | she a big breakfast |
| 43 | 3 | 2 | 2 | 31 | 17 · 1 · 13 | 7 · 1 · 0 · 10 | `c5 c14 …` ×17 | it's been raining all morning |
| 11 | 2 | 1 | 2 | 28 | 11 · 0 · 17 | 10 · 1 · 0 · 0 | `c5 c4 …` ×24 | yesterday karen and joe played tennis |
| 7 | 1 | 1 | 1 | 26 | 14 · 1 · 11 | 12 · 0 · 0 · 3 | `c13 c6 …` ×26 | no i've just had lunch |
| 41 | 5 | 2 | 1 | 23 | 11 · 0 · 12 | 6 · 0 · 0 · 5 | `c13 c0 …` ×7 | from when i arrived until now |
| 26 | 1 | 1 | 1 | 21 | 13 · 0 · 8 | 12 · 0 · 0 · 1 | `c6 c0 …` ×21 | he not do any sport |
| 0 | 6 | 5 | 1 | 20 | 11 · 0 · 9 | 10 · 0 · 0 · 1 | `c15 c6 …` ×8 | i we they you have i've etc |
| 31 | 5 | 4 | 1 | 17 | 5 · 2 · 10 | 1 · 3 · 0 · 3 | `c8 c8 …` ×9 | when has helen got to go |
| 8 | 1 | 1 | 1 | 14 | 8 · 0 · 6 | 6 · 0 · 0 · 2 | `c5 c6 …` ×14 | a we use can to say that something is possible or allowed |
| 22 | 1 | 1 | 1 | 14 | 11 · 0 · 3 | 4 · 2 · 0 · 5 | `c0 c0 …` ×14 | 2 3 4 5 6 |
| 27 | 2 | 2 | 1 | 14 | 6 · 2 · 6 | 6 · 0 · 0 · 2 | `c12 c0 …` ×8 | if somebody says tom lost his key |
| 21 | 4 | 2 | 2 | 11 | 3 · 2 · 6 | 3 · 1 · 0 · 1 | `c0 c3 …` ×6 | who's going to drive you or me |
| 28 | 2 | 2 | 1 | 8 | 5 · 0 · 3 | 4 · 0 · 0 · 1 | `c12 c12 …` ×6 | shut shut it was cold so i shut the window |
| 33 | 2 | 2 | 1 | 6 | 3 · 0 · 3 | 2 · 0 · 0 · 1 | `c11 c3 …` ×3 | age size colour etc |
| 30 | 1 | 1 | 1 | 5 | 5 · 0 · 0 | 2 · 3 · 0 · 0 | `c8 c6 …` ×5 | can we stop walking soon |
| 34 | 2 | 1 | 2 | 2 | 1 · 0 · 1 | 0 · 0 · 0 · 1 | `c12 c10 …` ×1 | pay back money pay somebody back |
