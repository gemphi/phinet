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
| hash | 16 | open | 256 | 6 | 3 | 51.7% | 64.9% | 0.043 | 22.2 |
| hash | 16 | close | 256 | 6 | 4 | 51.7% | 64.9% | 0.011 | 3.5 |
| hash | 16 | both | 2717 | 16 | 4 | 53.1% | 64.9% | 0.060 | 0.9 |
| hash | 16 | grown | 4664 | 18 | 5 | 52.6% | 64.9% | 0.080 | 1.5 |
| hash | 32 | open | 929 | 10 | 4 | 51.7% | 64.9% | 0.062 | 13.5 |
| hash | 32 | close | 998 | 10 | 4 | 52.1% | 64.9% | 0.030 | 3.5 |
| hash | 32 | both | 2945 | 16 | 4 | 52.4% | 64.9% | 0.061 | 0.5 |
| hash | 32 | grown | 4455 | 17 | 5 | 51.8% | 64.9% | 0.078 | 2.7 |
| hash | 64 | open | 1694 | 14 | 4 | 52.2% | 64.9% | 0.093 | 11.2 |
| hash | 64 | close | 2263 | 16 | 4 | 52.9% | 64.9% | 0.064 | 1.5 |
| hash | 64 | both | 2049 | 13 | 4 | 51.8% | 64.9% | 0.032 | 1.9 |
| hash | 64 | grown | 4252 | 19 | 5 | 53.3% | 64.9% | 0.126 | 6.9 |
| given | 16 | open | 89 | 6 | 3 | 53.1% | 64.9% | 0.088 | 62.7 |
| given | 16 | close | 86 | 6 | 3 | 51.8% | 64.9% | 0.035 | 20.1 |
| given | 16 | both | 452 | 8 | 4 | 51.8% | 65.2% | 0.071 | 23.4 |
| given | 16 | grown | 4127 | 16 | 4 | 53.2% | 64.9% | 0.085 | 5.6 |
| learned | 16 | open | 234 | 7 | 4 | 52.8% | 65.0% | 0.102 | 48.3 |
| learned | 16 | close | 231 | 7 | 4 | 52.5% | 64.9% | 0.025 | 8.5 |
| learned | 16 | both | 1239 | 10 | 4 | 52.4% | 64.9% | 0.037 | 5.5 |
| learned | 16 | grown | 4498 | 20 | 5 | 52.1% | 64.9% | 0.125 | 5.3 |
| learned | 32 | open | 597 | 10 | 4 | 52.3% | 64.9% | 0.126 | 35.0 |
| learned | 32 | close | 633 | 8 | 3 | 52.0% | 65.3% | 0.038 | 11.2 |
| learned | 32 | both | 2020 | 16 | 4 | 51.9% | 64.9% | 0.085 | 5.3 |
| learned | 32 | grown | 4507 | 20 | 5 | 53.1% | 64.9% | 0.126 | 5.4 |
| learned | 64 | open | 886 | 11 | 4 | 53.3% | 64.9% | 0.165 | 41.0 |
| learned | 64 | close | 1049 | 10 | 4 | 52.0% | 64.9% | 0.036 | 5.0 |
| learned | 64 | both | 2393 | 15 | 4 | 51.8% | 64.9% | 0.068 | 3.9 |
| learned | 64 | grown | 4489 | 19 | 5 | 52.9% | 64.9% | 0.119 | 6.1 |

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

## hash, mode 16, grown templates: what each label is sensed by

| label | kinds that sense it (chosen on dev) | held out | sensed | hit | F1 |
|---|---|---|---|---|---|
| function: explain | 18 | 1641 | 3176 | 1641 | 68.1% |
| function: instruct | 0 | 98 | 0 | 0 | 0.0% |
| function: answer | 0 | 1437 | 0 | 0 | 0.0% |
| mark: stop | 17 | 1128 | 1647 | 1074 | 77.4% |
| mark: question | 0 | 187 | 0 | 0 | 0.0% |
| mark: exclamation | 0 | 3 | 0 | 0 | 0.0% |
| mark: line | 1 | 421 | 92 | 26 | 10.1% |

