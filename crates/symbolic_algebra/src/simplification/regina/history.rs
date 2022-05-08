use crate::algo::{
    structures::Expression,
    regex::Regexable
};

pub struct History {
    pub history: Vec<String>,
}

impl History {

    pub fn new() -> History {
        return History {
            history: vec![],
        }
    }

    pub fn is_new_state(&mut self, expression: &Expression) -> bool {
        let tree_string = (expression.to_regex()).unwrap();
        if self.history.contains(&tree_string) {
            return false;
        }else{
            self.history.push(tree_string);
            return true;
        }
    }

    pub fn len(&self) -> usize {
        self.history.len()
    }
}