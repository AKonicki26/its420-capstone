use std::any::Any;
use regex::Regex;

#[derive(Debug)]
pub enum Token {
    Whitespace,
    LineBreak { index: i128 },
    Comment { index: i128, value: String },
    StringLiteral { index: i128, value: String },
    NumberLiteral { index: i128, value: f64 },
    Identifier { index: i128, value: String },

    // keywords / literals
    Null { index: i128 },
    True { index: i128 },
    False { index: i128 },
    Import { index: i128 },
    Export { index: i128 },
    From { index: i128 },
    As { index: i128 },
    For { index: i128 },
    While { index: i128 },
    In { index: i128 },
    Of { index: i128 },
    Break { index: i128 },
    Continue { index: i128 },
    Do { index: i128 },
    If { index: i128 },
    Else { index: i128 },
    Switch { index: i128 },
    Case { index: i128 },
    Default { index: i128 },
    Function { index: i128 },
    Return { index: i128 },
    Yield { index: i128 },
    Await { index: i128 },
    Try { index: i128 },
    Catch { index: i128 },
    Finally { index: i128 },
    Throw { index: i128 },
    New { index: i128 },
    Class { index: i128 },
    Super { index: i128 },
    Let { index: i128 },
    Const { index: i128 },
    This { index: i128 },

    // operators & symbols
    LBrace { index: i128 },
    RBrace { index: i128 },
    LBracket { index: i128 },
    RBracket { index: i128 },
    LParen { index: i128 },
    RParen { index: i128 },
    Semicolon { index: i128 },
    Colon { index: i128 },
    Comma { index: i128 },
    Spread { index: i128 },
    Dot { index: i128 },
    Pow { index: i128 },
    Star { index: i128 },
    TripleEq { index: i128 },
    DoubleEq { index: i128 },
    Arrow { index: i128 },
    Eq { index: i128 },
    NotTripleEq { index: i128 },
    NotEq { index: i128 },
    AndAnd { index: i128 },
    And { index: i128 },
    Caret { index: i128 },
    Tilde { index: i128 },
    Bang { index: i128 },
    OrOr { index: i128 },
    Or { index: i128 },
    PlusPlus { index: i128 },
    Plus { index: i128 },
    MinusMinus { index: i128 },
    Minus { index: i128 },
    Backslash { index: i128 },
    Percent { index: i128 },
    QMarkQMark { index: i128 },
    QMark { index: i128 },
    Gte { index: i128 },
    Lte { index: i128 },
    RShift { index: i128 },
    Gt { index: i128 },
    LShift { index: i128 },
    Lt { index: i128 },
}

trait StringExt {
    fn remove_first_and_last(self) -> String;
}
impl StringExt for String {
    fn remove_first_and_last(self) -> String {
        let mut chars = self.chars();
        chars.next();
        chars.next_back();
        chars.collect()
    }
}

macro_rules! token {

    // Version for when no variables are used
    ($regex: expr, |_, _| $body:expr) => {
        (Regex::new($regex).unwrap(), |_, _| $body)
    };

    // Version for when only one variable is used
    ($regex: expr, |$index:ident, _| $body:expr) => {
        (Regex::new($regex).unwrap(), |$index: i128, _| $body)
    };

    // Version for when both variables are used
    ($regex:expr, |$index:ident, $val:ident| $body:expr) => {
        (Regex::new($regex).unwrap(), |$index: i128, $val: String| $body)
    };


}