family resemblance: 18 pieces over the kinds (shuffled kinds: 18.0, z 0.0); the widest shared feature holds 15.5% of a kind's variations (shuffled: 15.4%, z 0.7)
  kinds, depth 1: 1 splits kept, 6 passes, gains 1390 296 70 11 2 0
  kinds, depth 2: 2 splits kept, 6 passes, gains 1415 376 85 4 2 0
  kinds, depth 3: 4 splits kept, 7 passes, gains 1664 351 90 69 36 14 0
  kinds, depth 4: 8 splits kept, 8 passes, gains 1468 358 128 37 15 4 8 1
  kinds, depth 5: 2 splits kept, 10 passes, gains 1710 372 109 51 39 24 11 10 8 1
  kinds, depth 6: 0 splits kept, 9 passes, gains 302 37 17 6 2 1 2 1 0
  kinds: 18 at depth 5

## given, mode 16, grown templates: what each label is sensed by

| label | kinds that sense it (chosen on dev) | held out | sensed | hit | F1 |
|---|---|---|---|---|---|
| function: explain | 16 | 1641 | 3176 | 1641 | 68.1% |
| function: instruct | 0 | 98 | 0 | 0 | 0.0% |
| function: answer | 0 | 1437 | 0 | 0 | 0.0% |
| mark: stop | 16 | 1128 | 1739 | 1128 | 78.7% |
| mark: question | 0 | 187 | 0 | 0 | 0.0% |
| mark: exclamation | 0 | 3 | 0 | 0 | 0.0% |
| mark: line | 0 | 421 | 0 | 0 | 0.0% |

family resemblance: 17 pieces over the kinds (shuffled kinds: 16.2, z 2.1); the widest shared feature holds 32.5% of a kind's variations (shuffled: 32.6%, z -0.4)
  kinds, depth 1: 1 splits kept, 6 passes, gains 1048 234 55 8 2 0
  kinds, depth 2: 2 splits kept, 8 passes, gains 1318 341 115 59 14 13 3 0
  kinds, depth 3: 4 splits kept, 10 passes, gains 1398 220 24 5 8 5 7 10 2 0
  kinds, depth 4: 8 splits kept, 9 passes, gains 1436 282 83 32 22 7 3 2 0
  kinds, depth 5: 0 splits kept, 7 passes, gains 1559 318 98 35 22 11 0
  kinds: 16 at depth 4

## learned, mode 16, grown templates: what each label is sensed by

| label | kinds that sense it (chosen on dev) | held out | sensed | hit | F1 |
|---|---|---|---|---|---|
| function: explain | 20 | 1641 | 3176 | 1641 | 68.1% |
| function: instruct | 0 | 98 | 0 | 0 | 0.0% |
| function: answer | 0 | 1437 | 0 | 0 | 0.0% |
| mark: stop | 20 | 1128 | 1739 | 1128 | 78.7% |
| mark: question | 0 | 187 | 0 | 0 | 0.0% |
| mark: exclamation | 0 | 3 | 0 | 0 | 0.0% |
| mark: line | 0 | 421 | 0 | 0 | 0.0% |

family resemblance: 20 pieces over the kinds (shuffled kinds: 20.3, z -0.6); the widest shared feature holds 23.1% of a kind's variations (shuffled: 23.2%, z -0.7)
  kinds, depth 1: 1 splits kept, 7 passes, gains 1220 287 63 29 8 3 0
  kinds, depth 2: 2 splits kept, 7 passes, gains 1452 246 69 165 76 12 0
  kinds, depth 3: 4 splits kept, 10 passes, gains 1442 275 71 22 17 15 2 5 3 0
  kinds, depth 4: 8 splits kept, 8 passes, gains 1561 371 90 33 18 15 3 0
  kinds, depth 5: 4 splits kept, 14 passes, gains 1593 411 85 43 40 22 11 9 7 14 13 16 2 0
  kinds, depth 6: 0 splits kept, 9 passes, gains 589 108 26 11 13 3 2 2 1
  kinds: 20 at depth 5

## learned, mode 16, grown templates: the kinds

