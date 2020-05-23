//! These types represent a more programmer-friendly organization of the
//! data slurped up from the parser.
//!
//! Where the raw data extracted from a plain-text ER file includes more
//! information about the structure of the, document the types here are more to
//! do with the structure of the diagram (our final output will be a dot
//! language representation of this).

use std::collections::HashMap;

/// The haskell version refers to this as "global options" in the parser but
/// also "directives."
///
/// Formatting options can be specified for each of these directive types, and
/// the options set will provide a fallback for each when rendering the
/// various object types in the graph.
pub type GlobalOptions = HashMap<Directive, Options>;

/// A collection of formatting options.
pub type Options = HashMap<FormatOption, String>;

pub struct Attribute {
    field: String,
    pk: bool,
    fk: bool,
    options: Options,
}

/// Defined at each side of a [Relation](struct.Relation.html) a cardinality
/// describes the count constraints for each entity.
pub enum Cardinality {
    ZeroOne,
    One,
    ZeroPlus,
    OnePlus,
}

/// Used as a key for the [GlobalOptions](type.GlobalOptions.html) type.
pub enum Directive {
    Title,
    Header,
    Entity,
    Relationship,
}

pub struct Entity {
    name: String,
    attribs: Vec<Attribute>,
    options: Options,
}

/// Used as a key in the [Options](type.Options.html) type.
pub enum FormatOption {
    BgColor,
    Color,
    FontFace,
    FontSize,
    Border,
    BorderColor,
    CellSpacing,
    CellBorder,
    CellPadding,
    TextAlignment,
}

pub struct Relation {
    entity1: String,
    entity2: String,
    card1: Cardinality,
    card2: Cardinality,
    options: Options,
}
