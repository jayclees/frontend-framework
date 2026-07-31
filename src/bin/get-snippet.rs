use std::fs::read_to_string;
use std::str::Chars;

fn main() {
    let string = read_to_string("app/my-page.app").unwrap();
    let chars = string.chars();
    let snippet = chars.skip(66).take(1).collect::<Vec<char>>();
    dbg!(snippet);
}
