
pub enum MetaTreeNode {
    Leaf(usize),
    Internal(usize),
}

pub struct MetaTree {
    tokens: Vec<MetaTreeNode>,
}

impl<Idx> std::ops::Index<Idx> for MetaTree where Idx: std::slice::SliceIndex<[MetaTreeNode]> {
    type Output = Idx::Output;
    
    fn index(&self, index: Idx) -> &Self::Output {
        &self.tokens.index(index)
    }
}