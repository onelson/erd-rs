//! These types represent a more programmer-friendly organization of the
//! data slurped up from the parser.
//!
//! Where the raw data extracted from a plain-text ER file includes more
//! information about the structure of the, document the types here are more to
//! do with the structure of the diagram (our final output will be a dot
//! language representation of this).

use crate::{Error, Result};
use std::cmp::Ordering;
use std::collections::hash_map::HashMap;
use std::fmt::{Display, Formatter};

/// Represents a single schema.
#[derive(Debug, Default, PartialEq)]
pub struct ER {
    pub global_opts: GlobalOptions,
    pub entities: Vec<Entity>,
    pub rels: Vec<Relation>,
    pub title: Options,
}

/// Represents a single entity in a schema.
#[derive(Debug, Eq, PartialEq)]
pub struct Entity {
    pub name: String,
    pub attribs: Vec<Attribute>,
    /// Formatting options for the header.
    pub hoptions: Options,
    /// Formatting options for the entity "body."
    pub eoptions: Options,
}

/// Default ordering for `Entity` (by name).
impl Ord for Entity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}
/// Default ordering for `Entity` (by name).
impl PartialOrd for Entity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

/// Represents an attribute on a particular entity.
#[derive(Debug, Eq, PartialEq)]
pub struct Attribute {
    pub field: String,
    pub pk: bool,
    pub fk: bool,
    pub options: Options,
}

/// Default ordering for `Attribute` (by field name).
impl Ord for Attribute {
    fn cmp(&self, other: &Self) -> Ordering {
        self.field.cmp(&other.field)
    }
}
/// Default ordering for `Attribute` (by field name).
impl PartialOrd for Attribute {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.field.cmp(&other.field))
    }
}

/// The haskell version refers to this as "global options" in the parser but
/// also "directives."
///
/// Formatting options can be specified for each of these directive types in the
/// header section of the er file.
/// The options will provide a fallback for each when rendering the various
/// object types in the graph.
#[derive(Debug, Default, PartialEq)]
pub struct GlobalOptions {
    pub title: Options,
    pub header: Options,
    pub entity: Options,
    pub relationship: Options,
}

/// A collection of formatting options.
#[derive(Debug, Default)]
pub struct Options(pub HashMap<String, Opt>);

