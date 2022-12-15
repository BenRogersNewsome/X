use crate::{Tree, Node, TreeNode, PatternLeaf, tree::ParsableTreeNode};

#[derive(Debug, PartialEq)]
pub enum Operator {
    ADD,
    MULTIPLY,
}

impl TreeNode for Operator {
    fn to_string(&self) -> String {
        match self {
            Self::ADD => String::from("+"),
            Self::MULTIPLY => String::from("*"),
        }
    }
}

impl ParsableTreeNode for Operator {
    fn from_string(id: &str) -> Result<Self, &'static str> {
        match id {
            "+" => Ok(Self::ADD),
            "*" => Ok(Self::MULTIPLY),
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Element {
    pub label: Vec<u8>,
}

impl TreeNode for Element {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.label).to_string()
    }
}

#[derive(Debug, PartialEq)]
pub struct Equation {
    tokens: Vec<Node<Element, Operator>>,
}

impl ParsableTreeNode for Element {
    fn from_string(id: &str) -> Result<Self, &'static str> {
        Ok(Self {
            label: id.as_bytes().to_owned(),
        })
    }
}

impl Tree for Equation {
    type L = Element;
    type I = Operator;
    
    fn new(tokens: Vec<Node<Self::L, Self::I>>) -> Self {
        Self {
            tokens
        }
    }

    fn iter(&self) -> std::slice::Iter<Node<Self::L, Self::I>> {
        self.tokens.iter()
    }

    fn tokens(&self) -> &Vec<Node<Self::L, Self::I>> {
        &self.tokens
    }

    fn tokens_mut<'a>(&'a mut self) -> &'a mut Vec<Node<Self::L, Self::I>> {
        &mut self.tokens
    }

    fn into_tokens(self) -> Vec<Node<Self::L, Self::I>> {
        self.tokens
    }
}

pub struct Identity {
    tokens: Vec<Node<PatternLeaf<Element>, Operator>>,
}

impl Tree for Identity {
    type I = Operator;
    type L = PatternLeaf<Element>;

    fn new(tokens: Vec<Node<Self::L, Self::I>>) -> Self {
        Self {
            tokens
        }
    }

    fn iter(&self) -> std::slice::Iter<Node<Self::L, Self::I>> {
        self.tokens.iter()
    }

    fn tokens(&self) -> &Vec<Node<Self::L, Self::I>> {
        &self.tokens
    }

    fn tokens_mut<'a>(&'a mut self) -> &'a mut Vec<Node<Self::L, Self::I>> {
        &mut self.tokens
    }

    fn into_tokens(self) -> Vec<Node<Self::L, Self::I>> {
        self.tokens
    }
}