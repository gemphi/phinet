//! The grammar of the frozen file: a row is nine tab-separated cells, and
//! the roles cell is a filler grammar that nests, because a role's filler
//! is sometimes itself a frame.
//!
//! This is a tabular reader — one cell per Frame field — and not the act
//! text form `ASK(wait(agent: you, in: rain), past, progressive)`, which
//! `ir::show` writes and `ir::text` reads. The two are separate jobs: this
//! one reads a frozen input, that one is the round trip R10.10 judges.

use super::{Entry, Flaw};
use crate::ir::{Act, Aspect, Attr, Det, Filler, Force, Frame, Modal, Polarity, Role, Tense, Thing, Voice, Wh};

/// The frozen file's seven frame columns, in the file's own order: a clause
/// writes the same seven between its brackets.
type Cell<'a> = [&'a str; 7];

/// Every row of the file, in file order, ids from 1. A blank line and a
/// line opening `#` are the header and the notes, not frames.
pub(super) fn all(text: &str) -> Result<Vec<Entry>, Flaw> {
    let mut out = Vec::new();
    for line in text.lines().map(str::trim_end) {
        match line.is_empty() || line.starts_with('#') {
            true => continue,
            false => out.push(row(out.len() + 1, line)?),
        }
    }
    Ok(out)
}

fn row(id: usize, line: &str) -> Result<Entry, Flaw> {
    let read: Vec<&str> = line.split('\t').map(str::trim).collect();
    let column: [&str; 9] = read
        .try_into()
        .map_err(|r: Vec<&str>| why(id, "row", line, format!("holds {} columns, not 9", r.len())))?;
    let [sentence, force, pred, roles, tense, aspect, modal, voice, polarity] = column;
    let force = Force::named(force)
        .ok_or_else(|| why(id, "force", force, "is not one of the seven".to_string()))?;
    let frame = frame(id, [pred, roles, tense, aspect, modal, voice, polarity])?;
    match sentence.is_empty() {
        true => Err(why(id, "sentence", sentence, "is empty".to_string())),
        false => Ok(Entry { id, sentence: sentence.to_string(), act: Act::new(force, frame) }),
    }
}

/// The seven frame cells as a Frame: the predicate, its slots, and what the
/// verb group says — tense, aspect, modality, voice, polarity.
fn frame(id: usize, cell: Cell<'_>) -> Result<Frame, Flaw> {
    let [pred, roles, tense, aspect, modal, voice, polarity] = cell;
    if pred.is_empty() {
        return Err(why(id, "pred", pred, "is empty".to_string()));
    }
    let tense = Tense::named(tense).ok_or_else(|| named(id, "tense", tense))?;
    let mut f = Frame::new(pred, tense);
    f.slots = self::roles(id, roles)?;
    f.aspect = Aspect::named(aspect).ok_or_else(|| named(id, "aspect", aspect))?;
    f.modal = match modal {
        "-" => None,
        m => Some(Modal::named(m).ok_or_else(|| named(id, "modal", m))?),
    };
    f.voice = Voice::named(voice).ok_or_else(|| named(id, "voice", voice))?;
    f.polarity = Polarity::named(polarity).ok_or_else(|| named(id, "polarity", polarity))?;
    Ok(f)
}

/// `role=filler` joined by `|`, or `-` for a frame with no role.
fn roles(id: usize, cell: &str) -> Result<Vec<(Role, Filler)>, Flaw> {
    match cell {
        "-" => Ok(Vec::new()),
        _ => split(cell, '|').into_iter().map(|p| role(id, p.trim())).collect(),
    }
}

fn role(id: usize, text: &str) -> Result<(Role, Filler), Flaw> {
    let (name, rest) =
        cut(text, '=').ok_or_else(|| why(id, "roles", text, "is not `role=filler`".to_string()))?;
    Ok((Role::named(name.trim()), filler(id, rest.trim())?))
}

/// A filler by its opening character: `*` a name, `^` a pronoun, `'` a bare
/// word, `?` a hole, `[` a clause, anything else a thing.
fn filler(id: usize, text: &str) -> Result<Filler, Flaw> {
    match text.chars().next() {
        Some('*') => Ok(Filler::name(&text[1..])),
        Some('^') => Ok(Filler::pronoun(&text[1..])),
        Some('\'') => Ok(Filler::word(&text[1..])),
        Some('?') => Wh::named(&text[1..])
            .map(Filler::Hole)
            .ok_or_else(|| why(id, "roles", text, "is not a declared hole".to_string())),
        Some('[') => clause(id, text),
        Some(_) => thing(id, text).map(Filler::Thing),
        None => Err(why(id, "roles", text, "is an empty filler".to_string())),
    }
}

