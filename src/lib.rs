use std::str::FromStr;

pub mod prelude {
    pub use crate::{StringTemplate, Template};
}

#[derive(Default)]
pub struct StringTemplate;

impl FromTokens for StringTemplate {
    fn from_tokens(_tokens: TemplateTokenStream) -> Self {
        StringTemplate
    }
}

pub struct TemplateTokenStream;

pub trait FromTokens {
    fn from_tokens(tokens: TemplateTokenStream) -> Self;
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
            t: T::from_tokens(TemplateTokenStream),
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
