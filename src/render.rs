//! Given an ER, produce dot language as a string that can be handed off to
//! the graphviz `dot` cli tool.
//!
//! <https://www.graphviz.org/doc/info/command.html>
use crate::er::{Attribute, Entity, Options, Relation, ER};
use std::fmt::{Display, Formatter};

trait AsDot {
    fn as_dot(&self) -> String;
}

impl AsDot for Attribute {
    fn as_dot(&self) -> String {
        format!("<tr><td>{}</td></tr>", self.field)
    }
}

impl AsDot for Entity {
    fn as_dot(&self) -> String {
        let attrs = {
            let xs: Vec<_> = self.attribs.iter().map(|attr| attr.as_dot()).collect();
            xs.join("\n")
        };
        format!(
            r#""{0}" [label=<
        <table>
            <tr><td>{0}</td></tr>
            {1}
        </table>>];"#,
            self.name, attrs
        )
    }
}

impl AsDot for ER {
    fn as_dot(&self) -> String {
        let entities: Vec<_> = self.entities.iter().map(|e| e.as_dot()).collect();
        let relationships: Vec<_> = self.rels.iter().map(|e| e.as_dot()).collect();

        format!(
            r#"
        digraph ER {{
            {0}
            {1}
        }}
        "#,
            entities.join("\n"),
            relationships.join("\n")
        )
    }
}

impl AsDot for Relation {
    fn as_dot(&self) -> String {
        // note that the arrow *starts at the tail* and *ends at the head*.
        format!(
            r#""{0}" -> "{1}" [taillabel="{2}", headlabel="{3}"];"#,
            self.entity1, self.entity2, self.card1, self.card2,
        )
    }
}

impl Display for ER {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_dot())
    }
}
