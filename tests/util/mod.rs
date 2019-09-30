#[macro_export]
macro_rules! assert_template_renders_as {
    ($template:expr, $rendered:expr) => {{
        let template: Template<StringTemplate> = $template.parse().unwrap();
        assert_eq!(template.render(), $rendered);
    }};
}
