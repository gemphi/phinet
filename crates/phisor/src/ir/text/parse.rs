//! Recursive-descent reader for the reversible Act notation.

use super::super::filler::{Attr, Det, Thing};
use super::super::{Act, Aspect, Filler, Force, Frame, Modal, Polarity, Role, Tense, Voice, Wh};
use super::scan::Cursor;
use super::ParseError;
use phical::keyboard::MAX_CLAUSE_DEPTH;

pub(super) fn act(text: &str) -> Result<Act, ParseError> {
    let mut at = Cursor::new(text);
    let force_at = at.offset();
    let force_name = at.atom()?;
    let force = Force::named(&force_name)
        .ok_or_else(|| ParseError::new(force_at, format!("unknown force {force_name:?}")))?;
    at.expect('(', "after the force")?;
    let frame = frame(&mut at, ')', 0)?;
    at.expect(')', "after the Act")?;
    at.finish()?;
    Ok(Act::new(force, frame))
}

fn frame(at: &mut Cursor<'_>, close: char, depth: usize) -> Result<Frame, ParseError> {
    let pred = at.atom()?;
    let slots = slots(at, depth)?;
    at.expect(',', "before the tense")?;
    let tense_at = at.offset();
    let tense_name = at.segment(close)?;
    let tense = Tense::named(&tense_name)
        .ok_or_else(|| ParseError::new(tense_at, format!("unknown tense {tense_name:?}")))?;
    let mut out = Frame::new(&pred, tense);
    out.slots = slots;
    let mut seen = Seen::default();
    while at.eat(',') {
        let field_at = at.offset();
        let value = at.segment(close)?;
        feature(&mut out, &mut seen, &value).map_err(|why| ParseError::new(field_at, why))?;
    }
    at.skip();
    if at.peek() != Some(close) {
        return Err(at.error(format!("expected frame closing mark {close:?}")));
    }
    Ok(out)
}

fn slots(at: &mut Cursor<'_>, depth: usize) -> Result<Vec<(Role, Filler)>, ParseError> {
    at.expect('(', "after the predicate")?;
    let mut out = Vec::new();
    if at.eat(')') {
        return Ok(out);
    }
    loop {
        let role = role(at)?;
        at.expect(':', "after the role")?;
        out.push((role, filler(at, depth)?));
        if at.eat(')') {
            return Ok(out);
        }
        at.expect(',', "between roles")?;
    }
}

fn role(at: &mut Cursor<'_>) -> Result<Role, ParseError> {
    if at.eat('~') {
        Ok(Role::Via(at.atom()?))
    } else {
        Ok(Role::named(&at.atom()?))
    }
}

fn filler(at: &mut Cursor<'_>, depth: usize) -> Result<Filler, ParseError> {
    at.skip();
    match at.peek() {
        Some('*') => {
            at.bump();
            Ok(Filler::Name(at.atom()?))
        }
        Some('^') => {
            at.bump();
            Ok(Filler::Pronoun(at.atom()?))
        }
        Some('\'') => {
            at.bump();
            Ok(Filler::Word(at.atom()?))
        }
        Some('?') => {
            let start = at.offset();
            at.bump();
            let name = at.atom()?;
            Wh::named(&name)
                .map(Filler::Hole)
                .ok_or_else(|| ParseError::new(start, format!("unknown hole ?{name}")))
        }
        Some('[') => clause(at, depth),
        Some(_) => thing(at, depth).map(Filler::Thing),
        None => Err(at.error("expected a filler")),
    }
}

fn clause(at: &mut Cursor<'_>, depth: usize) -> Result<Filler, ParseError> {
    let start = at.offset();
    at.expect('[', "to open a clause")?;
    let depth = depth + 1;
    if depth > MAX_CLAUSE_DEPTH {
        return Err(ParseError::new(
            start,
            format!("clause depth {depth} exceeds the cap {MAX_CLAUSE_DEPTH}"),
        ));
    }
    let inner = frame(at, ']', depth)?;
    at.expect(']', "to close a clause")?;
    Ok(Filler::Clause(Box::new(inner)))
}

fn thing(at: &mut Cursor<'_>, depth: usize) -> Result<Thing, ParseError> {
    let det = if at.take("quant(") {
        let value = at.atom()?;
        at.expect(')', "after a quantifier")?;
        Some(Det::Quant(value))
    } else if at.take("owner(") {
        let value = at.atom()?;
        at.expect(')', "after an owner")?;
        Some(Det::Owner(value))
    } else if at.keyword("the") {
        Some(Det::Definite)
    } else if at.keyword("a") {
        Some(Det::Indefinite)
    } else if at.keyword("this") {
        Some(Det::Near)
    } else if at.keyword("that") {
        Some(Det::Far)
    } else {
        None
    };
    let head = at.atom()?;
    let mut out = Thing::new(&head);
    out.det = det;
    out.plural = at.take("·pl");
    if !at.eat('{') {
        return Ok(out);
    }
    if at.eat('}') {
        return Ok(out);
    }
    loop {
        let attr = attr(at)?;
        at.expect(':', "after the attribute")?;
        let value = filler(at, depth)?;
        out.attrs.push((attr, value));
        if at.eat('}') {
            return Ok(out);
        }
        at.expect(',', "between attributes")?;
    }
}

fn attr(at: &mut Cursor<'_>) -> Result<Attr, ParseError> {
    if at.eat('~') {
        return Ok(Attr::Via(at.atom()?));
    }
    if at.take("kind(") {
        let name = at.atom()?;
        at.expect(')', "after an attribute kind")?;
        return Ok(Attr::Kind(name));
    }
    Ok(match at.atom()?.as_str() {
        "mod" => Attr::Mod,
        "order" => Attr::Order,
        "count" => Attr::Count,
        name => Attr::Kind(name.to_string()),
    })
}

#[derive(Default)]
struct Seen {
    aspect: bool,
    modal: bool,
    voice: bool,
    polarity: bool,
}

fn feature(frame: &mut Frame, seen: &mut Seen, value: &str) -> Result<(), String> {
    if let Some(aspect) = Aspect::named(value) {
        unique(&mut seen.aspect, "aspect")?;
        frame.aspect = aspect;
    } else if let Some(modal) = Modal::named(value) {
        unique(&mut seen.modal, "modal")?;
        frame.modal = Some(modal);
    } else if let Some(voice) = Voice::named(value) {
        unique(&mut seen.voice, "voice")?;
        frame.voice = voice;
    } else if value == "not" {
        unique(&mut seen.polarity, "polarity")?;
        frame.polarity = Polarity::Negative;
    } else if let Some(polarity) = Polarity::named(value) {
        unique(&mut seen.polarity, "polarity")?;
        frame.polarity = polarity;
    } else {
        return Err(format!("unknown frame feature {value:?}"));
    }
    Ok(())
}

fn unique(seen: &mut bool, field: &str) -> Result<(), String> {
    if *seen {
        Err(format!("duplicate {field}"))
    } else {
        *seen = true;
        Ok(())
    }
}
