//! Pre-tokeniser, byte table, and a tiny merge table.

    use super::*;
    use super::bytes::bytes_to_unicode;

    #[test]
    fn pretokenizer_matches_gpt2_pattern() {
        assert_eq!(pretokenize("Hello world"), vec!["Hello", " world"]);
        assert_eq!(pretokenize("I'm here, you're not."), vec!["I", "'m", " here", ",", " you", "'re", " not", "."]);
        assert_eq!(pretokenize("a  b"), vec!["a", " ", " b"]);
        assert_eq!(pretokenize("x\n\ny"), vec!["x", "\n", "\n", "y"]); // \s+(?!\S) backtracks to one
        assert_eq!(pretokenize("1984 was"), vec!["1984", " was"]);
        assert_eq!(pretokenize("end  "), vec!["end", "  "]);
    }

    #[test]
    fn byte_table_is_a_bijection() {
        let (t, r) = bytes_to_unicode();
        for b in 0..=255u8 {
            assert_eq!(r[&t[b as usize]], b);
        }
    }

    #[test]
    fn a_tiny_vocabulary_merges_by_rank() {
        let vocab = r#"{"h":0,"e":1,"l":2,"o":3,"he":4,"ll":5,"hell":6,"Ġ":7,"Ġo":8}"#;
        let merges = "#version: 0.2\nh e\nl l\nhe ll\nĠ o";
        let bpe = Bpe::load(vocab, merges).unwrap();
        assert_eq!(bpe.pieces("hello"), vec!["hell", "o"]);
        assert_eq!(bpe.encode("hello o"), vec![6, 3, 8]);
        assert_eq!(bpe.decode(&[6, 3, 8]), "hello o");
    }
