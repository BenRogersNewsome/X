extern crate rc_wrap;
extern crate enum_dispatch;
mod item;
mod operation;
mod set_element;
mod set;
mod logic;


/// Two-way onion like structure
/// Like a react router
/// Outer layers are responsible for maintaining consistency with inner structures
pub use operation::{BinaryOperation, UnaryOperation};
pub use item::{Item, ItemType};
pub use set_element::SetElement;
pub use set::{Set, SetType};
pub use logic::LBool;