| kind | variations | widest shared | pieces | held out | explain · instruct · answer | stop · question · excl. · line | commonest variation | an example |
|---|---|---|---|---|---|---|---|---|
| 8 | 322 | 75 | 1 | 233 | 118 · 10 · 105 | 85 · 12 · 0 · 31 | `c8 c8 …` ×9 | b i am doing something i started doing it and i haven't |
| 17 | 306 | 75 | 1 | 225 | 119 · 5 · 101 | 75 · 16 · 0 · 33 | `c1 c13 c14 …` ×6 | or negative i'm not doing etc |
| 0 | 277 | 67 | 1 | 218 | 115 · 3 · 100 | 82 · 11 · 1 · 24 | `c5 c4 c2 …` ×6 | the cafe opens at 7 |
| 7 | 233 | 59 | 1 | 181 | 88 · 4 · 89 | 51 · 13 · 0 · 28 | `c5 c2 c0 …` ×8 | andrew started evening classes recently |
| 1 | 255 | 58 | 1 | 167 | 85 · 9 · 73 | 63 · 11 · 0 · 20 | `c14 c4 c11 c14 c4 c11` ×6 | i need to eat something soon |
| 6 | 234 | 50 | 1 | 164 | 86 · 4 · 74 | 54 · 12 · 0 · 24 | `c14 c13 …` ×7 | i suggest that you |
| 16 | 227 | 47 | 1 | 160 | 87 · 0 · 73 | 62 · 12 · 0 · 13 | `c5 c5 c10 c2` ×5 | we use the continuous for things happening at we use the simple |
| 2 | 226 | 51 | 1 | 157 | 84 · 4 · 69 | 63 · 2 · 0 · 23 | `c4 c2 c13 …` ×5 | the museum at 4 o'clock on 7 the panama canal the |
| 13 | 212 | 47 | 1 | 156 | 83 · 9 · 64 | 63 · 9 · 1 · 19 | `c8 c12 …` ×9 | put the verb into the correct form present continuous or present simple |
| 4 | 220 | 49 | 1 | 155 | 79 · 3 · 73 | 58 · 7 · 1 · 16 | `c12 c14 …` ×4 | he enjoy his course |
| 15 | 272 | 62 | 1 | 154 | 73 · 4 · 77 | 45 · 6 · 0 · 26 | `c0 c12 c6 c9 …` ×6 | a study this example situation |
| 14 | 217 | 53 | 1 | 150 | 83 · 5 · 62 | 49 · 11 · 0 · 28 | `c5 c2 c13 c7 …` ×5 | i think she works in a shop |
| 19 | 133 | 27 | 1 | 143 | 74 · 6 · 63 | 56 · 6 · 0 · 18 | `c15 c7 c9 …` ×7 | anna doesn't seem very happy right now |
| 12 | 219 | 55 | 1 | 138 | 69 · 3 · 66 | 48 · 7 · 0 · 17 | `c14 c4 c2 …` ×7 | what's all that noise |
| 11 | 203 | 53 | 1 | 135 | 70 · 2 · 63 | 49 · 9 · 0 · 14 | `c5 c2 c2 …` ×6 | i knew sarah was busy so i her |
| 10 | 186 | 41 | 1 | 133 | 70 · 3 · 60 | 45 · 12 · 0 · 16 | `c2 c4 c2 …` ×4 | but perhaps she isn't learning italian at the time of speaking |
| 18 | 176 | 42 | 1 | 130 | 69 · 1 · 60 | 46 · 7 · 0 · 17 | `c15 c13 …` ×6 | i don't go away very often |
| 3 | 202 | 45 | 1 | 128 | 54 · 10 · 64 | 33 · 10 · 0 · 21 | `c9 c3 …` ×13 | try to find a job |
| 9 | 180 | 38 | 1 | 126 | 70 · 10 · 46 | 57 · 6 · 0 · 17 | `c2 c4 c11 …` ×9 | 2 3 4 5 6 |
| 5 | 198 | 51 | 1 | 123 | 65 · 3 · 55 | 44 · 8 · 0 · 16 | `c0 c13 c1 …` ×3 | he is there now or on his way there |
