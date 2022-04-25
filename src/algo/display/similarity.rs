use crate::algo::structures::{Tree, Node, Expression};

/// Calculate how similar two expressions are to each other: Lower is better.
/// 
/// Currently the two expressions must have the same number of unique elements - the method will return an error if this is not the case
/// # Arguments
/// * `tree_1` - The tree representing the first expression
/// * `tree_2` - The tree representing the second expression
pub fn similarity(ex_1: &Expression, ex_2: &Expression) -> Result<i32, &'static str> {
    let elements: Vec<Vec<&[u8]>> = [ex_1, ex_2].into_iter().map(|tree|{
        tree.iter().fold(vec![], |mut accumulator: Vec<&[u8]>, token| {
            match token {
                Node::Leaf(e) => {
                    if !accumulator.contains(&&e.label[..]){
                        accumulator.push(&e.label[..])
                    }
                },
                _ => {},
            };
            accumulator
        })
    }).collect();

    if elements[0].len() != elements[1].len() {
        return Err("");
    }

    let mut total = 0;
    for (i, item) in elements[0].iter().enumerate(){
        let position = elements[1].iter().position(|item_2| item_2 == item);
        match position {
            Some(p) => {
                total += ( i as i32 - p as i32 ).abs();
            },
            None => return Err("")
        }
    };
    
    return Ok(total);
}

#[cfg(test)]
mod tests {
    
    use crate::algo::regex::Regexable;
    use crate::algo::structures::Expression;
    use super::similarity;

    #[test]
    fn test_similarity() {
        // ac + ad + bc + bd
        let tree_1 = Expression::from_regex(r"+(+(*(a)(c))(*(a)(d)))(+(*(b)(c))(*(b)(d)))").unwrap();
        // (a + b)(c + d)
        let tree_2 = Expression::from_regex(r"*(+(a)(b))(+(c)(d))").unwrap();
        // (c + d)(a + b)
        let tree_3 = Expression::from_regex(r"*(+(d)(c))(+(b)(a))").unwrap();

        assert_eq!(
            similarity(&tree_1, &tree_2) < similarity(&tree_1, &tree_3),
            true,
        )
    }
}