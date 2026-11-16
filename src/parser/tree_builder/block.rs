use std::panic::RefUnwindSafe;
use crate::parser::tokenizer::directive::Directive;
use crate::parser::tree_builder::postfix_processor::PostfixProcessor;

#[derive(Debug)]
pub struct Block {
    tag: String,
    attributes: Vec<Attr>,
    children: Vec<Block>,
    directives: Vec<Box<dyn Directive + RefUnwindSafe>>
}

impl Block {
    pub fn new(tag: String) -> Block {
        Block {
            tag,
            attributes: Default::default(),
            children: vec![],
            directives: vec![],
        }
    }

    pub fn to_html(&self) -> String {
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
            let chars = attr.value.chars();
            let string = chars.skip(1).take(attr.value.len() - 2).collect::<String>();

            match attr.handle.to_lowercase().as_ref() {
                "text" => content.push(html_escape::encode_double_quoted_attribute(&string).to_string()),
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

        for child in &self.children {
            html.push_str(child.to_html().as_str());
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
pub enum BlockContent {
    Html(String),
    Text(String),
    Block(Block),
}

#[derive(PartialEq, Clone, Debug)]
pub struct BlockBuilder {
    tag: String,
    attributes: Vec<Attr>,
    children: Vec<BlockBuilder>,
}

impl BlockBuilder {
    pub fn new(tag: String) -> BlockBuilder {
        BlockBuilder {
            tag,
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn add_attr(&mut self, attr: Attr) {
        self.attributes.push(attr);
    }

    pub fn add_child(&mut self, child: BlockBuilder) {
        self.children.push(child);
    }

    pub fn build(&mut self) -> Block {
        let mut content: Vec<Block> = vec![];

        for child in &mut self.children {
            content.push(child.build());
        }

        Block {
            tag: std::mem::take(&mut self.tag),
            attributes: std::mem::take(&mut self.attributes),
            children: content,
            directives: vec![],
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Attr {
    handle: String,
    value: String,
}

#[derive(Debug)]
pub struct AttrBuilder<'a> {
    handle: String,
    pub(super) expr_processor: PostfixProcessor<'a>,
}

impl<'a> AttrBuilder<'a> {
    pub fn new(handle: String) -> AttrBuilder<'a> {
        AttrBuilder {
            handle,
            expr_processor: PostfixProcessor::new(),
        }
    }

    pub fn build(&mut self) -> Attr {
        Attr {
            handle: std::mem::take(&mut self.handle),
            // value: self.expr_parser.build_tree().resolve(),
            value: match self.expr_processor.resolve() {
                BlockContent::Html(str) => str,
                BlockContent::Text(str) => str,
                BlockContent::Block(_) => unimplemented!("this shouldn't be called?"),
            },
        }
    }
}
