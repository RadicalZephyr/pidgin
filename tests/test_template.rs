use pidgin::prelude::*;

#[derive(Default)]
pub struct TestTemplate {
    literals: Vec<String>,
}

impl TestTemplate {
    pub fn literals(&self) -> Literals {
        Literals(self.literals.iter().cloned().collect())
    }
}

pub struct Literals(Vec<String>);

impl Iterator for Literals {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.first().cloned()
    }
}

#[test]
fn test_simple_literal() {
    let template: Template<TestTemplate> = "Hello".parse().unwrap();
    let test_template = template.renderer();
    assert_eq!(test_template.literals().nth(0), Some("Hello".into()));
}
