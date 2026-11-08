use super::word::{load_tagged, Words};
use std::sync::OnceLock;

tagged_list!(
    /// Conjunctions that join two clauses.
    clause_conjunctions,
    "conjunction.tsv",
    "clause"
);

tagged_list!(
    /// Words that join or open a clause, for the abstention buckets.
    clause_openers,
    "conjunction.tsv",
    "opener"
);
