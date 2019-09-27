use pidgin::prelude::*;

#[test]
fn string_template_literal() {
    let template = Template::from_str("Hello").unwrap();
    assert_eq!(template.render::<String>(), "Hello");
    let template = Template::from_str("World").unwrap();
    assert_eq!(template.render::<String>(), "World");
}
