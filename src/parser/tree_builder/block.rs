use crate::parser::tokenizer::token::Expr;

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    tag: String,
    attributes: Vec<Attr>,
    content: Vec<BlockContent>,
}

impl Block {
    pub fn new(tag: String) -> Block {
        Block {
            tag,
            attributes: Default::default(),
            content: vec![],
        }
    }

    pub fn to_html(&self) -> String {
        let mut html = String::new();

        html.push_str(format!("<{}>", self.tag).as_str());

        for child in &self.content {
            match child {
                BlockContent::Html(str) => {
                    html.push_str(str);
                }
                BlockContent::Text(str) => {
                    html.push_str(html_escape::encode_text(str).as_ref());
                }
                BlockContent::Block(block) => {
                    html.push_str(block.to_html().as_str());
                }
            }
        }

        // todo:
        // Create a list or enum of html tags that are self closing, check
        // against it, and close it only if the tag requires it.
        html.push_str(format!("</{}>", self.tag).as_str());

        html
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BlockContent {
    Html(String),
    Text(String),
    Block(Block),
}

#[derive(PartialEq, Clone)]
pub struct BlockBuilder {
    tag: Option<String>,
    attributes: Vec<Attr>,
    content: Vec<BlockContent>,
}

impl BlockBuilder {
    pub fn new(tag: String) -> BlockBuilder {
        BlockBuilder {
            tag: Some(tag),
            attributes: vec![],
            content: vec![],
        }
    }

    pub fn build(&mut self) -> Block {
        Block {
            tag: self.tag.take().expect("Tag should be set."),
            attributes: std::mem::take(&mut self.attributes),
            content: std::mem::take(&mut self.content),
        }
    }

    pub fn handle(&mut self, handle: String) -> &mut BlockBuilder {
        self.tag = Some(handle);
        self
    }

    pub fn attributes(&mut self, attributes: Vec<Attr>) -> &mut BlockBuilder {
        self.attributes = attributes;
        self
    }

    pub fn add_attr(&mut self, attr: Attr) {
        self.attributes.push(attr);
    }

    pub fn add_content(&mut self, content: BlockContent) -> &mut BlockBuilder {
        self.content.push(content);
        self
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Attr {
    handle: String,
    expression: Vec<Expr>,
}

#[derive(PartialEq, Clone)]
pub struct AttrBuilder {
    handle: Option<String>,
    expression: Vec<Expr>,
}

impl AttrBuilder {
    pub fn new(handle: String) -> AttrBuilder {
        AttrBuilder {
            handle: Some(handle),
            expression: vec![],
        }
    }

    pub fn append_expr(&mut self, expr: Expr) {
        self.expression.push(expr);
    }

    pub fn build(&mut self) -> Attr {
        let mut expression = vec![];
        expression.append(&mut self.expression);
        Attr {
            handle: self.handle.take().expect("Should be set"),
            expression,
        }
    }
}
