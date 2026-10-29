# the kinds of intention — crates/phiano/data/person/murphy.txt

19058 sentences: train 14977 · dev 881 · held out 3200 (every answer from the Key among them)

held out by function: explain 1685, instruct 78, answer 1437 · by mark: stop 1123, question 193, exclamation 6, line 1878

the word code: 2883 words coded, learned in 0.8s
  words, depth 1 (2 classes): 6 passes, gains 6055 634 260 8 11 0
  words, depth 2 (4 classes): 5 passes, gains 13908 852 96 35 6
  words, depth 3 (8 classes): 7 passes, gains 13733 4703 1234 72 508 56 2
  words, depth 4 (16 classes): 6 passes, gains 14368 4063 698 62 82 2
  words, depth 5 (32 classes): 5 passes, gains 14119 2524 468 52 9
  words, depth 6 (64 classes): 6 passes, gains 14048 938 297 33 35 0

the learned classes at mode 16, most told members first:

- c0 (226 words): go like say had going want need got went used said come wish just
- c1 (261 words): it me something there her now work somebody them which away him who us
- c2 (564 words): sentences car job past verb present day house same party other verbs hotel week
- c3 (503 words): time etc people here more much well things holiday late money years nice tired
- c4 (76 words): you not use don't what been didn't no can't could also couldn't never always
- c5 (338 words): a in of at so get all how where after before take correct into
- c6 (78 words): he b she if when they c d i'll put shall compare yes he's
- c7 (46 words): 1 2 3 4 5 6 7 8 9 complete 10 11 12 additional
- c8 (54 words): to that out about up home than off ago again tennis forms lots over
- c9 (181 words): is and was for on or are with as has were will would by
- c10 (23 words): but using because page united 38 speed consonant officer shocked flows boils crashed fallen
- c11 (15 words): pages appendix type under arrives amazing castle differences european interrupted junction today's wetter </s>
- c12 (80 words): the this my your an unit these his new units next each every our
- c13 (175 words): be very some long good any one last two few too many most working
- c14 (218 words): have do can see did know think lot should doing friend live usually tell
- c15 (45 words): i we i'm it's i've i'd why there's you're she's o'clock exercises you've let's

## a story in mode 16: structure only

- sarah is in her car
  - hash: c11 c10 c5 c5 c4
  - given: c0 c3 c6 c1 c0
  - learned: c1 c9 c5 c1 c2
- she is on her way to work
  - hash: c0 c10 c15 c5 c15 c11 c1
  - given: c1 c3 c6 c1 c0 c6 c0
  - learned: c6 c9 c9 c1 c2 c8 c1
- she's driving to work
  - hash: c12 c7 c11 c1
  - given: c2 c2 c6 c0
  - learned: c15 c1 c8 c1
- this means she is driving now at the time of speaking
  - hash: c14 c14 c0 c10 c7 c6 c3 c1 c13 c7 c0
  - given: c1 c2 c1 c3 c2 c5 c6 c8 c0 c6 c2
  - learned: c12 c5 c6 c9 c1 c1 c5 c12 c3 c5 c1
- am is are ing is the present continuous
  - hash: c12 c10 c4 c2 c10 c1 c11 c15
  - given: c3 c3 c3 c2 c3 c8 c0 c2
  - learned: c0 c9 c9 c1 c9 c12 c2 c3
- i am i'm driving
  - hash: c9 c12 c15 c7
  - given: c1 c3 c0 c2
  - learned: c15 c0 c15 c1

## every arm, mode and twin, on the held-out sentences

