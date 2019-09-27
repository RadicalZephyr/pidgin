use std::str::FromStr;

pub mod prelude {
    pub use crate::{StringTemplate, Template};
}

#[derive(Default)]
pub struct StringTemplate;

pub struct Template<T> {
    s: String,
    t: T,
}

impl<T> FromStr for Template<T>
where
    T: Default,
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Template {
            s: s.into(),
            t: T::default(),
        })
    }
}

impl<T> Template<T> {
    pub fn renderer(&self) -> &T {
        &self.t
    }

    pub fn render(&self) -> String {
        self.s.clone()
    }
}
