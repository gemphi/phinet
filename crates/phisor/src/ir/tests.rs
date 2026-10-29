use super::*;

/// "a <who> was wearing a <colour> <what>"
fn wearing(who: &str, verb: &str, what: &str, colour: &str) -> Act {
    let theme = Thing::new(what).det(Det::Indefinite).with(Attr::Kind("colour".into()), Filler::word(colour));
    Act::new(
        Force::Assert,
        Frame::new(verb, Tense::Past)
            .aspect(Aspect::Progressive)
            .with(Role::Agent, Filler::Thing(Thing::new(who).det(Det::Indefinite)))
            .with(Role::Theme, Filler::Thing(theme)),
    )
}

#[test]
fn the_girl_and_the_boy_are_one_template_with_two_fillings() {
    let girl = wearing("girl", "wear", "suit", "blue");
    let boy = wearing("boy", "wear", "cap", "white");
    assert_eq!(girl.to_string(), "ASSERT(wear(agent: girl, theme: suit{colour: blue}), past, progressive)");
    assert_eq!(boy.to_string(), "ASSERT(wear(agent: boy, theme: cap{colour: white}), past, progressive)");
    assert_ne!(girl, boy);
    assert_eq!(girl.template(), boy.template());
    assert_eq!(girl.template().as_str(), "ASSERT(wear(agent: $N, theme: $N{$A}), past, progressive)");
}

#[test]
fn another_verb_is_another_template_but_the_same_construction() {
    let girl = wearing("girl", "wear", "suit", "blue");
    let man = wearing("man", "drive", "car", "red");
    assert_ne!(girl.template(), man.template());
    assert_eq!(girl.construction(), man.construction());
    assert_eq!(man.construction().as_str(), "ASSERT($V(agent: $N, theme: $N{$A}), past, progressive)");
}

#[test]
fn the_shape_keeps_attributes_and_features_but_not_determiners() {
    let girl = wearing("girl", "wear", "suit", "blue");
    // "the girl was wearing a blue suit": another determiner, the same shape.
    let mut the_girl = girl.clone();
    the_girl.frame.slots[0].1 = Filler::Thing(Thing::new("girl").det(Det::Definite));
    assert_eq!(girl.template(), the_girl.template());
    // "a girl was wearing a suit": one attribute fewer, another shape.
    let mut plain = girl.clone();
    plain.frame.slots[1].1 = Filler::Thing(Thing::new("suit").det(Det::Indefinite));
    assert_ne!(girl.template(), plain.template());
    // "a girl is wearing a blue suit": another tense, another template.
    let mut now = girl.clone();
    now.frame.tense = Tense::Present;
    assert_ne!(girl.template(), now.template());
}

#[test]
fn the_first_king_of_france_is_a_question_with_a_hole() {
    let king = Thing::new("king").det(Det::Definite).with(Attr::Order, Filler::word("first")).with(Attr::Via("of".into()), Filler::name("France"));
    let ask = Act::new(Force::Ask, Frame::new("be", Tense::Past).with(Role::Theme, Filler::Hole(Wh::Who)).with(Role::As, Filler::Thing(king)));
    assert_eq!(ask.to_string(), "ASK(be(theme: ?who, as: king{order: first, of: France}), past)");
    assert_eq!(ask.template().as_str(), "ASK(be(theme: ?who, as: $N{order: $A, of: $N}), past)");
}

#[test]
fn a_wish_is_a_force_over_what_is_wished() {
    // "I wish I could sing": the force is the wish, the frame what is wished
    // (Searle's F(p)); `could` is `can` in the past.
    let sing = Frame::new("sing", Tense::Past).modal(Modal::Can).with(Role::Agent, Filler::pronoun("I"));
    assert_eq!(Act::new(Force::Wish, sing).to_string(), "WISH(sing(agent: I), past, can)");
}

#[test]
fn a_frame_can_fill_a_slot_of_another() {
    // "I want to go to London": operators over operators.
    let go = Frame::new("go", Tense::Present).with(Role::Agent, Filler::pronoun("I")).with(Role::Via("to".into()), Filler::name("London"));
    let want = Act::new(Force::Assert, Frame::new("want", Tense::Present).with(Role::Agent, Filler::pronoun("I")).with(Role::Theme, Filler::clause(go)));
    assert_eq!(want.to_string(), "ASSERT(want(agent: I, theme: [go(agent: I, to: London), present]), present)");
    assert_eq!(want.template().as_str(), "ASSERT(want(agent: $N, theme: [go(agent: $N, to: $N), present]), present)");
}

#[test]
fn every_feature_is_written_and_plurals_are_marked() {
    let f = Frame::new("break", Tense::Present)
        .aspect(Aspect::Perfect)
        .passive()
        .negative()
        .with(Role::Theme, Filler::Thing(Thing::new("window").det(Det::Definite).plural()));
    assert_eq!(Act::new(Force::Assert, f).to_string(), "ASSERT(break(theme: window·pl), present, perfect, passive, not)");
}

#[test]
fn each_force_has_searles_point_and_fit() {
    assert_eq!(Force::Ask.point(), Point::Directive);
    assert_eq!(Force::Wish.point(), Point::Expressive);
    assert_eq!(Force::Wish.fit(), Fit::WorldToWord);
    assert_eq!(Force::Express.fit(), Fit::Null);
    assert_eq!(Force::Assert.fit(), Fit::WordToWorld);
    assert_eq!(Force::Declare.fit(), Fit::Both);
    for f in Force::ALL {
        assert_eq!(Force::named(f.name()), Some(f));
    }
    assert_eq!(Force::named("assert"), None);
}