| arm | mode | twin | kinds | variations | function purity | mark purity | river MI (bits) | river z |
|---|---|---|---|---|---|---|---|---|
| hash | 16 | open | 50 | 256 | 57.8% | 60.5% | 0.744 | 16.1 |
| hash | 16 | close | 38 | 256 | 55.1% | 59.2% | 0.338 | 2.6 |
| hash | 16 | both | 64 | 2767 | 56.5% | 59.4% | 0.903 | 0.7 |
| hash | 32 | open | 64 | 942 | 59.8% | 61.0% | 0.995 | 13.0 |
| hash | 32 | close | 64 | 996 | 56.1% | 58.9% | 0.870 | 2.9 |
| hash | 32 | both | 64 | 3113 | 54.6% | 60.5% | 0.441 | 2.1 |
| hash | 64 | open | 64 | 1800 | 58.1% | 61.8% | 0.997 | 14.5 |
| hash | 64 | close | 64 | 2280 | 56.7% | 59.4% | 0.899 | 0.9 |
| hash | 64 | both | 64 | 1921 | 53.4% | 60.1% | 0.162 | 3.2 |
| given | 16 | open | 37 | 86 | 64.2% | 66.2% | 0.533 | 42.5 |
| given | 16 | close | 41 | 85 | 54.6% | 59.2% | 0.349 | 13.6 |
| given | 16 | both | 64 | 422 | 62.2% | 65.2% | 0.552 | 15.6 |
| learned | 16 | open | 61 | 202 | 64.5% | 71.1% | 0.802 | 28.5 |
| learned | 16 | close | 56 | 198 | 55.7% | 59.2% | 0.369 | 6.9 |
| learned | 16 | both | 64 | 1089 | 60.6% | 67.2% | 0.809 | 8.7 |
| learned | 32 | open | 64 | 498 | 63.7% | 70.5% | 0.930 | 24.5 |
| learned | 32 | close | 64 | 512 | 55.9% | 58.8% | 0.550 | 5.0 |
| learned | 32 | both | 64 | 1803 | 58.9% | 65.1% | 0.799 | 5.4 |
| learned | 64 | open | 64 | 943 | 64.9% | 70.7% | 1.068 | 26.2 |
| learned | 64 | close | 64 | 1005 | 55.2% | 58.9% | 0.638 | 3.5 |
| learned | 64 | both | 64 | 2135 | 57.8% | 65.0% | 0.626 | 6.0 |

the majority baselines on the held-out sentences: function 52.7%, mark 58.7%

## hash, mode 16, both ends: what each label is sensed by

| label | kinds that sense it (chosen on dev) | held out | sensed | hit | F1 |
|---|---|---|---|---|---|
| function: explain | 64 | 1685 | 3200 | 1685 | 69.0% |
| function: instruct | 0 | 78 | -0 | -0 | 0.0% |
| function: answer | 0 | 1437 | -0 | -0 | 0.0% |
| mark: stop | 56 | 1123 | 2908 | 1019 | 50.6% |
| mark: question | 0 | 193 | -0 | -0 | 0.0% |
| mark: exclamation | 0 | 6 | -0 | -0 | 0.0% |
| mark: line | 8 | 1878 | 292 | 167 | 15.4% |
  kinds, depth 1 (2 kinds): 5 passes, gains 890 169 30 5 0
  kinds, depth 2 (4 kinds): 7 passes, gains 945 192 71 21 17 16 1
  kinds, depth 3 (8 kinds): 6 passes, gains 910 152 48 34 3 0
  kinds, depth 4 (16 kinds): 7 passes, gains 930 205 64 20 9 4 0
  kinds, depth 5 (32 kinds): 12 passes, gains 1124 264 73 34 11 13 7 6 13 10 3 1
  kinds, depth 6 (64 kinds): 12 passes, gains 1627 250 92 66 29 44 36 12 8 2 2 0

## learned, mode 16, both ends: what each label is sensed by

| label | kinds that sense it (chosen on dev) | held out | sensed | hit | F1 |
|---|---|---|---|---|---|
| function: explain | 60 | 1685 | 3181 | 1673 | 68.8% |
| function: instruct | 1 | 78 | 6 | 0 | 0.0% |
| function: answer | 0 | 1437 | -0 | -0 | 0.0% |
| mark: stop | 40 | 1123 | 2693 | 988 | 51.8% |
| mark: question | 6 | 193 | 234 | 33 | 15.5% |
| mark: exclamation | 0 | 6 | -0 | -0 | 0.0% |
| mark: line | 15 | 1878 | 260 | 197 | 18.4% |
  kinds, depth 1 (2 kinds): 5 passes, gains 346 40 18 4 0
  kinds, depth 2 (4 kinds): 6 passes, gains 540 94 30 7 1 0
  kinds, depth 3 (8 kinds): 8 passes, gains 451 118 89 28 9 4 1 0
  kinds, depth 4 (16 kinds): 6 passes, gains 553 106 32 9 3 0
  kinds, depth 5 (32 kinds): 9 passes, gains 570 96 37 18 9 16 5 3 0
  kinds, depth 6 (64 kinds): 6 passes, gains 809 138 43 8 6 1

