

pub fn subtree_regex(name: String) -> String {
    format!(r"(?<{name}>[+*]\(\g<{name}>\)\(\g<{name}>\)|[a-z])")
}