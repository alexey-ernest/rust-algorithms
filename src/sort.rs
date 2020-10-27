//! Classic sorting algorithms implemented in Rust.
pub mod selection;
pub mod insertion;
pub mod shell;
pub mod bubble;
pub mod merge;
pub mod quick;
pub mod quick_3_way;
pub mod flag;
pub mod heap;

pub use selection::*;
pub use insertion::*;
pub use shell::*;
pub use bubble::*;
pub use merge::*;
pub use quick::*;
pub use quick_3_way::*;
pub use flag::*;
pub use heap::*;  
