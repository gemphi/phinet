//! Unit tests for topology and spider net.

use super::*;

#[test]
fn test_spider_net_keyed_relations() {
    let mut net = SpiderNet::new();
    let _ptype = net.process_text("I want to hug you. Can you hear me? Yes indeed!");

    assert_eq!(net.total_sentences, 3);
    assert_eq!(net.total_paragraphs, 1);
    assert!(!net.key_to_hexagrams.is_empty());
    assert!(!net.hexagram_to_keys.is_empty());
    assert!(!net.pos_shape_relations.is_empty());

    let pron_relations = net.pos_shape_relations.get("PRON");
    assert!(pron_relations.is_some());

    assert!(net.sentence_diversity() > 0.0);
    assert!(net.paragraph_diversity() > 0.0);
}

#[test]
fn test_spider_net_spin_structure() {
    let mut net = SpiderNet::new();
    net.process_text("I want to hug you. We need to love them.");

    let syntax_keys: Vec<_> = net.key_to_hexagrams.keys().cloned().collect();
    assert!(!syntax_keys.is_empty());

    let spun = net.spin_structure(&syntax_keys[0], 0.0);
    assert!(!spun.is_empty());
}
