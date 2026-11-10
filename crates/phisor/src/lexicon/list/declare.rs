/// Declare an accessor for a plain list file, read once on first use.
macro_rules! plain_list {
    ($(#[$doc:meta])* $name:ident, $file:literal) => {
        $(#[$doc])*
        pub fn $name() -> &'static Words {
            static CELL: OnceLock<Words> = OnceLock::new();
            CELL.get_or_init(|| load_plain($file))
        }
    };
}

/// Declare an accessor for one tag of a tagged list file.
macro_rules! tagged_list {
    ($(#[$doc:meta])* $name:ident, $file:literal, $tag:literal) => {
        $(#[$doc])*
        pub fn $name() -> &'static Words {
            static CELL: OnceLock<Words> = OnceLock::new();
            CELL.get_or_init(|| load_tagged($file, $tag))
        }
    };
}
