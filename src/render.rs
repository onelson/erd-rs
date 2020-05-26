//! Given an ER, produce dot language as a string that can be handed off to
//! the graphviz `dot` cli tool.
//!
//! <https://www.graphviz.org/doc/info/command.html>
use crate::er::{Attribute, Entity, Options, Relation, ER};

trait AsDot {
    fn as_dot(&self) -> String;
}
