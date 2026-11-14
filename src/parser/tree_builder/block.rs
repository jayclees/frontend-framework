use crate::parser::tree_builder::postfix_processor::PostfixProcessor;

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    tag: String,
    attributes: Vec<Attr>,
    children: Vec<Block>,
}

impl Block {
    pub fn new(tag: String) -> Block {
        Block {
            tag,
            attributes: Default::default(),
            children: vec![],
        }
    }

    pub fn to_html(&self) -> String {
        let mut html = String::new();

        // todo handle custom components
        html.push_str(format!("<{}", self.tag).as_str());

        // Append attributes to string
        for attr in &self.attributes {
            html.push_str(
                format!(
                    r#" {}="{}""#,
                    attr.handle,
                    html_escape::encode_double_quoted_attribute(&attr.value)
                )
                .as_ref(),
            );
        }

        html.push('>');

        for child in &self.children {
            html.push_str(child.to_html().as_str());
            // match child {
            //     BlockContent::Html(str) => {
            //         html.push_str(str);
            //     }
            //     BlockContent::Text(str) => {
            //         html.push_str(html_escape::encode_text(str).as_ref());
            //     }
            //     BlockContent::Block(block) => {
            //         html.push_str(block.to_html().as_str());
            //     }
            // }
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
