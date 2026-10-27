//! Vocabulary constant keys organized by grammatical part-of-speech.

pub const PRONOUN_KEYS: &[&str] = &[
    "i", "you", "he", "she", "it", "we", "they",
    "me", "him", "her", "us", "them",
    "my", "your", "his", "its", "our", "their",
    "mine", "yours", "hers", "ours", "theirs",
    "this", "that", "these", "those",
    "who", "whom", "which", "what",
];

pub const DETERMINER_KEYS: &[&str] = &[
    "a", "an", "the", "some", "any", "each", "every",
    "all", "both", "either", "neither", "no",
];

pub const AUXILIARY_KEYS: &[&str] = &[
    "is", "are", "was", "were", "be", "been", "being",
    "am", "do", "does", "did", "have", "has", "had",
    "can", "could", "will", "would", "shall", "should",
    "may", "might", "must", "ought",
];

pub const PREPOSITION_KEYS: &[&str] = &[
    "to", "of", "in", "on", "at", "for", "with", "from",
    "as", "by", "about", "into", "through", "during",
    "before", "after", "above", "below", "between",
    "under", "over", "against", "among", "behind",
    "beyond", "within", "without", "upon", "toward",
    "towards", "until", "off", "out", "up", "down",
];

pub const CONJUNCTION_KEYS: &[&str] = &[
    "and", "or", "but", "if", "so", "than",
    "because", "while", "although", "though", "unless",
    "since", "where", "when", "whether",
];

pub const INTERJECTION_KEYS: &[&str] = &[
    "oh", "ah", "wow", "hey", "hi", "hello", "bye",
    "yes", "okay", "ok", "please", "thanks",
];

pub const ADVERB_KEYS: &[&str] = &[
    "not", "very", "really", "just", "also", "too",
    "always", "never", "often", "sometimes", "usually",
    "now", "then", "here", "there", "today", "tomorrow",
    "yesterday", "soon", "late", "early", "quickly",
    "slowly", "well", "badly", "only", "even", "still",
];

pub const ADJECTIVE_KEYS: &[&str] = &[
    "good", "bad", "great", "small", "large", "big",
    "new", "old", "young", "first", "last", "next",
    "same", "different", "own", "other", "such",
    "more", "most", "less", "least", "many", "much",
    "few", "little", "enough", "whole", "half",
];

pub const VERB_KEYS: &[&str] = &[
    "want", "hug", "like", "love", "need", "wish", "hope",
    "see", "look", "hear", "listen", "say", "tell", "speak", "ask",
    "know", "think", "believe", "understand", "remember", "forget",
    "go", "come", "make", "take", "give", "get", "find", "use",
    "help", "try", "feel", "run", "walk", "hold", "touch", "embrace",
    "write", "read", "learn", "teach", "play", "live", "stay", "leave",
    "open", "close", "start", "stop", "call", "send", "show", "bring",
];
