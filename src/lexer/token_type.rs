pub enum TokenType {
    Integer(i64),
    Real(f64),
    Identifier(String),
    Plus,               // +
    Minus,              // -
    Asterisk,           // *
    Slash,              // /
    EqualEqual,         // ==
    ExclamationEqual,   // !=
    LessEqual,          // <=
    GreaterEqual,       // >=
    Less,               // <
    Greater,            // >
    Exclamation,        // !
    Ampersand,          // &
    VerticalBar,        // |
    Circumflex,         // ^
    GreaterGreater,     // >>
    LessLess,           // <<
    Equal,              // =
    LeftSquareBracket,  // [
    RightSquareBracket, // ]
    Comma,              // ,
    LeftParenthesis,    // (
    RightParenthesis,   // )
    LeftBrace,          // {
    RightBrace,         // }
    Semicolon,          // ;
    Var,
    Break,
    Continue,
    Return,
    If,
    Else,
    Unknown,
}
