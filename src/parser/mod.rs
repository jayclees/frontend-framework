pub mod tokenizer;

use std::collections::HashMap;

struct Block {
    handle: String,
    r#type: BlockType,
    attributes: HashMap<String, String>,
    html: String,         // must be one of these three, maybe turn into enum
    text: String,         // must be one of these three, maybe turn into enum
    children: Vec<Block>, // must be one of these three, maybe turn into enum
}

enum BlockType {
    Page,
    Div,
    Heading,
    P,
    Html(&'static str), // Takes HTML tag as arg
}
