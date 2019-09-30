use std::str::FromStr;

use once_cell::unsync::OnceCell;

pub mod prelude {
    pub use crate::{StringTemplate, Template};
}

#[derive(Default)]
pub struct StringTemplate(Vec<Token>);

impl FromTokens for StringTemplate {
    fn from_tokens(tokens: Tokens) -> Self {
        StringTemplate(tokens.collect())
    }
}

pub enum Token {
    Literal(String),
}

impl Token {
    pub fn into_literal(self) -> Option<String> {
        match self {
            Token::Literal(s) => Some(s),
        }
    }
}

pub struct Tokens<'a> {
    raw: &'a str,
    start: usize,
    len: usize,
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.len {
            return None;
        }

        let (next_index, token) = next_token(&self.raw[self.start..self.len]);
        self.start = next_index;
        Some(token)
    }
}

fn next_token(unparsed: &str) -> (usize, Token) {
    (unparsed.len(), Token::Literal(String::from(unparsed)))
}

pub trait FromTokens {
    fn from_tokens(tokens: Tokens) -> Self;
}

pub struct Template<T: FromTokens> {
    raw_template: String,
    t: OnceCell<T>,
}

impl<T> FromStr for Template<T>
where
    T: FromTokens,
{
    type Err = ();

    fn from_str(template: &str) -> Result<Self, Self::Err> {
        let raw_template = String::from(template);
        Ok(Template {
            raw_template,
            t: OnceCell::new(),
        })
    }
}

impl<T: FromTokens> Template<T> {
    pub fn renderer(&self) -> &T {
        self.t.get_or_init(|| {
            let raw = self.raw_template.as_str();
            T::from_tokens(Tokens {
                raw,
                start: 0,
                len: raw.len(),
            })
        })
    }

    pub fn render(&self) -> String {
        self.raw_template.clone()
    }
}
