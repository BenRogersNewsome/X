use zsft::SetElement;
use symbolic_algebra::{self, Tree, TreeNode, Node};

/// A mathematical expression, e.g. `a+2*b`, expressed as a tree in it's pre-traversal representation.
#[derive(PartialEq, Debug, Clone)]
pub struct Expression {
    expression: symbolic_algebra::Expression,
}

pub type ExpressionNode = TreeNode<Expression>;

impl IntoIterator for Expression {
    type Item = <symbolic_algebra::Expression as IntoIterator>::Item;
    type IntoIter = <symbolic_algebra::Expression as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.expression.into_iter()
    }
}

impl Tree for Expression {

    type Leaf = <symbolic_algebra::Expression as Tree>::Leaf;
    type Binary = <symbolic_algebra::Expression as Tree>::Binary;
    type Unary = <symbolic_algebra::Expression as Tree>::Unary;

    fn new<I: IntoIterator<Item = TreeNode<Self>>>(nodes: I) -> Self {
        Self {
            expression: symbolic_algebra::Expression::new(nodes)
        }
    }

    fn iter<'b>(&'b self) -> std::slice::Iter<TreeNode<Self>> {
        self.expression.iter()
    }

    fn join(operator: Self::Binary, tree_1: Self, tree_2: Self) -> Self {
        Self {
            expression: symbolic_algebra::Expression::join(operator, tree_1.expression, tree_2.expression)
        }
    }
}

impl<'a> Expression {
    pub fn new(expression: Vec<TreeNode<symbolic_algebra::Expression>>) -> Self {
        Self {
            expression: symbolic_algebra::Expression::new(expression),
        }
    }

    pub fn to_set_element(&self) -> SetElement {
        let mut elements = self.expression.iter();

        Self::_to_set_element(&mut elements)
    }

    fn _to_set_element<T: Iterator<Item=&'a TreeNode<symbolic_algebra::Expression>>>(elements: &mut T) -> SetElement {
        match elements.next() {
            Some(Node::Leaf(e)) => e.clone(),
            Some(Node::Binary(b)) => {
                let left = Self::_to_set_element(elements);
                let right = Self::_to_set_element(elements);
                SetElement::from_binary_operation(b, &left,&right).unwrap()
            },
            Some(Node::Unary(u)) => {
                todo!();
                // let right = Self::_to_set_element(elements);
                // SetElement::from_unary_operation(u, &right).unwrap()
            },
            None => panic!("Unexpected end of input"),
        }
    }
}

pub struct Equality {
    pub left: Expression,
    pub right: Expression,
}