use std::rc::Rc;
use super::step::{Simplification, Step, Root};
use super::history::History;

use crate::algo::structures::{
    Tree,
    Identity,
    Expression,
};

use super::match_pattern::replace_identity;

const MAX_DEPTH: usize = 4;

pub struct Strand<'a> {
    pub step: Rc<Step<'a>>,
    pub current: Expression,
}

impl<'a> Strand<'a> {

    /// Initialize an empty strand with a single root node.
    pub fn init<'b>(expression: &'b Expression) -> Strand<'b> {
        Strand {
            step: Rc::new(Step::Root(Root {
                tree: expression
            })),
            current: expression.clone(),
        }
    }

    pub fn propagate<'b, 'c>(self, identities: &'b [Identity], history: &mut History) -> Result<Vec<Strand<'c>>, Strand<'c>> where 'b: 'c, 'a: 'c{
        let mut new_strands = vec![];
        for (_, identity) in identities.iter().enumerate() {
            let new_trees = replace_identity(&self.current, identity).unwrap();
            for tree in new_trees {
                if history.is_new_state(&tree) && self.is_valid_with(&tree){
                    let new_step = Simplification {
                        identity,
                        direction: true,
                        simplicity: tree.simplicity(),
                        parent: Rc::clone(&self.step),
                    };
                    
                    new_strands.push(Strand {
                        step: Rc::new(Step::Simplification(new_step)),
                        current: tree,

                    })
                } // Else simply drop the strand by doing nothing
            }
        };
        if new_strands.len() > 0 {
            return Ok(new_strands);
        }else{
            return Err(self);
        }
    }

    fn is_valid_with(&self, expression: &Expression) -> bool {
        match &*self.step {
            Step::Simplification(simp) => {
                match simp.backtrack(MAX_DEPTH) {
                    Some(step) => {
                        match step {
                            Step::Simplification(back_simp) => {
                                return expression.simplicity() < back_simp.simplicity
                            },
                            Step::Root(root) => {
                                return expression.simplicity() < root.tree.simplicity()
                            }
                        };
                    },
                    None => {
                        return true;
                    },
                };
            },
            Step::Root(_) => {
                return true;
            },
        };
    }

}