use crate::{
    semantics::GraphTraversalSignature,
    semantics::{
        elements::{Arguments, ElementQuantifier, ElementSet},
        Predicate, PredicateNode,
    },
    AssertionResponse, TruthValue,
};

/// Assert that a given predicate is true for some set of arguments.
pub struct TrueForArguments<E, const ARITY: usize> {
    arguments: Vec<Arguments<ElementQuantifier<E>, ARITY>>,
    inner: Box<dyn Predicate<E, ARITY>>,
}

impl<E: Eq, const ARITY: usize> Predicate<E, ARITY> for TrueForArguments<E, ARITY> {
    fn call_for_elements(
        &self,
        arguments: &Arguments<ElementQuantifier<E>, ARITY>,
        sig: &mut GraphTraversalSignature,
    ) -> TruthValue {
        let element_matches = self
            .arguments
            .iter()
            .any(|true_args| true_args == arguments);

        TruthValue::Determined(element_matches) | self.inner.call_for_elements(&arguments, sig)
    }

    fn get_elements_for_false(&self) -> Vec<Arguments<ElementSet<E>, ARITY>> {
        todo!()
    }

    fn get_elements_for_true(&self) -> Vec<Arguments<ElementSet<E>, ARITY>> {
        todo!()
    }
}

impl<E: 'static + Eq + Clone, const ARITY: usize> TrueForArguments<E, ARITY> {
    /// Make the assertion
    pub fn assert_on(
        predicate_node: &PredicateNode<E, ARITY>,
        args: Vec<Arguments<ElementQuantifier<E>, ARITY>>,
    ) -> AssertionResponse {
        let mut undermined_args = Vec::with_capacity(args.capacity());
        for arg in args {
            match predicate_node.call_for_elements(&arg, &mut Vec::new()) {
                TruthValue::Determined(false) => return AssertionResponse::AssertionInvalid,
                TruthValue::Determined(true) => {}
                TruthValue::Undetermined => undermined_args.push(arg),
            };
        }

        if undermined_args.len() == 0 {
            return AssertionResponse::AssertionRedundant;
        };

        predicate_node.replace(move |inner| {
            Box::new(TrueForArguments {
                arguments: undermined_args.clone(),
                inner,
            })
        });
        AssertionResponse::AssertionMade
    }
}