## learned, mode 16, both ends: the kinds

| kind | variations | widest shared | pieces | held out | explain · instruct · answer | stop · question · excl. · line | commonest variation | an example |
|---|---|---|---|---|---|---|---|---|
| 57 | 22 | 5 | 1 | 354 | 155 · 10 · 189 | 69 · 16 · 1 · 268 | `c3 c12 … c5` ×8 | but perhaps she isn't learning italian at the time of speaking |
| 56 | 38 | 9 | 1 | 140 | 55 · 0 · 85 | 25 · 6 · 0 · 109 | `c15 c0 … c1` ×27 | past now future past now future |
| 54 | 17 | 6 | 1 | 139 | 103 · 4 · 32 | 94 · 2 · 0 · 43 | `c7 c15 … c3` ×49 | 3 i have a car but i not use it much |
| 51 | 24 | 7 | 1 | 138 | 70 · 2 · 66 | 43 · 14 · 0 · 81 | `c15 c4 … c3` ×66 | correct them where necessary |
| 29 | 18 | 8 | 2 | 129 | 116 · 0 · 13 | 106 · 8 · 0 · 15 | `c7 c15 … c2` ×52 | 8 we're not in a hurry |
| 58 | 25 | 7 | 1 | 119 | 52 · 2 · 65 | 30 · 9 · 0 · 80 | `c12 c2 … c3` ×28 | you can say he's being you're being etc |
| 18 | 23 | 7 | 1 | 114 | 50 · 4 · 60 | 30 · 5 · 0 · 79 | `c12 c2 … c2` ×37 | so you can say |
| 59 | 30 | 8 | 1 | 102 | 45 · 5 · 52 | 26 · 3 · 0 · 73 | `c15 c5 … c2` ×10 | getting becoming changing improving |
| 21 | 30 | 7 | 1 | 97 | 46 · 0 · 51 | 31 · 7 · 0 · 59 | `c15 c4 … c1` ×45 | i shouldn't have said it |
| 20 | 28 | 6 | 1 | 92 | 32 · 6 · 54 | 31 · 2 · 0 · 59 | `c1 c9 … c2` ×45 | the room very clean |
| 16 | 15 | 4 | 1 | 88 | 43 · 2 · 43 | 35 · 3 · 0 · 50 | `c1 c9 … c3` ×26 | i always get hungry in the afternoon |
| 50 | 10 | 3 | 4 | 85 | 31 · 0 · 54 | 24 · 0 · 0 · 61 | `c15 c0 … c2` ×54 | i hope kate passes the driving test |
| 60 | 20 | 9 | 1 | 76 | 47 · 6 · 23 | 36 · 7 · 0 · 33 | `c6 c9 … c2` ×18 | what's all that noise |
| 49 | 18 | 8 | 1 | 73 | 32 · 1 · 40 | 24 · 0 · 0 · 49 | `c15 c0 … c3` ×49 | takes me about half an hour |
| 55 | 18 | 9 | 1 | 71 | 59 · 2 · 10 | 40 · 14 · 0 · 17 | `c7 c1 … c3` ×26 | 4 kate phoned last night |
| 53 | 15 | 9 | 1 | 70 | 54 · 4 · 12 | 51 · 2 · 0 · 17 | `c7 c12 … c3` ×27 | h the company is losing money |
| 52 | 23 | 12 | 1 | 66 | 50 · 1 · 15 | 33 · 12 · 0 · 21 | `c14 c4 … c13` ×8 | 3 she told me her name but i not remember it now |
| 63 | 33 | 9 | 1 | 65 | 34 · 1 · 30 | 16 · 1 · 1 · 47 | `c5 c8 … c3` ×8 | 1 2 3 4 5 6 |
| 23 | 24 | 6 | 1 | 64 | 24 · 3 · 37 | 18 · 1 · 0 · 45 | `c5 c12 … c2` ×25 | all the way completely |
| 17 | 25 | 7 | 1 | 62 | 30 · 1 · 31 | 21 · 5 · 0 · 36 | `c6 c4 … c3` ×11 | they still make films |
| 61 | 24 | 9 | 1 | 62 | 37 · 0 · 25 | 25 · 1 · 0 · 36 | `c6 c15 … c3` ×13 | c it's starting to rain |
| 28 | 28 | 9 | 1 | 59 | 43 · 4 · 12 | 40 · 3 · 0 · 16 | `c6 c0 … c2` ×10 | 10 the washing machine has been repaired |
| 4 | 15 | 3 | 2 | 55 | 24 · 0 · 31 | 18 · 2 · 0 · 35 | `c1 c9 … c1` ×19 | it isn't raining now |
| 19 | 21 | 6 | 1 | 54 | 23 · 1 · 30 | 20 · 0 · 0 · 34 | `c12 c2 … c1` ×18 | 2 some children at the zoo are looking at a giraffe |
| 22 | 27 | 8 | 2 | 54 | 26 · 3 · 25 | 9 · 6 · 0 · 39 | `c14 c4 … c2` ×11 | he doesn't have it now |
| 62 | 25 | 5 | 1 | 50 | 28 · 1 · 21 | 13 · 3 · 0 · 34 | `c15 c0 … c14` ×10 | buy catch cost fall hurt sell spend teach throw write |
| 24 | 20 | 6 | 1 | 49 | 34 · 1 · 14 | 25 · 6 · 1 · 17 | `c6 c15 … c1` ×14 | b i am doing something i started doing it and i haven't |
| 48 | 14 | 4 | 1 | 47 | 24 · 0 · 23 | 15 · 5 · 0 · 27 | `c15 c4 … c14` ×12 | i'm a very careful person |
| 25 | 21 | 6 | 1 | 46 | 34 · 0 · 12 | 15 · 15 · 0 · 16 | `c7 c5 … c2` ×14 | 2 a you listen to the radio |
| 5 | 25 | 5 | 1 | 45 | 24 · 1 · 20 | 15 · 3 · 0 · 27 | `c6 c4 … c2` ×10 | 7 last year we went to mexico |
| 31 | 17 | 6 | 1 | 44 | 32 · 2 · 10 | 21 · 9 · 1 · 13 | `c7 c4 … c2` ×15 | 5 is the meeting still going on or it finish |
| 30 | 25 | 5 | 1 | 34 | 19 · 5 · 10 | 18 · 2 · 0 · 14 | `c7 c12 … c5` ×9 | it's the last day of your holiday |
| 27 | 18 | 4 | 1 | 33 | 13 · 0 · 20 | 5 · 0 · 0 · 28 | `c4 c14 … c1` ×11 | jane no i haven't been to india |
| 26 | 17 | 5 | 1 | 30 | 7 · 0 · 23 | 3 · 2 · 0 · 25 | `c4 c14 … c14` ×10 | what do you do |
| 39 | 16 | 5 | 1 | 29 | 14 · 0 · 15 | 9 · 2 · 0 · 18 | `c1 c0 … c3` ×4 | i've drunk four cups of coffee today |
| 2 | 15 | 6 | 1 | 25 | 10 · 1 · 14 | 2 · 0 · 0 · 23 | `c5 c5 … c2` ×10 | a study this example situation |
| 0 | 18 | 4 | 2 | 23 | 9 · 1 · 13 | 3 · 0 · 0 · 20 | `c5 c0 … c0` ×4 | 76 if was were unit 39c if i had known unit 40 |
| 3 | 18 | 4 | 1 | 23 | 13 · 0 · 10 | 7 · 3 · 0 · 13 | `c1 c8 … c3` ×7 | what is your opinion |
| 7 | 16 | 4 | 1 | 23 | 7 · 0 · 16 | 4 · 0 · 0 · 19 | `c1 c4 … c4` ×6 | 1 you know that lisa plays tennis |
| 38 | 7 | 3 | 4 | 20 | 10 · 0 · 10 | 6 · 1 · 0 · 13 | `c15 c4 … c5` ×12 | we use will we'll invite to announce a |
| 1 | 13 | 4 | 3 | 19 | 9 · 2 · 8 | 6 · 2 · 0 · 11 | `c15 c9 … c0` ×6 | complete the sentences using will ll or be going to |
| 6 | 17 | 5 | 1 | 19 | 8 · 0 · 11 | 3 · 3 · 0 · 13 | `c5 c15 … c1` ×4 | what time do you finish work tomorrow |
| 37 | 13 | 6 | 3 | 19 | 9 · 0 · 10 | 2 · 0 · 0 · 17 | `c12 c3 … c2` ×4 | present perfect simple units 7 8 present perfect continuous unit 9 |
| 10 | 13 | 4 | 2 | 15 | 4 · 1 · 10 | 2 · 1 · 0 · 12 | `c14 c4 … c14` ×8 | want like know go etc |
| 33 | 7 | 2 | 4 | 14 | 14 · 0 · 0 | 10 · 2 · 2 · 0 | `c7 c4 … c14` ×7 | 10 what your father do |
| 47 | 7 | 2 | 2 | 13 | 5 · 0 · 8 | 5 · 0 · 0 · 8 | `c15 c12 … c2` ×5 | or he has never driven a car before |
| 32 | 11 | 3 | 3 | 12 | 6 · 0 · 6 | 4 · 0 · 0 · 8 | `c14 c4 … c9` ×4 | b very often the past simple ends in ed regular verbs |
| 36 | 15 | 4 | 1 | 12 | 8 · 0 · 4 | 3 · 0 · 0 · 9 | `c10 c15 … c1` ×2 | believe eat flow go grow make rise tell translate |
| 46 | 11 | 4 | 3 | 12 | 6 · 1 · 5 | 2 · 0 · 0 · 10 | `c2 c2 … c2` ×5 | find the sentences with a similar meaning |
| 40 | 9 | 3 | 3 | 11 | 6 · 0 · 5 | 6 · 0 · 0 · 5 | `c1 c3 … c2` ×2 | 1 she got up at 7 o'clock |
| 43 | 12 | 3 | 2 | 11 | 4 · 0 · 7 | 0 · 2 · 0 · 9 | `c0 c5 … c2` ×2 | do they have a car |
| 9 | 16 | 3 | 1 | 10 | 3 · 0 · 7 | 3 · 0 · 0 · 7 | `c14 c13 … c3` ×5 | a it depends what you want me to do |
| 35 | 7 | 2 | 3 | 10 | 5 · 0 · 5 | 3 · 0 · 0 · 7 | `c4 c0 … c14` ×4 | you want to know |
| 11 | 7 | 3 | 4 | 9 | 6 · 0 · 3 | 5 · 0 · 0 · 4 | `c5 c8 … c2` ×3 | she is waiting for you |
| 14 | 6 | 3 | 3 | 9 | 2 · 0 · 7 | 1 · 0 · 0 · 8 | `c14 c5 … c3` ×5 | have a look have lunch have a party have a nice time |
| 34 | 8 | 5 | 3 | 9 | 4 · 0 · 5 | 2 · 1 · 0 · 6 | `c8 c1 … c1` ×3 | that continues until now |
| 41 | 11 | 3 | 3 | 9 | 3 · 0 · 6 | 1 · 0 · 0 · 8 | `c9 c5 … c14` ×3 | 6 two people were to put out by smoke the fire overcome |
| 42 | 8 | 2 | 3 | 9 | 7 · 0 · 2 | 3 · 1 · 0 · 5 | `c14 c4 … c4` ×3 | not i wish it would have been |
| 15 | 13 | 3 | 1 | 7 | 1 · 0 · 6 | 0 · 1 · 0 · 6 | `c8 c1 … c2` ×4 | can i have some sugar please |
| 44 | 10 | 3 | 1 | 7 | 4 · 0 · 3 | 0 · 0 · 0 · 7 | `c9 c4 … c5` ×2 | the negative is i'd better not i had better not |
| 13 | 8 | 2 | 3 | 6 | 4 · 0 · 2 | 2 · 0 · 0 · 4 | `c1 c14 … c13` ×2 | we use it to say that something happens all the |
| 8 | 7 | 3 | 2 | 5 | 4 · 0 · 1 | 1 · 0 · 0 · 4 | `c3 c3 … c2` ×3 | patience people picture queue space umbrella |
| 12 | 8 | 2 | 4 | 3 | 3 · 0 · 0 | 2 · 0 · 0 · 1 | `c0 c8 … c9` ×1 | see saw we saw alice in town a few days ago |
| 45 | 9 | 2 | 5 | 1 | 1 · 0 · 0 | 1 · 0 · 0 · 0 | `c4 c2 … c2` ×1 | you phone for an appointment |