lazy_static::lazy_static! {
    pub static ref TOKEN_CONVERTERS: Vec<(Regex, fn(i128, String) -> Token)> = vec![
        token!(r"[ \t]+", |_, _| Token::Whitespace),
        token!(r"\r?\n", |index, _| Token::LineBreak { index }),
        token!(r"//(.*?)(?=\r?\n|$)", |index, val| Token::Comment { index, value: val[2..].to_string() }),
        token!(r#""[^"\r\n]+""#, |index, val| Token::StringLiteral { index, value: val.remove_first_and_last() }),
        token!(r#"'[^'\r\n]+'"#, |index, val| Token::StringLiteral { index, value: val.remove_first_and_last() }),
        token!(r#"`[^`]+`"#, |index, val| Token::StringLiteral { index, value: val.remove_first_and_last() }),
        token!(r"-?[0-9]+\.?[0-9]*(?![a-zA-Z$_])", |index, val| Token::NumberLiteral { index, value: val.parse::<f64>().unwrap() }),

        // punctuation / symbols
        token!(r"\{", |index, _| Token::LBrace { index }),
        token!(r"\}", |index, _| Token::RBrace { index }),
        token!(r"\[", |index, _| Token::LBracket { index }),
        token!(r"\]", |index, _| Token::RBracket { index }),
        token!(r"\(", |index, _| Token::LParen { index }),
        token!(r"\)", |index, _| Token::RParen { index }),
        token!(r";", |index, _| Token::Semicolon { index }),
        token!(r":", |index, _| Token::Colon { index }),
        token!(r",", |index, _| Token::Comma { index }),
        token!(r"\.\.\.", |index, _| Token::Spread { index }),
        token!(r"\.", |index, _| Token::Dot { index }),
        token!(r"\*\*", |index, _| Token::Pow { index }),
        token!(r"\*", |index, _| Token::Star { index }),
        token!(r"===", |index, _| Token::TripleEq { index }),
        token!(r"==", |index, _| Token::DoubleEq { index }),
        token!(r"=>", |index, _| Token::Arrow { index }),
        token!(r"=", |index, _| Token::Eq { index }),
        token!(r"!==", |index, _| Token::NotTripleEq { index }),
        token!(r"!=", |index, _| Token::NotEq { index }),
        token!(r"&&", |index, _| Token::AndAnd { index }),
        token!(r"&", |index, _| Token::And { index }),
        token!(r"\^", |index, _| Token::Caret { index }),
        token!(r"~", |index, _| Token::Tilde { index }),
        token!(r"!", |index, _| Token::Bang { index }),
        token!(r"\|\|", |index, _| Token::OrOr { index }),
        token!(r"\|", |index, _| Token::Or { index }),
        token!(r"\+\+", |index, _| Token::PlusPlus { index }),
        token!(r"\+", |index, _| Token::Plus { index }),
        token!(r"\-\-", |index, _| Token::MinusMinus { index }),
        token!(r"\-", |index, _| Token::Minus { index }),
        token!(r"\\", |index, _| Token::Backslash { index }),
        token!(r"%", |index, _| Token::Percent { index }),
        token!(r"\?\?", |index, _| Token::QMarkQMark { index }),
        token!(r"\?", |index, _| Token::QMark { index }),
        token!(r">=", |index, _| Token::Gte { index }),
        token!(r"<=", |index, _| Token::Lte { index }),
        token!(r">>", |index, _| Token::RShift { index }),
        token!(r">", |index, _| Token::Gt { index }),
        token!(r"<<", |index, _| Token::LShift { index }),
        token!(r"<", |index, _| Token::Lt { index }),

        // keywords / literals
        token!(r"null", |index, _| Token::Null { index }),
        token!(r"true", |index, _| Token::True { index }),
        token!(r"false", |index, _| Token::False { index }),
        token!(r"import", |index, _| Token::Import { index }),
        token!(r"export", |index, _| Token::Export { index }),
        token!(r"from", |index, _| Token::From { index }),
        token!(r"as", |index, _| Token::As { index }),
        token!(r"for", |index, _| Token::For { index }),
        token!(r"while", |index, _| Token::While { index }),
        token!(r"in", |index, _| Token::In { index }),
        token!(r"of", |index, _| Token::Of { index }),
        token!(r"break", |index, _| Token::Break { index }),
        token!(r"continue", |index, _| Token::Continue { index }),
        token!(r"do", |index, _| Token::Do { index }),
        token!(r"if", |index, _| Token::If { index }),
        token!(r"else", |index, _| Token::Else { index }),
        token!(r"switch", |index, _| Token::Switch { index }),
        token!(r"case", |index, _| Token::Case { index }),
        token!(r"default", |index, _| Token::Default { index }),
        token!(r"function", |index, _| Token::Function { index }),
        token!(r"return", |index, _| Token::Return { index }),
        token!(r"yield", |index, _| Token::Yield { index }),
        token!(r"await", |index, _| Token::Await { index }),
        token!(r"try", |index, _| Token::Try { index }),
        token!(r"catch", |index, _| Token::Catch { index }),
        token!(r"finally", |index, _| Token::Finally { index }),
        token!(r"throw", |index, _| Token::Throw { index }),
        token!(r"new", |index, _| Token::New { index }),
        token!(r"class", |index, _| Token::Class { index }),
        token!(r"super", |index, _| Token::Super { index }),
        token!(r"let", |index, _| Token::Let { index }),
        token!(r"const", |index, _| Token::Const { index }),
        token!(r"this", |index, _| Token::This { index }),

        // generic identifier last (catch-all)
        token!(r"[a-zA-Z$_][a-zA-Z0-9$_]*", |index, val| Token::Identifier { index, value: val }),
    ];
}


pub struct Tokenizer {



}

impl Tokenizer {
    pub(crate) fn tokenize(input: &str) -> Vec<Token> {
        let mut index = 0;
        let mut tokens: Vec<Token> = vec![];

        while (index < input.len()) {
            let has_match = false;

            for (regex, creator) in TOKEN_CONVERTERS.iter() {
                todo!("")


            }
        }

        tokens
    }
}