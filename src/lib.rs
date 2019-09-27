pub mod prelude {
    pub use crate::Template;
}

pub struct Template;

impl Template {
    pub fn from_str(_s: impl AsRef<str>) -> Result<Template, ()> {
        Ok(Template)
    }

    pub fn render<T>(&self) -> String {
        "Hello".into()
    }
}

#[cfg(test)]
mod tests {}
