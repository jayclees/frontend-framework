use crate::parser::tokenizer::token::Expr;

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    tag: String,
    attributes: Vec<Attr>,
    html: Option<String>,         // must be one of these three, maybe turn into enum
    text: Option<String>,         // must be one of these three, maybe turn into enum
    children: Vec<Block>, // must be one of these three, maybe turn into enum
}

impl Block {
    pub fn new(tag: String) -> Block {
        Block {
            tag,
            attributes: Default::default(),
            html: None,
            text: None,
            children: vec![],
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct BlockBuilder {
    tag: Option<String>,
    attributes: Vec<Attr>,
    html: Option<String>,
    text: Option<String>,
    children: Option<Vec<Block>>,
}

impl BlockBuilder {
    pub fn new(tag: String) -> BlockBuilder {
        BlockBuilder {
            tag: Some(tag),
            attributes: vec![],
            html: None,
            text: None,
            children: Some(vec![]),
        }
    }

    pub fn build(&mut self) -> Block {
        Block {
            tag: self.tag.take().expect("Tag should be set."),
            attributes: std::mem::take(&mut self.attributes),
            html: self.html.take(),
            text: self.text.take(),
            children: self.children.take().expect("Children should be set."),
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

    pub fn html(&mut self, html: String) -> &mut BlockBuilder {
        self.html = Some(html);
        self
    }

    pub fn text(&mut self, text: String) -> &mut BlockBuilder {
        self.text = Some(text);
        self
    }

    pub fn children(&mut self, children: Vec<Block>) -> &mut BlockBuilder {
        self.children = Some(children);
        self
    }

    pub fn add_attr(&mut self, attr: Attr) {
        self.attributes.push(attr);
    }

    pub fn add_child(&mut self, child: Block) -> &mut BlockBuilder {
        self.children.as_mut().expect("Should be set").push(child);
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
