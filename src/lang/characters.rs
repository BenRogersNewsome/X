pub fn is_alpha(byte: u8) -> bool {
    (byte >= b'A' && byte <= b'Z') || (byte >= b'a' && byte <= b'z')
}

pub fn is_underscore(byte: u8) -> bool {
    byte == b'_'
}

pub fn is_alpha_or_underscore(byte: u8) -> bool {
    is_alpha(byte) || is_underscore(byte)
}