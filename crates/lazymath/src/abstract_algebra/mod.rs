
mod math_structure;

pub use math_structure::StructBinding;

pub use math_structure::{
    MathStructure,
    MathStructureInstantiationError,
    MathStructureInstance,
    FutureStructBinding,
    AbstractBinaryOperationDefinition,
    IdentityDefinition,
    IdentityDefinitionElement,
    IdentityExpressionDefinition,
    IdentityExpressionDefinitionTerm,
    future::FutureValue,
};