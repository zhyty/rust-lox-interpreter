use std::iter::Iterator;
use std::iter::Peekable;
use std::string::ToString;
use unicode_segmentation::UnicodeSegmentation;

pub struct Scanner<'a> {
    source: &'a str,
    graphemes_iter: Peekable<unicode_segmentation::GraphemeIndices<'a>>,

    token_start_byte_offset: usize,
    current_byte_offset: usize,

    line_number: usize,
    tokens: Vec<AnnotatedToken<'a>>,
    // TODO: this has_error stuff is duplicated in Lox. Maybe have a reference
    // to Lox somehow?
    has_error: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            graphemes_iter: source.grapheme_indices(true).peekable(),
            token_start_byte_offset: 0,
            // Should be updated by first `advance` call.
            current_byte_offset: 0,

            line_number: 1,
            tokens: Vec::new(),
            has_error: false,
        }
    }

    // NOTE: maybe this should consume self?
    pub fn scan_tokens(&mut self) -> &Vec<AnnotatedToken<'a>> {
        while self.scan_token() {}
        self.add_token(Token::EOF);
        &self.tokens
    }

    pub fn has_error(&self) -> bool {
        self.has_error
    }

    // TODO: return nothing, just update state?
    fn scan_token(&mut self) -> bool {
        let grapheme = match self.advance() {
            None => return false,
            Some(grapheme) => grapheme,
        };
        match grapheme {
            "(" => self.add_token(Token::LeftParen),
            ")" => self.add_token(Token::RightParen),
            "{" => self.add_token(Token::LeftBrace),
            "}" => self.add_token(Token::RightBrace),
            "," => self.add_token(Token::Comma),
            "." => self.add_token(Token::Dot),
            "-" => self.add_token(Token::Minus),
            "+" => self.add_token(Token::Plus),
            ";" => self.add_token(Token::Semicolon),
            "*" => self.add_token(Token::Star),
            "!" => {
                if self.advance_if_next_matches("=") {
                    self.add_token(Token::BangEqual);
                } else {
                    self.add_token(Token::Bang);
                };
            }
            "=" => {
                if self.advance_if_next_matches("=") {
                    self.add_token(Token::EqualEqual);
                } else {
                    self.add_token(Token::Equal);
                }
            }
            "<" => {
                if self.advance_if_next_matches("=") {
                    self.add_token(Token::LessEqual);
                } else {
                    self.add_token(Token::Less);
                }
            }
            ">" => {
                if self.advance_if_next_matches("=") {
                    self.add_token(Token::GreaterEqual);
                } else {
                    self.add_token(Token::Greater);
                }
            }
            "/" => {
                if self.advance_if_next_matches("/") {
                    // Comment
                    while let Some(grapheme) = self.peek() {
                        if is_newline(grapheme) {
                            break;
                        }
                        // Note: learning Rust, no problem getting a `&mut
                        // self` here because `grapheme`'s lifetime ends
                        // before it.
                        self.advance();
                        // grapheme; // uncommenting this line causes a
                        // double &mut borrow error.
                    }
                } else {
                    // Division
                    self.add_token(Token::Slash);
                }
            }
            "\"" => {
                // Note: we're differing from the textbook by including the
                // quotation marks. We could always trim them in `add_token`.
                if self.advance_string() {
                    self.add_string_token();
                }
            }
            // Must precede whitespace check since newlines are also whitespace.
            // Note: maybe we can move the line number counting logic into
            // advance?
            _ if is_newline(grapheme) => {
                self.line_number += 1;
            }
            _ if is_whitespace(grapheme) => {
                // continue
            }
            // Lox just understands ASCII 0-9
            _ if is_digit(grapheme) => {
                self.advance_number();
                self.add_number_token();
            }
            _ if is_identifier_head(grapheme) => {
                self.advance_identifier();
                self.add_identifier_token();
            }
            _ => {
                self.report_error(self.line_number, "Unexpected character.");
                // Continue trying to lex/scan...
            }
        }

        // Move on to next token.
        self.token_start_byte_offset = self.current_byte_offset;

        true
    }

    fn peek(&mut self) -> Option<&str> {
        let (_, grapheme) = self.graphemes_iter.peek()?;
        Some(grapheme)
    }

    fn double_peek(&mut self) -> Option<&str> {
        let mut iter_clone = self.graphemes_iter.clone();
        // We want to early return if it's null, since our iterator interface should signal EOF when None.
        iter_clone.next()?;
        let (_, grapheme) = iter_clone.next()?;
        Some(grapheme)
    }

    fn advance(&mut self) -> Option<&str> {
        let (byte_offset, grapheme) = self.graphemes_iter.next()?;
        self.current_byte_offset = byte_offset + grapheme.len();
        Some(grapheme)
    }

    fn advance_if_next_matches(&mut self, to_match: &str) -> bool {
        let matches = self.peek() == Some(to_match);
        if matches {
            self.advance();
        }
        matches
    }

    fn advance_string(&mut self) -> bool {
        while let Some(grapheme) = self.peek() {
            if grapheme == "\"" {
                break;
            }

            // We allow multiline strings as well, so watch out for line number
            // changes.
            if is_newline(grapheme) {
                self.line_number += 1;
            }

            self.advance();
        }

        // Give up on string if we don't see a terminating quote.
        if self.peek().is_none() {
            self.report_error(self.line_number, "Unterminated string");
            return false;
        }
        // Past the closing quotation mark.
        self.advance();

        true
    }

    // Note: doesn't return bool since it can't really fail.
    fn advance_number(&mut self) {
        while let Some(grapheme) = self.peek() {
            if !is_digit(grapheme) {
                break;
            }
            self.advance();
        }

        // Not a number anymore, but could be a decimal point. If decimal point
        // is followed by a number, we continue processing the number.
        // Otherwise, it could be a function call (DOT).

        // Note: what if we treated the decimal "." as DOT as well? And worried
        // about number/function call during parsing? I guess we would have to
        // worry about operator precedence then... and "NUMBER . FUNCTION .
        // NUMBER" could be a weird case.
        if self.peek() == Some(".") {
            if let Some(grapheme_after_dot) = self.double_peek() {
                if is_digit(grapheme_after_dot) {
                    self.advance();

                    while let Some(grapheme) = self.peek() {
                        if !is_digit(grapheme) {
                            break;
                        }
                        self.advance();
                    }
                }
            }
        }
    }

    fn advance_identifier(&mut self) {
        while let Some(grapheme) = self.peek() {
            if !is_identifier_tail(grapheme) {
                break;
            }
            self.advance();
        }
    }

    fn add_token(&mut self, token: Token<'a>) {
        self.tokens.push(AnnotatedToken {
            token,
            line_number: self.line_number,
        })
    }

    // Trying to pass a token containing a immutable reference while holding a
    // mutable reference causes some problems for certain tokens.
    fn add_number_token(&mut self) {
        let number_lexeme = &self.source[self.token_start_byte_offset..self.current_byte_offset];
        // TODO: consider replacing unwrap?
        let number: f64 = number_lexeme.parse().unwrap();
        self.tokens.push(AnnotatedToken {
            token: Token::Number { number },
            line_number: self.line_number,
        })
    }

    // See `add_number_token`
    fn add_identifier_token(&mut self) {
        let identifier = &self.source[self.token_start_byte_offset..self.current_byte_offset];
        self.tokens.push(AnnotatedToken {
            token: token_from_identifier(identifier),
            line_number: self.line_number,
        });
    }

    // See `add_number_token`
    fn add_string_token(&mut self) {
        let quoted_str = &self.source[self.token_start_byte_offset..self.current_byte_offset];
        self.tokens.push(AnnotatedToken {
            token: Token::String { quoted_str },
            line_number: self.line_number
        });
    }

    fn report_error(&mut self, line: usize, message: &str) {
        eprintln!("[line {}] Error (TODO where): {}", line, message);
        self.has_error = true;
    }
}

