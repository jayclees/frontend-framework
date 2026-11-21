use crate::parser::tokenizer::directive::DirectiveEnum;
use crate::parser::tree_builder::postfix_processor::PostfixProcessor;
use serde_json::Value;

#[derive(Debug)]
pub struct Block<'token> {
    tag: String,
    attributes: Vec<Attr>,
    children: Vec<Block<'token>>,
    directives: Vec<DirectiveEnum<'token>>,
}

impl<'token> Block<'token> {
    pub fn new(tag: String) -> Self {
        Block {
            tag,
            attributes: Default::default(),
            children: vec![],
            directives: vec![],
        }
    }

    pub fn directives(&self) -> &Vec<DirectiveEnum<'token>> {
        &self.directives
    }

    pub fn evaluate_is_true(&mut self, state: &Value) -> bool {
        for directive in &mut self.directives {
            if let DirectiveEnum::If(directive) = directive {
                return directive.evaluate_is_true(state);
            }
        }

        true
    }

    pub fn if_dir_for(&self, state: &Value) {
        for directive in &self.directives {
            if let DirectiveEnum::For(for_directive) = directive {}
        }
    }

    pub fn render_html(&mut self, state: &Value) -> String {
        if !self.evaluate_is_true(state) {
            return "".to_owned();
        }

        let mut html = String::new();
        let is_self_closing = matches!(
            self.tag.to_lowercase().as_ref(),
            "area"
                | "base"
                | "br"
                | "col"
                | "embed"
                | "hr"
                | "img"
                | "input"
                | "link"
                | "meta"
                | "param"
                | "source"
                | "track"
                | "wbr"
        );

        // todo handle custom components
        html.push_str(format!("<{}", self.tag).as_str());

        let mut content: Vec<String> = vec![];

        // Append attributes to string
        for attr in &self.attributes {
            let string = attr.value.to_owned();
            match attr.handle.to_lowercase().as_ref() {
                "text" => {
                    content.push(html_escape::encode_double_quoted_attribute(&string).to_string())
                }
                "html" => content.push(string),
                handle => {
                    html.push_str(
                        format!(
                            r#" {}="{}""#,
                            handle,
                            html_escape::encode_double_quoted_attribute(&string)
                        )
                        .as_ref(),
                    );
                }
            }
        }

        html.push('>');

        for item in content {
            html.push_str(item.as_ref());
        }

        for child in &mut self.children {
            html.push_str(child.render_html(&state).as_str());
        }

        // todo:
        // Create a list or enum of html tags that are self closing, check
        // against it, and close it only if the tag requires it.
        if !is_self_closing {
            html.push_str(format!("</{}>", self.tag).as_str());
        }

        html
    }
}

#[derive(Debug)]
pub enum BlockContent<'token> {
    Html(String),
    Text(String),
    Block(Block<'token>),
}

#[derive(Debug)]
pub struct BlockBuilder<'token> {
    tag: String,
    attributes: Vec<Attr>,
    children: Vec<BlockBuilder<'token>>,
    directives: Vec<DirectiveEnum<'token>>,
}

impl<'token> BlockBuilder<'token> {
    pub fn new(tag: String) -> Self {
        BlockBuilder {
            tag,
            attributes: vec![],
            children: vec![],
            directives: vec![],
        }
    }

    pub fn add_attr(&mut self, attr: Attr) {
        self.attributes.push(attr);
    }

    pub fn add_child(&mut self, child: BlockBuilder<'token>) {
        self.children.push(child);
    }

    pub fn add_directive(&mut self, directive: DirectiveEnum<'token>) {
        self.directives.push(directive);
    }

    pub fn build(&mut self) -> Block<'token> {
        let mut content: Vec<Block> = vec![];

        for child in &mut self.children {
            content.push(child.build());
        }

        Block {
            tag: std::mem::take(&mut self.tag),
            attributes: std::mem::take(&mut self.attributes),
            children: content,
            directives: std::mem::take(&mut self.directives),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Attr {
    handle: String,
    value: String,
}

#[derive(Debug)]
pub struct AttrBuilder<'token> {
    handle: String,
    pub(super) expr_processor: PostfixProcessor<'token>,
}

impl<'token> AttrBuilder<'token> {
    pub fn new(handle: String) -> AttrBuilder<'token> {
        AttrBuilder {
            handle,
            expr_processor: PostfixProcessor::new(),
        }
    }

    pub fn build(&mut self) -> Attr {
        Attr {
            handle: std::mem::take(&mut self.handle),
            // value: self.expr_parser.build_tree().resolve(),
            value: match self.expr_processor.convert().evaluate() {
                Value::Null => "null".to_owned(),
                Value::Bool(bool) => {
                    if bool {
                        "true".to_owned()
                    } else {
                        "false".to_owned()
                    }
                }
                Value::Number(n) => n.to_string(),
                Value::String(s) => s,
                Value::Array(a) => unimplemented!(),
                Value::Object(o) => unimplemented!(),
            },
        }
    }
}
