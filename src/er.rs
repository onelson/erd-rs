//! These types represent a more programmer-friendly organization of the
//! data slurped up from the parser.
//!
//! Where the raw data extracted from a plain-text ER file includes more
//! information about the structure of the, document the types here are more to
//! do with the structure of the diagram (our final output will be a dot
//! language representation of this).

use crate::Result;
use std::collections::hash_map::{Entry, HashMap};

/// The haskell version refers to this as "global options" in the parser but
/// also "directives."
///
/// Formatting options can be specified for each of these directive types, and
/// the options set will provide a fallback for each when rendering the
/// various object types in the graph.
pub type GlobalOptions = HashMap<Directive, Options>;

// FIXME:
//  Seems like the `Options`/`FormatOption` type might be all wrong. We need specific key
//  names matched against specific value types. This sounds like a struct
//  to me, but a struct would not give us the filter-ability needed to run
//  selectors over without a common type for the values.
//  Perhaps we need to change `FormatOptions` to carry a typed value in each
//  variant. Each variant would correspond to a string name. The `Options` type
//  would then become `HashMap<String, FormatOption>` (if still relevant).
/// A collection of formatting options.
pub type Options = HashMap<FormatOption, String>;
type OptEntry<'m> = Entry<'m, FormatOption, String>;

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

pub struct Relation {
    entity1: String,
    entity2: String,
    card1: Cardinality,
    card2: Cardinality,
    options: Options,
}

/// Given two sets of options, merge the second into first, where elements
/// in the first take precedence.
fn merge_opts(_: Options, _: Options) -> Options {
    unimplemented!()
}

/// Given a set of options and a selector function, return the list of
/// only those options which matched. Examples of the selector function are
/// `opt_to_font`, `opt_to_html` and `opt_to_label`.
fn options_to<'a, F>(_: &'a Options, _: F) -> Options
where
    F: Fn(&OptEntry<'a>) -> Option<&'a OptEntry<'a>>,
{
    unimplemented!()
}

/// Given an option name and a string representation of its value,
/// `option_by_name` will attempt to parse the string as a value corresponding
/// to the option. If the option doesn't exist or there was a problem parsing
/// the value, an error is returned.
fn option_by_name(_: &str, _: &str) -> Result<()> {
    // FIXME: need a way to unify the return type.
    //  `FormatOption` with an inner value?
    unimplemented!()
}

/// A wrapper around the GraphViz's parser for any particular option.
fn option_parse() -> Result<()> {
    // FIXME: this one is probably not going to help us if we don't have a dot
    //  parser. It also seems like our pest parser should already know how to
    //  break up the quoted strings.
    //  For us, we'll probably have to generate the dot blind and let the dot
    //  cli handle this sort of validation.
    unimplemented!()
}

/// Selects an option if and only if it corresponds to a font attribute.
fn opt_to_font(_opt: &FormatOption) -> Option<&FormatOption> {
    unimplemented!()
}

/// Selects an option if and only if it corresponds to an HTML attribute.
/// In particular, for tables or table cells.
fn opt_to_html(_opt: &FormatOption) -> Option<&FormatOption> {
    unimplemented!()
}

/// Selects an option if and only if it corresponds to a label.
fn opt_to_label(_opt: &FormatOption) -> Option<&FormatOption> {
    unimplemented!()
}
