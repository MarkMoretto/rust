
// Split a string by character length.

use std::str;

fn split_str_by_len(string_input: &str, char_len: usize) -> Vec<&str> {
    let mut output = Vec::with_capacity(string_input.len() / char_len);
    let mut iter = string_input.chars();
    let mut pos = 0;
    
    while pos < string_input.len() {
        let mut len = 0;
        for x in iter.by_ref().take(char_len) {
            len += x.len_utf8();
        }
        output.push(&string_input[pos..pos + len]);
        pos += len;
    }
    output
}
