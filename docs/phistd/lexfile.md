# lexfile: a word list in a file

The user's ruling of 2026-11-07: no linguistic data in code. Contractions, irregular verbs, closed-word lists, templates — every level of what is known lives in the file system, where a run can read it and append to it; the Rust keeps only the functions that find the abstractions. This is the one reader and appender every crate uses for it.

```mermaid
graph LR
    subgraph file["data/…/name.txt"]
        F["one entry per line<br/># a comment · a blank line ignored<br/>the whole line is the entry: 'no, ' ≠ 'no,'"]
    end
    read["read(path) → Vec of String<br/>a missing file is an empty list"]
    parse["parse(text)"]
    append["append(path, entry) → added?<br/>not doubled · creates the file and its folder"]
    F --> read --> parse
    append --> F
    subgraph readers["who reads and appends"]
        Phiton["phiton p64 syntax::vocab<br/>data/lex/{pronoun … verb}.txt · 9 classes<br/>PosDictionary::learn appends"]
        Phiano["phiano server::intent::greeting<br/>data/intent/{greetings … stop_words}.txt · 7 lists"]
    end
    Phiton --> read
    Phiton --> append
    Phiano --> read
```

## The test

`a_list_is_read_appended_and_not_doubled`: a missing file reads as empty; `parse` drops comments and blanks and keeps a trailing space; `append` adds once, refuses a repeat and a blank, and `read` gives the two back in order.
