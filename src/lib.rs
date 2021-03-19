pub mod context;
pub mod parser;

pub mod prelude {
    pub use crate::context::{Context, GenerateDirectives};
    pub use crate::parser::DirectiveParser;
}