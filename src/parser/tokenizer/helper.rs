pub(super) fn is_valid_var_start(char: char) -> bool {
    char.is_ascii_alphabetic() || char == '_'
}

pub(super) fn is_valid_var_char(char: char) -> bool {
    char.is_ascii_alphanumeric() || char == '_'
}
