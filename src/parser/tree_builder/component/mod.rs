use crate::parser::tree_builder::block::Attr;

pub trait ComponentInterface {
    fn id(&self) -> &str;

    fn tag(&self) -> &str {
        "div"
    }

    fn default_attributes() -> Vec<Attr>;

    // Hooks

    fn before_render() -> String;
    fn render() -> String;
    fn after_render() -> String;
}

pub struct AppComponent {}

impl ComponentInterface for AppComponent {
    fn id(&self) -> &str {
        "App"
    }

    fn tag(&self) -> &str {
        "div"
    }

    fn default_attributes() -> Vec<Attr> {
        vec![]
    }

    fn before_render() -> String {
        todo!()
    }

    fn render() -> String {
        todo!()
    }

    fn after_render() -> String {
        todo!()
    }
}
