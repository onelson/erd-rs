#[macro_use]
extern crate pest_derive;
pub use pest::Parser;

#[derive(Parser)]
#[grammar = "erd.pest"]
pub struct ErdParser;

#[cfg(test)]
mod tests {
    use super::{ErdParser, Parser, Rule};

    #[test]
    fn test_parse_single_entity() {
        let input = r#"
        [Person]
            *name
            height
            weight
            `birth date`
            +birth_place_id
        "#;
        ErdParser::parse(Rule::document, input).unwrap();
    }

    #[test]
    fn test_parse_double_entity() {
        let input = r#"
        [Person]
            *name
            height
            weight
            `birth date`
            +birth_place_id

        [`Birth Place`]
            *id
            `birth city`
            'birth state'
            "birth country"
        "#;
        ErdParser::parse(Rule::document, input).unwrap();
    }

    #[test]
    fn test_parse_double_entity_with_cardinality() {
        let input = r#"
        [Person]
            *name
            height
            weight
            `birth date`
            +birth_place_id

        [`Birth Place`]
            *id
            `birth city`
            'birth state'
            "birth country"

        # Cardinality    Syntax
        # 0 or 1         ?
        # exactly 1      1
        # 0 or more      *
        # 1 or more      +
        Person *--1 `Birth Place`
        "#;
        ErdParser::parse(Rule::document, input).unwrap();
    }

    #[test]
    fn test_option_edge_case_example_from_readme() {
        // This is an example noted in the original erd (haskell) readme.
        // Mainly this is concerned with the framing of options.
        // The start/stop, the trailing commas.
        let input = r##"
        [Person]
          name {
            label: "string",
            color: "#3366ff", # i like bright blue
          }
          weight {
            label: "int",}
        "##;
        ErdParser::parse(Rule::document, input).unwrap();
    }
}
