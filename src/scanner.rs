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
    tokens: Vec<Token<'a>>,
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
    pub fn scan_tokens(&mut self) -> &Vec<Token<'a>> {
        while self.scan_token() {}
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
            "(" => self.add_token(TokenKind::LEFT_PAREN),
            ")" => self.add_token(TokenKind::RIGHT_PAREN),
            "{" => self.add_token(TokenKind::LEFT_BRACE),
            "}" => self.add_token(TokenKind::RIGHT_BRACE),
            "," => self.add_token(TokenKind::COMMA),
            "." => self.add_token(TokenKind::DOT),
            "-" => self.add_token(TokenKind::MINUS),
            "+" => self.add_token(TokenKind::PLUS),
            ";" => self.add_token(TokenKind::SEMICOLON),
            "*" => self.add_token(TokenKind::STAR),
            "!" => {
                if self.advance_if_next_matches("=") {
                    self.add_token(TokenKind::BANG_EQUAL);
                } else {
                    self.add_token(TokenKind::BANG);
                };
            }
            "=" => {
                if self.advance_if_next_matches("=") {
                    self.add_token(TokenKind::EQUAL_EQUAL);
                } else {
                    self.add_token(TokenKind::EQUAL);
                }
            }
            "<" => {
                if self.advance_if_next_matches("=") {
                    self.add_token(TokenKind::LESS_EQUAL);
                } else {
                    self.add_token(TokenKind::LESS);
                }
            }
            "/" => {
                if self.advance_if_next_matches("/") {
                    // Comment
                    while let Some(grapheme) = self.peek() {
                        if !is_newline(grapheme) {
                            // Note: learning Rust, no problem getting a `&mut
                            // self` here because `grapheme`'s lifetime ends
                            // before it.
                            self.advance();
                            // grapheme; // uncommenting this line causes a
                            // double &mut borrow error.
                        }
                    };
                } else {
                    // Division
                    self.add_token(TokenKind::SLASH);
                }
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

    fn advance(&mut self) -> Option<&str> {
        let (byte_offset, grapheme) = self.graphemes_iter.next()?;
        self.current_byte_offset = byte_offset + grapheme.len();
        Some(grapheme)
    }

    fn peek(&mut self) -> Option<&str> {
        let (_, grapheme) = self.graphemes_iter.peek()?;
        Some(grapheme)
    }

    fn advance_if_next_matches(&mut self, to_match: &str) -> bool {
        let matches = self.peek().map_or(false, |next_grapheme| next_grapheme == to_match);
        if matches {
            self.advance();
        }
        matches
    }

    fn add_token(&mut self, kind: TokenKind) {
        self.tokens.push(Token {
            kind,
            lexeme: &self.source[self.token_start_byte_offset..self.current_byte_offset],
            line_number: self.line_number,
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

#[derive(Debug)]
pub enum TokenKind {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

#[derive(Debug)]
pub struct Token<'a> {
    kind: TokenKind,
    lexeme: &'a str,
    // literal: ???
    line_number: usize,
}

impl<'a> ToString for Token<'a> {
    fn to_string(&self) -> String {
        format!(
            "{:?} {} {} {}",
            self.kind, self.lexeme, self.line_number, "TODO: literal"
        )
    }
}

fn is_newline(grapheme: &str) -> bool {
    // Note: can add support for other types of newlines later.
    grapheme == "\n"
}