use pidgin::prelude::*;

mod tests {
    use super::*;

    #[test]
    fn simple_literal() {
        let template: Template<StringTemplate> = "Hello".parse().unwrap();
        assert_eq!(template.render(), "Hello");
        let template: Template<StringTemplate> = "World".parse().unwrap();
        assert_eq!(template.render(), "World");
    }
}
