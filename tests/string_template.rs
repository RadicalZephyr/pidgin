use pidgin::prelude::*;

mod util;

mod tests {
    use super::*;

    #[test]
    fn simple_literal() {
        assert_template_renders_as!("Hello", "Hello");
        assert_template_renders_as!("World", "World");
    }
}
