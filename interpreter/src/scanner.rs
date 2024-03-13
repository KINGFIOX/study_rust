use std::collections::HashMap;
use std::fmt;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,

    LeftBrace,
    RightBrace,

    // 左 方括号
    LeftBracket,
    RightBracket,

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

    // Literals.
    Identifier, // 标识符
    String,
    Number,

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
    Lambda,

    Eof,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Identifier(String), // 字面量的枚举类型，附带有 字段
    Str(String),
    Number(f64),
}

#[derive(Clone)]
pub struct Token {
    pub ty: TokenType,
    pub lexeme: Vec<u8>, // 词素
    pub literal: Option<Literal>,
    pub line: usize,
    pub col: i64,
}

/**
 * 用于打印 token 的
 */
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Token {{ ty: {:?}, lexeme: \"{}\", literal: {:?}, line: {:?}, col: {:?}}}",
            self.ty,
            String::from_utf8(self.lexeme.clone()).unwrap(),
            self.literal,
            self.line,
            self.col
        )
    }
}

/**
 *
 */
pub fn scan_tokens(input: String) -> Result<Vec<Token>, Error> {
    let mut scanner: Scanner = Default::default();

    scanner.scan_tokens(input);

    match scanner.err {
        Some(err) => Err(err),
        None => Ok(scanner.tokens),
    }
}

#[derive(Debug)]
pub struct Error {
    pub what: String,
    pub line: usize,
    pub col: i64,
}

struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
    err: Option<Error>,
    start: usize,
    current: usize,
    line: usize,
    col: i64,
    keywords: HashMap<String, TokenType>,
}

/**
 * Scanner 的默认构造
 */
impl Default for Scanner {
    fn default() -> Scanner {
        Scanner {
            source: Vec::new(),
            tokens: Vec::new(),
            err: None,
            start: 0,
            current: 0,
            line: 1,
            col: -1,
            keywords: vec![
                ("and", TokenType::And),
                ("class", TokenType::Class),
                ("else", TokenType::Else),
                ("false", TokenType::False),
                ("for", TokenType::For),
                ("fun", TokenType::Fun),
                ("if", TokenType::If),
                ("nil", TokenType::Nil),
                ("or", TokenType::Or),
                ("print", TokenType::Print),
                ("return", TokenType::Return),
                ("super", TokenType::Super),
                ("this", TokenType::This),
                ("true", TokenType::True),
                ("var", TokenType::Var),
                ("while", TokenType::While),
                ("lambda", TokenType::Lambda)
            ]
                .into_iter()
                .map(|(k, v)| (String::from(k), v))
                .collect(), // 关键字对应的字符串 与 Token 进行对应
        }
    }
}

impl Scanner {
    fn scan_tokens(&mut self, input: String) {
        self.source = input.into_bytes();

        while !self.done() {
            self.start = self.current;
            self.scan_token();
        }

        match self.err {
            Some(_) => {} // 如果有错误，啥也不干
            None =>
                self.tokens.push(Token { // 添加一个哨兵
                    ty: TokenType::Eof,
                    lexeme: Vec::new(),
                    literal: None,
                    line: self.line,
                    col: self.col,
                }),
        }
    }

    /**
     * 前进一步，并返回上一个字符
     */
    fn advance(&mut self) -> char {
        self.current += 1;
        self.col += 1;

        char::from(self.source[self.current - 1])
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            '[' => self.add_token(TokenType::LeftBracket),
            ']' => self.add_token(TokenType::RightBracket),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),

            // 可能会有两个字符的 符号
            '!' => {
                let matches_eq = self.matches('=');
                self.add_token(if matches_eq { TokenType::BangEqual } else { TokenType::Bang })
            }
            '=' => {
                let matches_eq = self.matches('=');
                self.add_token(if matches_eq { TokenType::EqualEqual } else { TokenType::Equal })
            }
            '<' => {
                let matches_eq = self.matches('=');
                self.add_token(if matches_eq { TokenType::LessEqual } else { TokenType::Less })
            }
            '>' => {
                let matches_eq = self.matches('=');
                self.add_token(
                    if matches_eq {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    }
                )
            }
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => {} // 空白字符，跳过
            '\n' => {
                self.line += 1;
                self.col = 0;
            }
            '"' => self.string(), // 如果出现了 引号，那么获取 字符串
            _ => {
                // 如果不是符号
                if Scanner::is_decimal_digit(c) {
                    self.number()
                } else if Scanner::is_alpha(c) {
                    self.identifier()
                } else {
                    self.err = Some(Error {
                        what: format!("scanner can't handle {}", c),
                        line: self.line,
                        col: self.col,
                    });
                }
            }
        }
    }

    fn is_alpha(c: char) -> bool {
        c.is_alphabetic()
    }

    fn is_decimal_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_alphanumeric(c: char) -> bool {
        Scanner::is_alpha(c) || Scanner::is_decimal_digit(c)
    }

    /**
     *
     */
    fn identifier(&mut self) {
        // 前进，直到不是 字母
        while Scanner::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let literal_val = String::from_utf8(
            self.source[self.start..self.current].to_vec()
        ).unwrap();

        let token_type = match self.keywords.get(&literal_val) {
            Some(kw_token_type) => *kw_token_type,
            None => TokenType::Identifier, // 如果不是 keyword，那么就是变量名之类的标识符
        };

        match token_type {
            TokenType::Identifier =>
                self.add_token_literal(
                    TokenType::Identifier,
                    Some(Literal::Identifier(literal_val))
                ), // book doesn't do this. why not?}
            _ => self.add_token(token_type),
        }
    }

    fn number(&mut self) {
        while Scanner::is_decimal_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && Scanner::is_decimal_digit(self.peek_next()) {
            self.advance();
        }

        while Scanner::is_decimal_digit(self.peek()) {
            self.advance();
        }

        let val: f64 = String::from_utf8(self.source[self.start..self.current].to_vec())
            .unwrap()
            .parse()
            .unwrap();

        self.add_token_literal(TokenType::Number, Some(Literal::Number(val)))
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        // 如果没有 右边的引号，那么有问题
        if self.is_at_end() {
            self.err = Some(Error {
                what: "Unterminated string".to_string(),
                line: self.line,
                col: self.col,
            });
        }

        assert!(self.peek() == '"');

        self.advance(); // 跨过 右边的 "

        self.add_token_literal(
            TokenType::String,
            Some(
                Literal::Str(
                    String::from_utf8(
                        self.source[self.start + 1..self.current - 1].to_vec()
                    ).unwrap()
                )
            )
        )
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            char::from(self.source[self.current + 1])
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() { '\0' } else { char::from(self.source[self.current]) }
    }

    /**
     * 如果匹配，那么跳到下一个字符
     */
    fn matches(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return true;
        }

        if char::from(self.source[self.current]) != c {
            false
        } else {
            self.current += 1;
            self.col += 1;
            true
        }
    }

    /**
     * 添加 Token
     */
    fn add_token(&mut self, token_type: TokenType) {
        let text = self.source[self.start..self.current].to_vec();
        self.tokens.push(Token {
            ty: token_type,
            lexeme: text,
            literal: None,
            line: self.line,
            col: self.col,
        })
    }

    /**
     * 将 literal 添加 TOken 到表中
     */
    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = self.source[self.start..self.current].to_vec();

        self.tokens.push(Token {
            ty: token_type,
            lexeme: text,
            literal,
            line: self.line,
            col: self.col,
        })
    }

    /**
     * 到达文件尾部 或者是 有错误
     */
    fn done(&self) -> bool {
        self.err.is_some() || self.is_at_end()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
