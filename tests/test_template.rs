use pidgin::{prelude::*, Token, Tokens};

pub struct TestTemplate {
    literals: Vec<String>,
}

impl FromTokens for TestTemplate {
    fn from_tokens(tokens: Tokens) -> Self {
        TestTemplate {
            literals: tokens.filter_map(Token::into_literal).collect(),
        }
    }
}

impl Renderable for TestTemplate {
    type Result = ();

    fn render(&self) -> Self::Result {
        ()
    }
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
