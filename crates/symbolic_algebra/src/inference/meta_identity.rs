use super::meta_tree::MetaTree;

pub struct MetaIdentity (MetaTree, MetaTree);



///
/// 
/// 
/// +(a)(+(a)(b)(*(a)(c))(d))
/// a + (a + b + (a*c) + d)
/// +a+ab*acd
/// 
/// +(a)(+(a)(b)(*(a)(c)(d)))
/// a + (a + b + (a*c*d))
/// +a+ab*acd
/// 