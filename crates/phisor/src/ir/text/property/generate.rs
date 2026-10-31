use crate::ir::{
    Act, Aspect, Attr, Det, Filler, Force, Frame, Modal, Polarity, Role, Tense, Thing, Voice, Wh,
};
use phical::keyboard::MAX_CLAUSE_DEPTH;
use phistd::gen::{Gen, Seeded};

const TEXT: [&str; 12] = [
    "plain",
    "two words",
    "comma,value",
    "colon:value",
    "brace{value}",
    "quote\"slash\\",
    "line\nbreak",
    "ümlaut",
    "agent",
    "mod",
    "quant(value)",
    "",
];

#[derive(Debug)]
pub(super) struct Case(pub Act);

impl Gen for Case {
    fn draw(g: &mut Seeded) -> Self {
        let depth = g.below((MAX_CLAUSE_DEPTH + 1) as u64) as usize;
        Case(Act::new(*g.pick(&Force::ALL), frame(g, depth, 4)))
    }
}

fn frame(g: &mut Seeded, depth: usize, most_roles: usize) -> Frame {
    let mut out = Frame::new(g.pick(&TEXT), *g.pick(&Tense::ALL));
    out.aspect = *g.pick(&Aspect::ALL);
    out.modal = match g.below((Modal::ALL.len() + 1) as u64) as usize {
        0 => None,
        index => Some(Modal::ALL[index - 1]),
    };
    out.voice = *g.pick(&Voice::ALL);
    out.polarity = *g.pick(&Polarity::ALL);
    let roles = g.below((most_roles + 1) as u64) as usize;
    for _ in 0..roles {
        out.slots.push((role(g), filler(g, depth)));
    }
    out
}

fn role(g: &mut Seeded) -> Role {
    match g.below(7) {
        0 => Role::Agent,
        1 => Role::Theme,
        2 => Role::Recipient,
        3 => Role::As,
        4 => Role::Via("in".to_string()),
        5 => Role::Via("agent".to_string()),
        _ => Role::Via((*g.pick(&TEXT)).to_string()),
    }
}

fn filler(g: &mut Seeded, depth: usize) -> Filler {
    let variants = if depth == 0 { 5 } else { 6 };
    match g.below(variants) {
        0 => Filler::Thing(thing(g, depth)),
        1 => Filler::Name((*g.pick(&TEXT)).to_string()),
        2 => Filler::Pronoun((*g.pick(&TEXT)).to_string()),
        3 => Filler::Word((*g.pick(&TEXT)).to_string()),
        4 => Filler::Hole(*g.pick(&Wh::ALL)),
        _ => Filler::Clause(Box::new(frame(g, depth - 1, 1))),
    }
}

fn thing(g: &mut Seeded, depth: usize) -> Thing {
    let mut out = Thing::new(g.pick(&TEXT));
    out.det = match g.below(7) {
        0 => None,
        1 => Some(Det::Definite),
        2 => Some(Det::Indefinite),
        3 => Some(Det::Near),
        4 => Some(Det::Far),
        5 => Some(Det::Quant((*g.pick(&TEXT)).to_string())),
        _ => Some(Det::Owner((*g.pick(&TEXT)).to_string())),
    };
    out.plural = g.below(2) == 1;
    if g.below(2) == 1 {
        let attr = match g.below(7) {
            0 => Attr::Mod,
            1 => Attr::Order,
            2 => Attr::Count,
            3 => Attr::Kind("colour".to_string()),
            4 => Attr::Kind("mod".to_string()),
            5 => Attr::Via("of".to_string()),
            _ => Attr::Via((*g.pick(&TEXT)).to_string()),
        };
        let value = if depth == 0 {
            Filler::Word((*g.pick(&TEXT)).to_string())
        } else {
            filler(g, depth - 1)
        };
        out.attrs.push((attr, value));
    }
    out
}
