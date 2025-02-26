use phf::{phf_map, Map};

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Token {
    Identifier(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    BoolLiteral(bool),
    Plus,
    Minus,
    Asterisk,
    Slash,
    DoubleSlash,
    Backslash,
    Percent,
    Caret,
    LeftParenthesis,
    RightParenthesis,
    Comma,
    Period,
    Equals,
    NotEquals,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}
const RESERVED_MAP: Map<&'static str, Token> = phf_map!(
    "true" => Token::BoolLiteral(true),
    "false" => Token::BoolLiteral(false),
);
impl Token {
    pub(crate) fn is_reserved(s: &str) -> bool {
        RESERVED_MAP.contains_key(s)
    }
    pub(crate) fn from_reserved(s: &str) -> Option<Token> {
        RESERVED_MAP.get(s).cloned()
    }
    pub(crate) fn reserved_or_identifier(s: String) -> Token {
        if let Some(reserved) = Token::from_reserved(&s) {
            reserved
        } else {
            Token::Identifier(s)
        }
    }
}
