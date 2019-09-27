use std::{marker::PhantomData, str::FromStr};

pub mod prelude {
    pub use crate::{StringTemplate, Template};
}

pub struct StringTemplate;

pub struct Template<T> {
    s: String,
    _t: PhantomData<T>,
}

impl<T> FromStr for Template<T> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Template {
            s: s.into(),
            _t: PhantomData,
        })
    }
}

impl<T> Template<T> {
    pub fn render(&self) -> String {
        self.s.clone()
    }
}
