use super::Node;

use super::Identifier;

pub struct Create {
    structure: Identifier,
    name: Identifier,
}

impl Node for Create {
    fn to_str(&self) -> String {
        String::from("CREATE")
    }
}