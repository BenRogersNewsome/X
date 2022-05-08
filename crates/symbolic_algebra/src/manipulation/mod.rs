use std::rc::Rc;
use crate::algo::structures::Expression;

use crate::algo::patterns::replace_identity;

mod manipulation;
mod step;
mod strand;
// mod procedure;

pub use manipulation::Manipulation;
pub use step::Step;
pub use strand::Strand;

use super::structures::Identity;

pub trait Manipulatable where Self: Sized {
    fn manipulate(&self, identity: &Identity, position: usize) -> Result<Self, &'static str>;
    fn try_manipulate(&self, identity: &Identity) -> Vec<Self>;
}

impl Manipulatable for Expression {

    fn manipulate(&self, identity: &Identity, position: usize) -> Result<Self, &'static str> {
        Err("Not implemented")
    }

    fn try_manipulate(&self, identity: &Identity) -> Vec<Self> {
        replace_identity(&self, identity).unwrap().into_iter().map(|(e, p)|{
            e
        }).collect()
    }
}

impl Manipulatable for Strand<'_> {

    fn manipulate(&self, identity: &Identity, position: usize) -> Result<Self, &'static str> {
        Err("Not implemented")
    }

    fn try_manipulate(&self, identity: &Identity) -> Vec<Self> {
        let new_expressions = replace_identity(&self.current, identity).unwrap();

        new_expressions.into_iter().map(|(expression, position)|{
            let new_step = Step::Manipulation(Manipulation {
                identity,
                position,
                parent: Rc::clone(&(self.step))
            });
            Strand {
                step: Rc::new(new_step),
                current: expression,
            }
        }).collect()
    }
}