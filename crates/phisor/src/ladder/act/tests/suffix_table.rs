//! Property 6, recorded: the forms the old `-s/-ing/-ed` code gave, kept as a
//! fixed table now that the code is gone. Each row is
//! `(stem, third, plural, ing, past, doubles_final)`.

use crate::lexicon::suffix::{doubles_final, inflect, Suffix};

const TABLE: &[(&str, &str, &str, &str, &str, bool)] = &[
    ("go", "goes", "gos", "going", "went", false),
    ("fly", "flies", "flies", "flying", "flied", false),
    ("make", "makes", "makes", "making", "maked", false),
    ("agree", "agrees", "agrees", "agreeing", "agreed", false),
    ("stop", "stops", "stops", "stopping", "stopped", true),
    ("monday", "mondays", "mondays", "mondaying", "mondayed", false),
    ("quit", "quits", "quits", "quitting", "quitted", true),
    ("boil", "boils", "boils", "boiling", "boiled", false),
    ("watch", "watches", "watches", "watching", "watched", false),
    ("kiss", "kisses", "kisses", "kissing", "kissed", false),
    ("carry", "carries", "carries", "carrying", "carried", false),
    ("play", "plays", "plays", "playing", "played", false),
    ("see", "sees", "sees", "seeing", "saw", false),
    ("be", "bes", "bes", "being", "bed", false),
    ("have", "has", "haves", "having", "had", false),
    ("do", "does", "dos", "doing", "did", false),
    ("say", "says", "says", "saying", "said", false),
    ("child", "childs", "children", "childing", "childed", false),
    ("man", "mans", "men", "manning", "manned", true),
    ("foot", "foots", "feet", "footing", "footed", false),
    ("city", "cities", "cities", "citying", "citied", false),
    ("box", "boxes", "boxes", "boxing", "boxed", false),
    ("wish", "wishes", "wishes", "wishing", "wished", false),
    ("admit", "admits", "admits", "admitting", "admitted", true),
    ("visit", "visits", "visits", "visiting", "visited", false),
    ("open", "opens", "opens", "opening", "opened", false),
    ("rain", "rains", "rains", "raining", "rained", false),
    ("wait", "waits", "waits", "waiting", "waited", false),
    ("show", "shows", "shows", "showing", "showed", false),
    ("fix", "fixes", "fixes", "fixing", "fixed", false),
    ("add", "adds", "adds", "adding", "added", false),
    ("run", "runs", "runs", "running", "runned", true),
    ("sit", "sits", "sits", "sitting", "sitted", true),
    ("plan", "plans", "plans", "planning", "planned", true),
    ("refer", "refers", "refers", "referring", "referred", true),
    ("prefer", "prefers", "prefers", "preferring", "preferred", true),
    ("travel", "travels", "travels", "traveling", "traveled", false),
    ("buy", "buys", "buys", "buying", "buyed", false),
    ("study", "studies", "studies", "studying", "studied", false),
    ("stay", "stays", "stays", "staying", "stayed", false),
    ("tax", "taxes", "taxes", "taxing", "taxed", false),
    ("bus", "buses", "buses", "bussing", "bussed", true),
    ("fee", "fees", "fees", "feeing", "feed", false),
    ("zoo", "zooes", "zoos", "zooing", "zooed", false),
    ("quiz", "quizs", "quizs", "quizzing", "quizzed", true),
    ("sky", "skies", "skies", "skying", "skied", false),
    ("key", "keys", "keys", "keying", "keyed", false),
    ("die", "dies", "dies", "diing", "died", false),
    ("lie", "lies", "lies", "liing", "lied", false),
    ("tie", "ties", "ties", "tiing", "tied", false),
    ("begin", "begins", "begins", "beginning", "beginned", true),
    ("occur", "occurs", "occurs", "occurring", "occurred", true),
    ("offer", "offers", "offers", "offering", "offered", false),
    ("listen", "listens", "listens", "listening", "listened", false),
    ("happen", "happens", "happens", "happening", "happened", false),
];

#[test]
fn the_rule_file_gives_the_recorded_forms() {
    for (w, third, plural, ing, past, doubles) in TABLE {
        assert_eq!(inflect(w, Suffix::THIRD), *third, "third {w}");
        assert_eq!(inflect(w, Suffix::PLURAL), *plural, "plural {w}");
        assert_eq!(inflect(w, Suffix::ING), *ing, "ing {w}");
        assert_eq!(inflect(w, Suffix::PAST), *past, "past {w}");
        assert_eq!(doubles_final(w), *doubles, "doubles {w}");
    }
}

#[test]
fn the_rule_file_gives_the_book_examples() {
    assert_eq!(inflect("go", Suffix::THIRD), "goes");
    assert_eq!(inflect("fly", Suffix::THIRD), "flies");
    assert_eq!(inflect("make", Suffix::ING), "making");
    assert_eq!(inflect("agree", Suffix::ING), "agreeing");
    assert_eq!(inflect("stop", Suffix::PAST), "stopped");
    assert_eq!(inflect("monday", Suffix::PLURAL), "mondays");
    assert_eq!(inflect("quit", Suffix::ING), "quitting");
}
