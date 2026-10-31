//! The canonical printer. Bare, uncomplicated values keep the notation's
//! compact form; quotes and variant markers appear only where omitting them
//! would make two different IR values read alike.

use super::super::filler::{Attr, Det, Thing};
use super::super::{Act, Aspect, Filler, Frame, Polarity, Role, Voice};
use std::fmt::Write;

pub(super) fn act(act: &Act) -> String {
    let mut out = String::new();
    out.push_str(act.force.name());
    out.push('(');
    frame(&mut out, &act.frame);
    out.push(')');
    out
}

fn frame(out: &mut String, frame: &Frame) {
    atom(out, &frame.pred);
    out.push('(');
    for (index, (role, value)) in frame.slots.iter().enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        role_name(out, role);
        out.push_str(": ");
        filler(out, value);
    }
    out.push(')');
    out.push_str(", ");
    out.push_str(frame.tense.name());
    if frame.aspect != Aspect::Simple {
        out.push_str(", ");
        out.push_str(frame.aspect.name());
    }
    if let Some(modal) = frame.modal {
        out.push_str(", ");
        out.push_str(modal.name());
    }
    if frame.voice == Voice::Passive {
        out.push_str(", passive");
    }
    if frame.polarity == Polarity::Negative {
        out.push_str(", not");
    }
}

fn role_name(out: &mut String, role: &Role) {
    match role {
        Role::Via(name) if Role::NAMED.iter().any(|reserved| reserved.name() == name) => {
            out.push('~');
            atom(out, name);
        }
        _ => atom(out, role.name()),
    }
}

fn filler(out: &mut String, value: &Filler) {
    match value {
        Filler::Thing(value) => thing(out, value),
        Filler::Name(value) => marked(out, '*', value),
        Filler::Pronoun(value) => marked(out, '^', value),
        Filler::Word(value) => marked(out, '\'', value),
        Filler::Hole(value) => {
            out.push('?');
            out.push_str(value.name());
        }
        Filler::Clause(value) => {
            out.push('[');
            frame(out, value);
            out.push(']');
        }
    }
}

fn marked(out: &mut String, mark: char, value: &str) {
    out.push(mark);
    atom(out, value);
}

fn thing(out: &mut String, thing: &Thing) {
    match &thing.det {
        None => {}
        Some(Det::Definite) => out.push_str("the "),
        Some(Det::Indefinite) => out.push_str("a "),
        Some(Det::Near) => out.push_str("this "),
        Some(Det::Far) => out.push_str("that "),
        Some(Det::Quant(value)) => carried_det(out, "quant", value),
        Some(Det::Owner(value)) => carried_det(out, "owner", value),
    }
    atom(out, &thing.head);
    if thing.plural {
        out.push_str("·pl");
    }
    if thing.attrs.is_empty() {
        return;
    }
    out.push('{');
    for (index, (attr, value)) in thing.attrs.iter().enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        attr_name(out, attr);
        out.push_str(": ");
        filler(out, value);
    }
    out.push('}');
}

fn carried_det(out: &mut String, name: &str, value: &str) {
    out.push_str(name);
    out.push('(');
    atom(out, value);
    out.push_str(") ");
}

fn attr_name(out: &mut String, attr: &Attr) {
    match attr {
        Attr::Mod => out.push_str("mod"),
        Attr::Order => out.push_str("order"),
        Attr::Count => out.push_str("count"),
        Attr::Kind(name) if matches!(name.as_str(), "mod" | "order" | "count") => {
            out.push_str("kind(");
            atom(out, name);
            out.push(')');
        }
        Attr::Kind(name) => atom(out, name),
        Attr::Via(name) => {
            out.push('~');
            atom(out, name);
        }
    }
}

/// An uncomplicated atom is left bare. Everything else is quoted with the
/// few escapes needed to preserve every Rust string exactly.
fn atom(out: &mut String, value: &str) {
    if !value.is_empty() && value.chars().all(safe) {
        out.push_str(value);
        return;
    }
    out.push('"');
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            ch if ch.is_control() => {
                let _ = write!(out, "\\u{{{:x}}}", ch as u32);
            }
            ch => out.push(ch),
        }
    }
    out.push('"');
}

fn safe(ch: char) -> bool {
    ch.is_alphanumeric() || matches!(ch, '_' | '-' | '.' | '/')
}
