use crate::parser::tokenizer::directive::DirectiveEnum;
use crate::parser::tree_builder::postfix_processor::{PostfixProcessor, ValueType};
use crate::parser::tree_builder::value_as_string;
use serde_json::{Value, json};

#[derive(Debug)]
pub struct Block<'token> {
    tag: String,
    attributes: Vec<Attr<'token>>,
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

    pub fn for_directive<'state>(&mut self) -> Option<DirectiveEnum<'token>> {
        for (i, directive) in &mut self.directives.iter().enumerate() {
            if let DirectiveEnum::For(for_directive) = directive {
                return Some(self.directives.remove(i));
            }
        }
        None
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

        let mut for_directive = self.for_directive();
        if let Some(DirectiveEnum::For(for_directive)) = for_directive.as_mut() {
            match for_directive.evaluate(state) {
                ValueType::Reference(Value::Array(arr)) => {
                    for (index, item) in arr.iter().enumerate() {
                        let item_handle = for_directive.item_var();
                        if let Some(index_handle) = for_directive.index_var() {
                            self.render_inner(
                                &mut html,
                                state,
                                is_self_closing,
                                Some((item_handle, item, Some((index_handle, index)))),
                            );
                        } else {
                            self.render_inner(
                                &mut html,
                                state,
                                is_self_closing,
                                Some((item_handle, item, None)),
                            );
                        }
                    }
                }
                ValueType::New(Value::Array(arr)) => {
                    dbg!(arr);
                    unimplemented!("This shouldn't be hit?")
                }
                _ => unimplemented!("Also shouldn't be hit?"),
            }
        } else {
            self.render_inner(&mut html, state, is_self_closing, None);
        }

        html
    }

    fn render_inner(
        &mut self,
        html: &mut String,
        state: &Value,
        is_self_closing: bool,
        for_item: Option<(&String, &Value, Option<(&String, usize)>)>,
    ) {
        // todo handle custom components
        html.push_str(format!("<{}", self.tag).as_str());

        let mut content: Vec<String> = vec![];
        let state2 = match for_item {
            Some(for_item) => {
                Some(match for_item.2 {
                    Some(index) => {
                        json!({
                            for_item.0: for_item.1,
                            index.0: index.1,
                        })
                    }
                    None => json!({for_item.0: for_item.1})
                })
            }
            None => None
        };

        // let state2 = for_item.map(|for_item| {
        //     match for_item.2 {
        //         Some(index) => {
        //             json!({
        //                 for_item.0: for_item.1,
        //                 index.0: index.1,
        //             })
        //         }
        //         None => {
        //             json!({for_item.0: for_item.1})
        //         }
        //     };
        // });

        // Append attributes to string
        for attr in &mut self.attributes {
            let value_type = if let Some(state2) = state2.as_ref() {
                attr.expr_processor.convert().evaluate(vec![state, state2])
            } else {
                attr.expr_processor.convert().evaluate(vec![state])
            };
            let string = match value_type {
                ValueType::Reference(val) => value_as_string(val.to_owned()),
                ValueType::New(val) => value_as_string(val),
            };
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
    attributes: Vec<Attr<'token>>,
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

    pub fn add_attr(&mut self, attr: Attr<'token>) {
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

#[derive(Debug)]
pub struct Attr<'token> {
    handle: String,
    expr_processor: PostfixProcessor<'token>,
}

#[derive(Debug)]
pub struct AttrBuilder<'token> {
    handle: String,
    pub(super) expr_processor: Option<PostfixProcessor<'token>>,
}

impl<'token> AttrBuilder<'token> {
    pub fn new(handle: String) -> AttrBuilder<'token> {
        AttrBuilder {
            handle,
            expr_processor: Some(PostfixProcessor::new()),
        }
    }

    pub fn build(&mut self, state: &Value) -> Attr<'token> {
        Attr {
            handle: std::mem::take(&mut self.handle),
            // expr_processor: PostfixProcessor::new(),
            expr_processor: self.expr_processor.take().unwrap(),
            // value: self.expr_parser.build_tree().resolve(),
            // value: match self.expr_processor.convert().evaluate(state) {
            //     Value::Null => "null".to_owned(),
            //     Value::Bool(bool) => {
            //         if bool {
            //             "true".to_owned()
            //         } else {
            //             "false".to_owned()
            //         }
            //     }
            //     Value::Number(n) => n.to_string(),
            //     Value::String(s) => s,
            //     Value::Array(a) => unimplemented!(),
            //     Value::Object(o) => unimplemented!(),
            // },
        }
    }
}
