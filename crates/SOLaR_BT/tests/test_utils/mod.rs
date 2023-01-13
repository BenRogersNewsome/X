use std::ops::Deref;
use solar_bt::{Tree, TreeNode, LeafPattern, NodeSpecification, LeafReplacement};

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    Add,
    Multiply,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Bang,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Element {
    pub label: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Equation {
    nodes: Vec<TreeNode<Self>>,
}

impl IntoIterator for Equation {
    type Item = TreeNode<Self>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}

impl Tree for Equation {
    type Leaf = Element;
    type Binary = BinaryOperator;
    type Unary = UnaryOperator;
    
    fn new<I: IntoIterator<Item = TreeNode<Self>>>(nodes: I) -> Self {
        Self {
            nodes: nodes.into_iter().collect(),
        }
    }

    fn iter(&self) -> std::slice::Iter<TreeNode<Self>> {
        self.nodes.iter()
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum ElementSpec {
    Label(Vec<u8>),
}

impl NodeSpecification<Element> for ElementSpec {
    fn is_match<N: Deref<Target=Element>>(&self, node: N) -> bool {
        match self {
            Self::Label(label) => *label == node.label,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct IdentityPattern {
    nodes: Vec<TreeNode<Self>>,
}

impl IntoIterator for IdentityPattern {
    type Item = TreeNode<Self>;
    type IntoIter = std::vec::IntoIter<TreeNode<Self>>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}

impl Tree for IdentityPattern {
    type Leaf = LeafPattern<ElementSpec>;
    type Binary = BinaryOperator;
    type Unary = UnaryOperator;

    fn new<I: IntoIterator<Item = TreeNode<Self>>>(nodes: I) -> Self {
        Self {
            nodes: nodes.into_iter().collect(),
        }
    }

    fn iter(&self) -> std::slice::Iter<TreeNode<Self>> {
        self.nodes.iter()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct IdentityReplacement {
    nodes: Vec<TreeNode<Self>>,
}

impl IntoIterator for IdentityReplacement {
    type Item = TreeNode<Self>;
    type IntoIter = std::vec::IntoIter<TreeNode<Self>>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}

impl Tree for IdentityReplacement {
    type Leaf = LeafReplacement<Equation>;
    type Binary = BinaryOperator;
    type Unary = UnaryOperator;

    fn new<I: IntoIterator<Item = TreeNode<Self>>>(nodes: I) -> Self {
        Self {
            nodes: nodes.into_iter().collect(),
        }
    }

    fn iter(&self) -> std::slice::Iter<TreeNode<Self>> {
        self.nodes.iter()
    }
}