use std::rc::Rc;

use super::{step::Step, Manipulatable, Manipulation};

pub struct Strand<'a, T: Manipulatable<'a>> {
    pub step: Rc<Step<'a, T>>,
    pub current: T,
}


impl<'a, T: 'a> ToString for Strand<'a, T> where T: ToString + Manipulatable<'a> {
    fn to_string(&self) -> String {
        self.current.to_string()
    }
}


impl<'a, T: 'a> Manipulatable<'a> for Strand<'a, T> where T: Manipulatable<'a> {

    type Instruction = T::Instruction;
    type Identity = T::Identity;
    type Error = T::Error;

    fn manipulate(&self, instruction: &'a Self::Instruction) -> Result<Option<Self>, Self::Error> {
        let maybe_new_current = self.current.manipulate(instruction)?;

        match maybe_new_current {
            None => return Ok(None),
            Some(new_current) => {
                let new_step = Step::Manipulation(Manipulation {
                    parent: self.step.clone(),
                    instruction: instruction.clone(),
                });
        
                return Ok(Some(Self {
                    step: Rc::new(new_step),
                    current: new_current,
                }))
            },
        };
    }

    fn try_manipulate(&self, identity: &'a Self::Identity) -> Result<Vec<(Self, Self::Instruction)>, &'static str> {
        let new_currents = self.current.try_manipulate(identity)?;

        Ok(new_currents.into_iter().map(|(new_current, instruction)|{
            let new_step = Step::Manipulation(Manipulation {
                parent: self.step.clone(),
                instruction: instruction.clone(),
            });

            (Self {
                step: Rc::new(new_step),
                current: new_current,
            }, instruction)
        }).collect())
    }
}