impl<'a> std::fmt::Debug for Scanner<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Scanner")
            .field("token_start_byte_offset", &self.token_start_byte_offset)
            .field("current_byte_offset", &self.current_byte_offset)
            .field("line_number", &self.line_number)
            .field("tokens", &self.tokens)
            .field("has_error", &self.has_error)
            .finish()
    }
}

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // Literals.
    Identifier { identifier: &'a str },
    String { quoted_str: &'a str },
    Number { number: f64 },

    EOF,
}

#[derive(Debug)]
pub struct AnnotatedToken<'a> {
    pub token: Token<'a>,
    pub line_number: usize,
}

impl<'a> ToString for AnnotatedToken<'a> {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

fn is_newline(grapheme: &str) -> bool {
    // Note: can add support for other types of newlines later.
    grapheme == "\n"
}

// Including newline!
fn is_whitespace(grapheme: &str) -> bool {
    grapheme.chars().all(char::is_whitespace)
}

// Digits are just restricted to 0-9.
fn is_digit(grapheme: &str) -> bool {
    // Note: not too sure this first check is necessary, but just in case there
    // are multiple codepoints in the grapheme, or some larger codepoint.
    grapheme.chars().count() == 1
        && grapheme
            .chars()
            .next()
            .map_or(false, |ch| char::is_digit(ch, 10))
}

// Lox accepts alphabetic (unicode) and underscore as the first grapheme of an
// identifier.
fn is_identifier_head(grapheme: &str) -> bool {
    grapheme
        .chars()
        .all(|ch| char::is_alphabetic(ch) || ch == '_')
}

// Lox accepts alphanumeric (unicode) and underscore after the first grapheme. I
// could have restricted the numbers similar to `is_digit`, but why not expand
// it.
fn is_identifier_tail(grapheme: &str) -> bool {
    grapheme
        .chars()
        .all(|ch| char::is_alphanumeric(ch) || ch == '_')
}

fn token_from_identifier(identifier: &str) -> Token {
    match identifier {
        "and" => Token::And,
        "class" => Token::Class,
        "else" => Token::Else,
        "false" => Token::False,
        "for" => Token::For,
        "fun" => Token::Fun,
        "if" => Token::If,
        "nil" => Token::Nil,
        "or" => Token::Or,
        "print" => Token::Print,
        "return" => Token::Return,
        "super" => Token::Super,
        "this" => Token::This,
        "true" => Token::True,
        "var" => Token::Var,
        "while" => Token::While,
        _ => Token::Identifier { identifier },
    }
}
