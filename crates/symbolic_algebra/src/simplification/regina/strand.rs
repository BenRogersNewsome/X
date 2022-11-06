use std::rc::Rc;
use super::step::{Simplification, Step};
use super::history::History;

use crate::manipulation::{Strand, Manipulatable};



/// Given a set of identities, I, generate a new set of strands, each formed by applying a single identity from the set I to the current strand.
/// 
/// # Arguments
/// 
/// * `identities` - An algebra to use for the propagation
/// * `History` - A string encoded list of previously found expressions to exclude (to prevent infinite loops)
/// 
/// # Returns
/// 
/// `Vec<Strand>` - A list of new strands.
pub fn propagate_strand<'a, 'c, T: Manipulatable>(strand: Strand<'a, T>, identities: &[&T::Instruction], history: &mut History) -> Vec<Strand<'a, T>>{
    let mut new_strands = vec![];
    for (_, identity) in identities.iter().enumerate() {
        let new_trees = replace_identity(&self.current, &identity).unwrap();
        for (tree, position) in new_trees {
            if !self.is_valid_with(&tree) {
            }else{
                println!("Valid");
                println!("{}", tree.simplicity());
            }
            if history.is_new_state(&tree) && self.is_valid_with(&tree){
                let new_step = Simplification {
                    identity: &identity,
                    direction: true,
                    position,
                    simplicity: tree.simplicity(),
                    parent: Rc::clone(&self.step),
                };
                let simplicity = match tree.simplicity() {
                    x if self.simplify == (x <= self.current.simplicity()) => x,
                    _ => tree.simplicity(),
                };
                new_strands.push(Strand {
                    step: Rc::new(Step::Simplification(new_step)),
                    current: tree,
                    simplify: self.simplify,
                    simplicity,
                })
            } // Else simply drop the strand by doing nothing
        }
    };
    if new_strands.len() == 0 {
        new_strands.push(self)
    }
    return new_strands;
}

fn is_valid_with(&self, expression: &Expression) -> bool {
    match &*self.step {
        Step::Simplification(simp) => {
            match simp.backtrack(MAX_DEPTH) {
                Some(step) => {
                    match &*step {
                        Step::Simplification(back_simp) => {
                            return (expression.simplicity() < back_simp.simplicity) == self.simplify
                        },
                        Step::Root(root) => {
                            return (expression.simplicity() < root.tree.simplicity()) == self.simplify
                        }
                    };
                },
                None => {
                    return true;
                },
            };
        },
        Step::Root(r) => {
            return (expression.simplicity() < r.tree.simplicity()) == self.simplify;
        },
    };
}

pub fn trim(mut self) -> Self {
    let mut step = self.step;
    loop {
        match &*step {
            Step::Simplification(s) => {
                if s.simplicity == self.simplicity {
                    self.step = step;
                    return self;
                }else{
                    // Unsafe, but we know that we will always have a parent 1 element back
                    step = Rc::clone(&s.parent);
                    continue;
                }
            },
            _ => {
                self.step = step;
                return self;
            }
        }
    }
}