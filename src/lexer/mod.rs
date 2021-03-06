use regex::Regex;
use errors::LexingError;
use codemap::{Span, Spanned};

mod token;
pub use self::token::Token;

lazy_static! {
    static ref WHITESPACES: Regex = Regex::new(r"^\s+").unwrap();
    static ref LINE_COMMENT: Regex = Regex::new(r"^//.*").unwrap();
    static ref LINE_PP_COMMENT: Regex = Regex::new(r"^#.*").unwrap();
    static ref BLOCK_COMMENT: Regex = Regex::new(r"^/\*(.|[\r\n])*?\*/").unwrap();
    static ref SKIPABLE: Vec<&'static Regex> = vec![
        &WHITESPACES, &LINE_COMMENT, &LINE_PP_COMMENT, &BLOCK_COMMENT
    ];

    static ref IDENTIFIER_REGEX: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
    static ref INTEGER_REGEX: Regex = Regex::new(r"^[0-9]+").unwrap();
    static ref DOUBLE_REGEX: Regex = Regex::new(r"^[0-9]*\.[0-9]+([eE][+-]?[0-9]+)?").unwrap();
    static ref STRING_REGEX: Regex = Regex::new(r##"^"(([^"]|\\")*[^\\])?""##).unwrap();
}

pub struct Lexer<'input> {
    input: &'input str,
    pos: usize,
    buffer: Option<Spanned<Token<'input>>>,
}

pub type LexingResult<T> = Result<T, Spanned<LexingError>>;

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            input,
            pos: 0,
            buffer: None,
        }
    }

    pub fn skip_whitespaces(&mut self) {
        'main_loop: loop {
            for regex in SKIPABLE.iter() {
                if let Some(m) = regex.find(&self.input[self.pos..]) {
                    self.pos += m.end();
                    continue 'main_loop;
                }
            }
            break;
        }
    }

    fn match_regex(&mut self, regex: &Regex) -> Option<&'input str> {
        regex.find(&self.input[self.pos..]).map(|m| {
            let new_pos = self.pos + m.end();
            let token_str = &self.input[self.pos..new_pos];
            self.pos = new_pos;
            token_str
        })
    }

    pub fn peek_token(&mut self) -> LexingResult<&Spanned<Token<'input>>> {
        if self.buffer.is_none() {
            self.buffer = Some(self.next_token()?);
        }

        if let Some(ref tok) = self.buffer {
            Ok(tok)
        } else {
            unreachable!()
        }
    }

    pub fn next_token(&mut self) -> LexingResult<Spanned<Token<'input>>> {
        macro_rules! match_literal {
            ($lexer:expr; $literal:tt => $ret_expr:expr) => {
                if (&$lexer.input[$lexer.pos..]).starts_with($literal) {
                    let len = $literal.len();
                    let start = $lexer.pos;
                    $lexer.pos += len;
                    return Ok(Spanned::new($ret_expr, Span::new_with_len(start, len)))
                }
            }
        }

        if let Some(token) = self.buffer.take() {
            return Ok(token);
        }

        self.skip_whitespaces();

        if self.pos >= self.input.len() {
            return Ok(Spanned::new(
                Token::EOF,
                Span::new_with_len(self.input.len(), 1),
            ));
        }

        match_literal!(self; "..." => Token::DotDotDot);
        match_literal!(self; "(" => Token::LeftParenthesis);
        match_literal!(self; ")" => Token::RightParenthesis);
        match_literal!(self; "{" => Token::LeftBracket);
        match_literal!(self; "}" => Token::RightBracket);
        match_literal!(self; "[" => Token::LeftSquare);
        match_literal!(self; "]" => Token::RightSquare);
        match_literal!(self; ";" => Token::SemiColon);
        match_literal!(self; ":" => Token::Colon);
        match_literal!(self; "," => Token::Comma);
        match_literal!(self; "." => Token::Dot);
        match_literal!(self; "->" => Token::Arrow);

        match_literal!(self; "==" => Token::EqualEqual);
        match_literal!(self; "!=" => Token::BangEqual);
        match_literal!(self; "++" => Token::PlusPlus);
        match_literal!(self; "--" => Token::MinusMinus);
        match_literal!(self; "<=" => Token::LessEqual);
        match_literal!(self; ">=" => Token::GreaterEqual);
        match_literal!(self; "||" => Token::PipePipe);
        match_literal!(self; "&&" => Token::AmpAmp);
        match_literal!(self; "&" => Token::Amp);

        match_literal!(self; "=" => Token::Equal);
        match_literal!(self; "+" => Token::Plus);
        match_literal!(self; "-" => Token::Minus);
        match_literal!(self; "*" => Token::Star);
        match_literal!(self; "/" => Token::Slash);
        match_literal!(self; "%" => Token::Percent);
        match_literal!(self; "<" => Token::Less);
        match_literal!(self; ">" => Token::Greater);
        match_literal!(self; "!" => Token::Bang);

        let start_pos = self.pos;
        if let Some(s) = self.match_regex(&IDENTIFIER_REGEX) {
            let len = s.len();
            let token = match s {
                "extern" => Token::ExternKeyword,
                "while" => Token::WhileKeyword,
                "for" => Token::ForKeyword,
                "if" => Token::IfKeyword,
                "else" => Token::ElseKeyword,
                "return" => Token::ReturnKeyword,
                "true" => Token::BooleanLiteral(true),
                "false" => Token::BooleanLiteral(false),
                "continue" => Token::ContinueKeyword,
                "break" => Token::BreakKeyword,
                "struct" => Token::StructKeyword,
                "as" => Token::AsKeyword,
                "fn" => Token::FnKeyword,
                "let" => Token::LetKeyword,
                "nullptr" => Token::NullptrKeyword,
                s => {
                    if s.starts_with("___") {
                        return Err(Spanned::new(
                            LexingError::ReservedIdentifier(s.to_string()),
                            Span::new_with_len(start_pos, len),
                        ));
                    }
                    Token::Identifier(s)
                }
            };
            return Ok(Spanned::new(token, Span::new_with_len(start_pos, len)));
        }
        if let Some(s) = self.match_regex(&DOUBLE_REGEX) {
            let len = s.len();
            let number = if let Ok(n) = s.parse() {
                n
            } else {
                return Err(Spanned::new(
                    LexingError::UnparsableNumber,
                    Span::new_with_len(start_pos, len),
                ));
            };
            let token = Token::DoubleLiteral(number);
            return Ok(Spanned::new(token, Span::new_with_len(start_pos, len)));
        }
        if let Some(s) = self.match_regex(&INTEGER_REGEX) {
            let len = s.len();
            let number = if let Ok(n) = s.parse() {
                n
            } else {
                return Err(Spanned::new(
                    LexingError::UnparsableNumber,
                    Span::new_with_len(start_pos, len),
                ));
            };
            let token = Token::IntegerLiteral(number);
            return Ok(Spanned::new(token, Span::new_with_len(start_pos, len)));
        }
        if let Some(s) = self.match_regex(&STRING_REGEX) {
            let len = s.len();
            let token = Token::StringLiteral(s);
            return Ok(Spanned::new(token, Span::new_with_len(start_pos, len)));
        }

        Err(Spanned::new(
            LexingError::UnknownChar((&self.input[self.pos..]).chars().next().unwrap()),
            Span::new_one(self.pos),
        ))
    }
}
