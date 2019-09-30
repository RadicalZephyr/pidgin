use std::str::FromStr;

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

pub struct Tokens;

impl Iterator for Tokens {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

pub trait FromTokens {
    fn from_tokens(tokens: Tokens) -> Self;
}

pub struct Template<T: FromTokens> {
    raw_template: String,
    t: T,
}

impl<T> FromStr for Template<T>
where
    T: FromTokens,
{
    type Err = ();

    fn from_str(template: &str) -> Result<Self, Self::Err> {
        Ok(Template {
            raw_template: template.into(),
            t: T::from_tokens(Tokens),
        })
    }
}

impl<T: FromTokens> Template<T> {
    pub fn renderer(&self) -> &T {
        &self.t
    }

    pub fn render(&self) -> String {
        self.raw_template.clone()
    }
}
