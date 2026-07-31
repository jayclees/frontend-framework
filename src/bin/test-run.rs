use std::fs::read_to_string;

fn main() {
    let chars = read_to_string("app/my-page.app").unwrap().chars();
    let snippet = chars[..];
}
