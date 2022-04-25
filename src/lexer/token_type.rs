pub enum TokenType {
    Integer(i64),
    Real(f64),
    Plus,             // +
    Minus,            // -
    Asterisk,         // *
    Slash,            // /
    EqualEqual,       // ==
    ExclamationEqual, // !=
    LessEqual,        // <=
    GreaterEqual,     // >=
    Less,             // <
    Greater,          // >
    Exclamation,      // !
    Ampersand,        // &
    VerticalBar,      // |
    Circumflex,       // ^
    GreaterGreater,   // >>
    LessLess,         // <<
    Unknown,
}