/// A frame as an argument: the seven frame fields between brackets, in the
/// columns' own order, nesting as deep as the data goes.
fn clause(id: usize, text: &str) -> Result<Filler, Flaw> {
    let inner = text
        .strip_prefix('[')
        .and_then(|t| t.strip_suffix(']'))
        .ok_or_else(|| why(id, "roles", text, "is an unclosed clause".to_string()))?;
    let read: Vec<&str> = split(inner, ';').into_iter().map(str::trim).collect();
    let cell: Cell<'_> = read.try_into().map_err(|r: Vec<&str>| {
        why(id, "roles", text, format!("holds {} clause fields, not 7", r.len()))
    })?;
    frame(id, cell).map(Filler::clause)
}

/// A noun phrase: an optional determiner, the head, `·pl` for a plural, and
/// `attr=filler` attributes in braces.
fn thing(id: usize, text: &str) -> Result<Thing, Flaw> {
    let (head, attrs) = match (text.find('{'), text.ends_with('}')) {
        (Some(i), true) => (text[..i].trim(), Some(&text[i + 1..text.len() - 1])),
        (None, false) => (text, None),
        _ => return Err(why(id, "roles", text, "has unbalanced attributes".to_string())),
    };
    let (det, head) = match head.split_once(' ') {
        Some((d, h)) => (
            Some(det(d).ok_or_else(|| why(id, "roles", text, format!("opens on {d:?}, which is not a determiner")))?),
            h.trim(),
        ),
        None => (None, head),
    };
    let (head, plural) = match head.strip_suffix("·pl") {
        Some(h) => (h, true),
        None => (head, false),
    };
    if head.is_empty() {
        return Err(why(id, "roles", text, "has no head".to_string()));
    }
    let mut t = Thing::new(head);
    t.det = det;
    t.plural = plural;
    for piece in attrs.map(|a| split(a, ',')).unwrap_or_default() {
        let (name, value) = cut(piece.trim(), '=')
            .ok_or_else(|| why(id, "roles", piece, "is not `attr=filler`".to_string()))?;
        t = t.with(attr(name.trim()), filler(id, value.trim())?);
    }
    Ok(t)
}

/// The determiner a spelling stands for. [`Det`] carries no spelling of its
/// own — the realiser chooses `a` or `an`, `this` or `these` — so this table
/// belongs to the frozen file rather than to the language.
fn det(word: &str) -> Option<Det> {
    match word {
        "the" => Some(Det::Definite),
        "a" | "an" => Some(Det::Indefinite),
        "this" | "these" => Some(Det::Near),
        "that" | "those" => Some(Det::Far),
        "some" | "any" | "no" | "every" | "each" | "all" | "many" | "both" => {
            Some(Det::Quant(word.to_string()))
        }
        "my" | "your" | "his" | "her" | "its" | "our" | "their" => {
            Some(Det::Owner(word.to_string()))
        }
        _ => None,
    }
}

/// The attribute a name stands for: the three fixed ones, `~p` for one
/// named by its preposition, any other name a kind the world knows.
fn attr(name: &str) -> Attr {
    match name {
        "mod" => Attr::Mod,
        "order" => Attr::Order,
        "count" => Attr::Count,
        _ => match name.strip_prefix('~') {
            Some(p) => Attr::Via(p.to_string()),
            None => Attr::Kind(name.to_string()),
        },
    }
}

/// Split on `sep` at bracket depth zero. `[`…`]` and `{`…`}` nest, so a
/// clause's own `;` and a thing's own `,` stay with the clause and the
/// thing rather than cutting the row.
fn split(text: &str, sep: char) -> Vec<&str> {
    let (mut out, mut depth, mut start) = (Vec::new(), 0i32, 0usize);
    for (i, c) in text.char_indices() {
        match c {
            '[' | '{' => depth += 1,
            ']' | '}' => depth -= 1,
            _ if c == sep && depth == 0 => {
                out.push(&text[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&text[start..]);
    out
}

/// The text before and after the first `sep` at bracket depth zero.
fn cut(text: &str, sep: char) -> Option<(&str, &str)> {
    let mut depth = 0i32;
    for (i, c) in text.char_indices() {
        match c {
            '[' | '{' => depth += 1,
            ']' | '}' => depth -= 1,
            _ if c == sep && depth == 0 => return Some((&text[..i], &text[i + c.len_utf8()..])),
            _ => {}
        }
    }
    None
}

fn named(id: usize, column: &'static str, cell: &str) -> Flaw {
    why(id, column, cell, format!("is not a declared {column}"))
}

fn why(id: usize, column: &'static str, cell: &str, why: String) -> Flaw {
    Flaw::Row { id, column, cell: cell.to_string(), why }
}
