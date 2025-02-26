use std::iter::Peekable;

use anyhow::{anyhow, Result};
use phf::{phf_set, Set};

use super::token::Token;
const INVALID_IDENTIFIER_CHAR: Set<char> = phf_set!(
    '+', '-', '/', '\'', '*', '!', '@', '#', '$', '%', '^', '&', '(', ')', ';', ':', '<', '>', '=',
    '?', ',', '.', '\\', '|', '~', '`', '"'
);

pub(crate) struct Tokens<C: Iterator<Item = char>> {
    chars: Peekable<C>,
    /// Number of chars already read (= index of the next char to read)
    read: usize,
}

#[derive(Debug)]
pub(crate) struct ParsedToken {
    pub(crate) token: Token,
    pub(crate) start: usize,
    pub(crate) end: usize,
}

impl<C: Iterator<Item = char>> Tokens<C> {
    pub(crate) fn new(chars: C) -> Self {
        Self {
            read: 0,
            chars: chars.peekable(),
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.chars.next();
        if c.is_some() {
            self.read += 1;
        }
        c
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.chars.peek()
    }
}

impl<C: Iterator<Item = char>> Iterator for Tokens<C> {
    type Item = Result<ParsedToken>;

    fn next(&mut self) -> Option<Self::Item> {
        let first_char = match self.peek_char() {
            Some(c) => c,
            None => return None,
        };
        Some(match first_char {
            c if c.is_whitespace() => {
                self.next_char();
                return self.next();
            }
            c if c.is_ascii_digit() || *c == '.' => self.parse_number(),
            c if INVALID_IDENTIFIER_CHAR.contains(c) => self.parse_reserved_char(),
            c if is_valid_identifier_starting_char(c) => self.parse_text(),

            _ => return Some(Err(anyhow!("Unexpected character: {}", first_char))),
        })
    }
}

fn is_identifier_ending_char(char: &char) -> bool {
    char.is_whitespace() || INVALID_IDENTIFIER_CHAR.contains(char)
}

fn is_valid_identifier_starting_char(char: &char) -> bool {
    !INVALID_IDENTIFIER_CHAR.contains(char)
}
impl<C: Iterator<Item = char>> Tokens<C> {
    fn parse_number(&mut self) -> Result<ParsedToken> {
        let mut parsed_string = String::new();
        let start = self.read;

        let mut parsing_mode = NumberParsingState::NoDecimalPoint;

        loop {
            match self.peek_char() {
                Some(c) if c.is_ascii_digit() => {
                    parsed_string.push(self.next_char().expect("peek_char returned Some"));
                }
                Some('.') if parsing_mode == NumberParsingState::NoDecimalPoint => {
                    parsed_string.push('.');
                    parsing_mode = NumberParsingState::DecimalPoint;
                    self.next_char();
                }
                _ => break,
            }
        }
        let end = self.read;
        let token = match parsing_mode {
            NumberParsingState::NoDecimalPoint => Token::IntLiteral(match parsed_string.parse() {
                Ok(i) => i,
                Err(err) => {
                    return Err(anyhow!(
                        "Failed to parse int literal at location {}..{}",
                        start,
                        end
                    )
                    .context(err))
                }
            }),
            NumberParsingState::DecimalPoint => Token::FloatLiteral(match parsed_string.parse() {
                Ok(f) => f,
                Err(err) => {
                    return Err(anyhow!(
                        "Failed to parse float literal at location {}..{}",
                        start,
                        end
                    )
                    .context(err))
                }
            }),
        };
        Ok(ParsedToken { token, start, end })
    }
    fn parse_text(&mut self) -> Result<ParsedToken> {
        let mut parsed_text = String::new();

        let start = self.read;
        loop {
            match self.peek_char() {
                Some(c) if !is_identifier_ending_char(c) => {
                    parsed_text.push(self.next_char().expect("peek_char returned Some"));
                }
                _ => break,
            }
        }
        let end = self.read;
        let token = Token::reserved_or_identifier(parsed_text);
        Ok(ParsedToken { token, start, end })
    }
    fn parse_reserved_char(&mut self) -> Result<ParsedToken> {
        let start = self.read;
        let token = match self.next_char().expect("peek_char returned Some") {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => {
                if self.peek_char() == Some(&'/') {
                    self.next_char();
                    Token::DoubleSlash
                } else {
                    Token::Slash
                }
            }
            '\\' => Token::Backslash,
            '%' => Token::Percent,
            '^' => Token::Caret,
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            ',' => Token::Comma,
            '.' => Token::Period,
            '=' => Token::Equals,
            '!' => {
                if self.peek_char() == Some(&'=') {
                    Token::NotEquals
                } else {
                    return Err(anyhow!(
                        "Unexpected character: ! at {}. Hint: ! is only valid as part of \"!=\"",
                        start
                    ));
                }
            }

            '<' => {
                if self.peek_char() == Some(&'=') {
                    self.next_char();
                    Token::LessThanOrEqual
                } else {
                    Token::LessThan
                }
            }
            '>' => {
                if self.peek_char() == Some(&'=') {
                    self.next_char();
                    Token::GreaterThanOrEqual
                } else {
                    Token::GreaterThan
                }
            }
            _ => return Err(anyhow!("Unexpected character: {}", self.read)),
        };
        let end = self.read;
        Ok(ParsedToken { token, start, end })
    }
}
#[derive(Debug, PartialEq, Eq)]
enum NumberParsingState {
    NoDecimalPoint,
    DecimalPoint,
}
