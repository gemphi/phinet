# oracle transcript

- oracle: `ollama run phi4:14b --nowordwrap`
- checker: `phisor::truth v0.1.0`
- asks 6; claims parsed 90; claims refused 32; rows told 58

## ask 0 (held 0)

- prompt: `Word: cat. Offer: is-a, colour, part-of, has, likes, near, capital-of, bigger-than.`
- parsed 15; refused 6; told 9
  - told: is-a | mammal | 0.9
  - told: is-a | animal | 0.8
  - told: has | fur | 0.9
  - told: has | whiskers | 0.8
  - told: likes | milk | 0.6
  - told: worships | moon | 0.7
  - told: invented | fire | 0.5
  - told: fears | water | 0.6
  - told: dreams | mice | 0.4
  - refused (degenerate): is-a | cat | 0.9
  - refused (degenerate): colour | colour | 1
  - refused (degenerate): has | | 0.7
  - refused (contradicting): colour | green | 0.5
  - refused (contradicting): colour | blue | 0.4
  - refused (contradicting): capital-of | france | 0.3

## ask 1 (held 0)

- prompt: `Word: dog. Offer: is-a, colour, part-of, has, likes, near, capital-of, bigger-than.`
- parsed 14; refused 5; told 9
  - told: is-a | mammal | 0.9
  - told: is-a | pet | 0.8
  - told: has | tail | 0.9
  - told: has | paws | 0.8
  - told: likes | bones | 0.7
  - told: drives | car | 0.6
  - told: speaks | latin | 0.5
  - told: counts | stars | 0.4
  - told: paints | fences | 0.3
  - refused (degenerate): is-a | dog | 1
  - refused (degenerate): likes | | 0.5
  - refused (degenerate): has | has | 0.6
  - refused (contradicting): colour | purple | 0.3
  - refused (contradicting): capital-of | spain | 0.4

## ask 2 (held 0)

- prompt: `Word: paris. Offer: is-a, colour, part-of, has, capital-of, near, bigger-than.`
- parsed 13; refused 5; told 8
  - told: is-a | city | 0.9
  - told: capital-of | france | 0.9
  - told: has | tower | 0.8
  - told: near | seine | 0.7
  - told: founded | rome | 0.4
  - told: eats | bread | 0.3
  - told: whispers | secrets | 0.5
  - told: owns | clouds | 0.4
  - refused (degenerate): is-a | paris | 0.8
  - refused (degenerate): part-of | | 0.6
  - refused (contradicting): capital-of | germany | 0.4
  - refused (contradicting): capital-of | italy | 0.3
  - refused (contradicting): colour | yellow | 0.5

## ask 3 (held 0)

- prompt: `Word: sky. Offer: is-a, colour, part-of, has, near, bigger-than.`
- parsed 13; refused 5; told 8
  - told: is-a | expanse | 0.7
  - told: colour | blue | 0.8
  - told: has | clouds | 0.7
  - told: near | horizon | 0.6
  - told: remembers | rain | 0.5
  - told: owes | money | 0.4
  - told: judges | birds | 0.3
  - told: collects | kites | 0.4
  - refused (degenerate): is-a | sky | 0.9
  - refused (degenerate): colour | | 0.5
  - refused (contradicting): colour | green | 0.4
  - refused (contradicting): colour | orange | 0.3
  - refused (contradicting): colour | pink | 0.2

## ask 4 (held 0)

- prompt: `Word: iron. Offer: is-a, colour, part-of, has, melts-at, bigger-than.`
- parsed 13; refused 5; told 8
  - told: is-a | metal | 0.9
  - told: is-a | element | 0.8
  - told: colour | grey | 0.7
  - told: has | mass | 0.8
  - told: part-of | tool | 0.6
  - told: sings | opera | 0.5
  - told: forgives | rust | 0.3
  - told: teaches | smiths | 0.4
  - refused (degenerate): is-a | iron | 1
  - refused (degenerate): has | | 0.6
  - refused (degenerate): colour | colour | 0.5
  - refused (contradicting): colour | transparent | 0.3
  - refused (contradicting): melts-at | zero | 0.4

## ask 5 (held 0)

- prompt: `Word: whale. Offer: is-a, colour, part-of, has, lives-in, bigger-than.`
- parsed 22; refused 6; told 16
  - told: is-a | mammal | 0.9
  - told: is-a | animal | 0.8
  - told: lives-in | ocean | 0.9
  - told: has | fins | 0.8
  - told: bigger-than | shark | 0.7
  - told: recites | poetry | 0.5
  - told: hoards | gold | 0.4
  - told: lectures | plankton | 0.3
  - told: builds | cathedrals | 0.4
  - told: audits | tides | 0.3
  - told: brews | coffee | 0.5
  - told: knits | sweaters | 0.4
  - told: debates | gulls | 0.3
  - told: files | taxes | 0.4
  - told: sketches | reefs | 0.3
  - told: narrates | storms | 0.5
  - refused (degenerate): is-a | whale | 1
  - refused (degenerate): lives-in | | 0.5
  - refused (degenerate): has | has | 0.4
  - refused (contradicting): is-a | fish | 0.6
  - refused (contradicting): colour | scarlet | 0.3
  - refused (contradicting): bigger-than | ocean | 0.5
