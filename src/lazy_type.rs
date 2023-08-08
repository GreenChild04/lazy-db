mod lazy_types;
mod converter;
pub use lazy_types::*; // Just to clean up code
pub use converter::*;

pub trait CustomLazyType {
    fn hollow() -> Self where Self: Sized;
}

#[derive(Debug, PartialEq, Eq)]
pub enum LazyType {
    Void,
    Custom(u8), // dunno how to implement yet
    String,
    INum(LazyINumType),
    UNum(LazyUNumType),
    Float(LazyFloatType),
    Binary,
}