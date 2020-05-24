//! These types represent a more programmer-friendly organization of the
//! data slurped up from the parser.
//!
//! Where the raw data extracted from a plain-text ER file includes more
//! information about the structure of the, document the types here are more to
//! do with the structure of the diagram (our final output will be a dot
//! language representation of this).

use crate::Result;
use std::cmp::Ordering;
use std::collections::hash_map::{Entry, HashMap};
use std::fmt::{Display, Formatter};

/// Represents a single schema.
#[derive(Debug, PartialEq)]
pub struct ER {
    entities: Vec<Entity>,
    rels: Vec<Relation>,
    title: Options,
}

/// Represents a single entity in a schema.
#[derive(Debug, Eq, PartialEq)]
pub struct Entity {
    name: String,
    attribs: Vec<Attribute>,
    /// Formatting options for the header.
    hoptions: Options,
    /// Formatting options for the entity "body."
    eoptions: Options,
}

/// Default ordering for `Entity` (by name).
impl Ord for Entity {
    fn cmp(&self, _other: &Self) -> Ordering {
        unimplemented!()
    }
}
/// Default ordering for `Entity` (by name).
impl PartialOrd for Entity {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        unimplemented!()
    }
}

/// Represents an attribute on a particular entity.
#[derive(Debug, Eq, PartialEq)]
pub struct Attribute {
    field: String,
    pk: bool,
    fk: bool,
    options: Options,
}

/// Default ordering for `Attribute` (by field name).
impl Ord for Attribute {
    fn cmp(&self, other: &Self) -> Ordering {
        unimplemented!()
    }
}
/// Default ordering for `Attribute` (by field name).
impl PartialOrd for Attribute {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        unimplemented!()
    }
}

/// The haskell version refers to this as "global options" in the parser but
/// also "directives."
///
/// Formatting options can be specified for each of these directive types, and
/// the options set will provide a fallback for each when rendering the
/// various object types in the graph.
pub type GlobalOptions = HashMap<Directive, Options>;

/// Used as a key for the [GlobalOptions](type.GlobalOptions.html) type.
#[derive(Debug, PartialEq)]
pub enum Directive {
    Title,
    Header,
    Entity,
    Relationship,
}

/// A collection of formatting options.
#[derive(Debug)]
// FIXME:
//  Seems like the `Options`/`FormatOption` type might be all wrong.
//  We need specific key names matched against specific value types.
//  This sounds like a struct to me, but a struct would not give us the
//  filter-ability needed to run selectors over without a common type for the
//  values.
//  Perhaps we need to change `FormatOptions` to carry a typed value in each
//  variant. Each variant would correspond to a string name. The `Options` type
//  would then become `HashMap<String, FormatOption>` (if still relevant).
pub struct Options(HashMap<FormatOption, String>);

impl Eq for Options {
    fn assert_receiver_is_total_eq(&self) {
        unimplemented!()
    }
}

impl PartialEq for Options {
    fn eq(&self, other: &Self) -> bool {
        unimplemented!()
    }
}

type OptEntry<'m> = Entry<'m, FormatOption, String>;

/// Used as a key in the [Options](type.Options.html) type.
#[derive(Debug, PartialEq)]
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

/// Given two sets of options, merge the second into first, where elements
/// in the first take precedence.
fn merge_opts(_: &Options, _: &Options) -> Options {
    unimplemented!()
}

/// Given a set of options and a selector function, return the list of
/// only those options which matched. Examples of the selector function are
/// `opt_to_font`, `opt_to_html` and `opt_to_label`.
fn options_to<'a, F>(_selector: F, _options: &'a Options) -> Options
where
    F: Fn(&OptEntry<'a>) -> Option<&'a OptEntry<'a>>,
{
    unimplemented!()
}

/// Given an option name and a string representation of its value,
/// `option_by_name` will attempt to parse the string as a value corresponding
/// to the option. If the option doesn't exist or there was a problem parsing
/// the value, an error is returned.
fn option_by_name(_name: &str, _value: &str) -> Result<()> {
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

#[derive(Debug, PartialEq)]
pub struct Relation {
    entity1: String,
    entity2: String,
    card1: Cardinality,
    card2: Cardinality,
    options: Options,
}

/// Defined at each side of a [Relation](struct.Relation.html) a cardinality
/// describes the count constraints for each entity.
#[derive(Debug, PartialEq)]
pub enum Cardinality {
    ZeroOne,
    One,
    ZeroPlus,
    OnePlus,
}

impl Display for Cardinality {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

fn card_by_name(_: char) -> Option<Cardinality> {
    unimplemented!()
}

/// Hard-coded default options for all graph titles.
fn default_title_opts() -> Options {
    unimplemented!()
}

/// Hard-coded default options for all entity headers.
fn default_header_opts() -> Options {
    unimplemented!()
}

/// Hard-coded default options for all entities.
fn default_entity_opts() -> Options {
    unimplemented!()
}

/// Hard-coded default options for all relationships.
fn default_rel_opts() -> Options {
    unimplemented!()
}

/// Hard-coded default options for all attributes.
fn default_attr_opts() -> Options {
    unimplemented!()
}
