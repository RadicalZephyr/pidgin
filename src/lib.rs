pub mod prelude {
    pub use crate::Template;
}

pub struct Template(String);

impl Template {
    pub fn from_str(s: impl AsRef<str>) -> Result<Template, ()> {
        Ok(Template(s.as_ref().into()))
    }

    pub fn render<T>(&self) -> String {
        self.0.clone()
    }
}