impl PartialEq for Options {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Options {}

// The following type aliases are stubs matching the Haskell types (mostly).
// In many cases, the types used to represent these formatting options are
// selected based on the graphviz api being used, but for us we have no such
// wrapper to align with. *Practically speaking* we could represent all these
// values as String and just pass them straight through in our dot output.
//
// This may in fact be what we need to do.

// Seems like this is going to be any valid hex color, or one of the named
// colors available in html.
// <https://graphviz.gitlab.io/_pages/doc/info/colors.html>
type Color = String;
type Text = String;
type Double = f64;
type Word8 = u8;
// FIXME: change to an enum?
//  Described in <https://www.graphviz.org/doc/info/shapes.html#html> as:
//  > specifies horizontal placement. When an object is allocated more space
//  > than required, this value determines where the extra space is placed left
//  > and right of the object.
//  >  - CENTER aligns the object in the center. (Default)
//  >  - LEFT aligns the object on the left.
//  >  - RIGHT aligns the object on the right.
//  >  - (<TD> only) TEXT aligns lines of text using the full cell width. The
//       alignment of a line is determined by its (possibly implicit) associated
//       <BR> element.
//  >
//  > The contents of a cell are normally aligned as a block.
//  > In particular, lines of text are first aligned as a text block based on
//  > the width of the widest line and the corresponding <BR> elements.
//  > Then, the entire text block is aligned within a cell. If, however, the
//  > cell's ALIGN value is "TEXT", and the cell contains lines of text, then
//  > the lines are justified using the entire available width of the cell. If
//  > the cell does not contain text, then the contained image or table is
//  > centered.
type Align = String;

/// Used as a key in the [Options](type.Options.html) type.
#[derive(Clone, Debug, PartialEq)]
pub enum Opt {
    Label(Text),
    BgColor(Color),
    Color(Color),
    FontFace(Text),
    FontSize(Double),
    Border(Word8),
    BorderColor(Color),
    CellSpacing(Word8),
    CellBorder(Word8),
    CellPadding(Word8),
    TextAlignment(Align),
}

impl Opt {
    /// The html attr name for the option.
    fn html_attr_name(&self) -> &str {
        match self {
            Opt::Label(_) => "label",
            Opt::Color(_) => "color",
            Opt::BgColor(_) => "bgcolor",
            Opt::FontSize(_) => "size",
            Opt::FontFace(_) => "font",
            Opt::Border(_) => "border",
            Opt::BorderColor(_) => "border-color",
            Opt::CellSpacing(_) => "cellspacing",
            Opt::CellBorder(_) => "cellborder",
            Opt::CellPadding(_) => "cellpadding",
            Opt::TextAlignment(_) => "text-alignment",
        }
    }
}

/// Given two sets of options, merge the second into first, where elements
/// in the first take precedence.
fn merge_opts(a: &Options, b: &Options) -> Options {
    Options(
        b.0.iter()
            .chain(a.0.iter())
            .map(|(key, val)| (key.clone(), val.clone()))
            .collect(),
    )
}

/// Given a set of options and a selector function, return the list of
/// only those options which matched. Examples of the selector function are
/// `opt_to_font`, `opt_to_html` and `opt_to_label`.
fn options_to<F>(selector: F, options: &Options) -> Options
where
    F: Fn(&Opt) -> Option<&Opt>,
{
    Options(
        options
            .0
            .iter()
            .filter_map(|(name, opt)| selector(opt).map(|opt| (name.clone(), opt.clone())))
            .collect(),
    )
}

/// Remove a double quote char from the first and last position in a string.
fn trim_quotes(s: &str) -> &str {
    let quote = '"';
    s.trim_start_matches(quote).trim_end_matches(quote)
}

/// Given an option name and a string representation of its value,
/// `option_by_name` will attempt to parse the string as a value corresponding
/// to the option. If the option doesn't exist or there was a problem parsing
/// the value, an error is returned.
pub fn option_by_name(name: &str, value: &str) -> Result<Opt> {
    let value = trim_quotes(value);
    let parsed = match name {
        "label" => Opt::Label(value.to_string()),
        "color" => Opt::Color(value.to_string()),
        "bgcolor" => Opt::BgColor(value.to_string()),
        "size" => Opt::FontSize(value.parse()?),
        "font" => Opt::FontFace(value.to_string()),
        "border" => Opt::Border(value.parse()?),
        "border-color" => Opt::BorderColor(value.to_string()),
        "cellspacing" => Opt::CellSpacing(value.parse()?),
        "cellborder" => Opt::CellBorder(value.parse()?),
        "cellpadding" => Opt::CellPadding(value.parse()?),
        "text-alignment" => Opt::TextAlignment(value.to_string()),
        _ => Err(Error::UnknownFormatOption(name.to_string()))?,
    };
    Ok(parsed)
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
fn opt_to_font(opt: &Opt) -> Option<&Opt> {
    use self::Opt::{Color, FontFace, FontSize};
    match opt {
        Color(_) | FontFace(_) | FontSize(_) => Some(opt),
        _ => None,
    }
}

/// Selects an option if and only if it corresponds to an HTML attribute.
/// In particular, for tables or table cells.
fn opt_to_html(opt: &Opt) -> Option<&Opt> {
    use self::Opt::{
        BgColor, Border, BorderColor, CellBorder, CellPadding, CellSpacing, TextAlignment,
    };
    match opt {
        BgColor(_) | Border(_) | BorderColor(_) | CellBorder(_) | CellSpacing(_)
        | CellPadding(_) | TextAlignment(_) => Some(opt),
        _ => None,
    }
}

/// Selects an option if and only if it corresponds to a label.
fn opt_to_label(opt: &Opt) -> Option<&Opt> {
    match opt {
        Opt::Label(_) => Some(opt),
        _ => None,
    }
}

#[derive(Debug, PartialEq)]
pub struct Relation {
    pub entity1: String,
    pub entity2: String,
    pub card1: Cardinality,
    pub card2: Cardinality,
    pub options: Options,
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Cardinality::*;
        let val = match self {
            ZeroOne => "{0,1}",
            One => "1",
            ZeroPlus => "0..N",
            OnePlus => "1..N",
        };
        Ok(write!(f, "{}", val)?)
    }
}

pub fn card_by_name(c: char) -> Option<Cardinality> {
    use Cardinality::*;
    match c {
        '?' => Some(ZeroOne),
        '1' => Some(One),
        '*' => Some(ZeroPlus),
        '+' => Some(OnePlus),
        _ => None,
    }
}

/// Hard-coded default options for all graph titles.
fn default_title_opts() -> Options {
    let defaults = vec![Opt::FontSize(30.0)]
        .into_iter()
        .map(|opt| (opt.html_attr_name().to_string(), opt))
        .collect();
    Options(defaults)
}

/// Hard-coded default options for all entity headers.
fn default_header_opts() -> Options {
    let defaults = vec![Opt::FontSize(16.0)]
        .into_iter()
        .map(|opt| (opt.html_attr_name().to_string(), opt))
        .collect();
    Options(defaults)
}

/// Hard-coded default options for all entities.
fn default_entity_opts() -> Options {
    let defaults = vec![
        Opt::Border(0),
        Opt::CellBorder(1),
        Opt::CellSpacing(0),
        Opt::CellPadding(4),
        Opt::FontFace("Helvetica".to_string()),
    ]
    .into_iter()
    .map(|opt| (opt.html_attr_name().to_string(), opt))
    .collect();
    Options(defaults)
}

/// Hard-coded default options for all relationships.
fn default_rel_opts() -> Options {
    Options(Default::default())
}

/// Hard-coded default options for all attributes.
fn default_attr_opts() -> Options {
    let defaults = vec![Opt::TextAlignment("LEFT".to_string())]
        .into_iter()
        .map(|opt| (opt.html_attr_name().to_string(), opt))
        .collect();
    Options(defaults)
}
