pub fn is_alpha_num(byte: u8) -> bool {
    (byte >= b'A' && byte <= b'Z') || (byte >= b'a' && byte <= b'z') || ( byte >= b'0' && byte <= b'9')
}

pub fn is_symbol(byte: u8) -> bool {
    [
        b'!',
        b'*',
        b'/',
        b'+',
        b'-',
        b'.',
    ].contains(&byte)
}

pub fn is_underscore(byte: u8) -> bool {
    byte == b'_'
}

pub fn is_alpha_num_or_underscore(byte: Option<u8>) -> bool {
    match byte {
        Some(x) => is_alpha_num(x) || is_underscore(x),
        None => false,
    }
}