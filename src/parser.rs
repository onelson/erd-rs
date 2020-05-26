use crate::Result;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};

#[derive(Parser)]
#[grammar = "er.pest"]
struct ErParser;

/// Parse an er file to get some pairs.
// TODO: Likely this will not be something we offer in the public API, but it's
//   useful to keep the `dump` example compiling for now.
pub fn parse_pairs(input: &str) -> Result<Pairs<Rule>> {
    Ok(ErParser::parse(Rule::document, input)?)
}

#[cfg(test)]
mod tests {
    //! The tests here aim to "prove the spec" described in the readme for the
    //! original erd (haskell) project.
    //!
    //! <https://github.com/BurntSushi/erd/blob/c5c6e1e7971a53c513aa27edd902cfd6492a57cf/README.md#the-er-file-format>
    //!
    //! - directives should not come after entities/relationships
    //! - options must not end on the same line as other things start.
    //! - leading whitespace doesn't matter anywhere.
    //! - options can appear next to all of:
    //!   - directives
    //!   - relationships
    //!   - entity headers
    //!   - attributes
    //! - option values must use double quotes.
    //! - options should start *on the same line* as the thing they are options
    //!   for, but can contain newlines.

    use super::parse_pairs;

    /// Directives must appear before all other items in the er file.
    #[test]
    fn test_directives_first_ok() {
        let input = r#"
        title { label: "foo bar" }
        header { label: "foo bar" }
        entity { label: "foo bar" }
        relationship { label: "foo bar" }
        
        [Baz]
            qux
        "#;
        parse_pairs(input).unwrap();
    }

    /// We only have 4 directives that are legal:
    ///
    /// - title
    /// - header
    /// - entity
    /// - relationship
    ///
    #[test]
    fn test_unexpected_directive_is_err() {
        let input = r#"        
        unknown { label: "foo bar" }
        [Baz]
            qux
        "#;
        assert!(parse_pairs(input).is_err()); // FIXME: inspect the Err
    }

    /// When what would otherwise be a valid directive appears after an entity
    /// has been defined, it'll effectively be treated as an attribute.
    /// Directives must appear at the very start of the er document, but it just
    /// so happens that they conform to all the same rules as attributes anyway.
    #[test]
    fn test_directives_after_entity_ok_but_attr_instead() {
        let input = r#"
        [Baz]
            qux
            title { label: "foo bar" }
        "#;
        parse_pairs(input).unwrap();
    }

    /// When what would otherwise be a valid directive appears after a
    /// relationship (and nothing else), that's illegal.
    #[test]
    fn test_directives_after_rel_is_err() {
        let input = r#"
        Foo 1--1 Bar
        title { label: "oops" }
        "#;
        assert!(parse_pairs(input).is_err()); // FIXME: inspect the Err
    }

    /// I don't know why you'd have an er with relationships defined and no
    /// entities, but I suppose a valid directive *before* that should be fine.
    #[test]
    fn test_directives_before_rel_is_ok() {
        let input = r#"
        title { label: "totally fine" }
        
        Foo 1--1 Bar
        "#;
        parse_pairs(input).unwrap();
    }

    #[test]
    fn test_option_end_with_blank_ok() {
        let input = r#"
        title { label: "totally fine" }          "#;
        parse_pairs(input).unwrap();
    }

    #[test]
    fn test_option_end_with_comment_ok() {
        let input = r#"
        title { label: "totally fine" } # comments"#;

        parse_pairs(input).unwrap();

        let input = r#"
        title { label: "totally fine" }     # comments
        "#;

        parse_pairs(input).unwrap();
    }

    #[test]
    fn test_option_end_with_newline_ok() {
        let input = r#"
        title { label: "totally fine" }
        "#;
        parse_pairs(input).unwrap();
    }

    #[test]
    fn test_option_start_on_blank_is_err() {
        let input = r#"
        [Person]
        name
        { label: "a person's name" }
        age
        "#;
        assert!(parse_pairs(input).is_err());
    }

    #[test]
    fn test_option_end_with_attr_is_err() {
        let input = r#"
        [Person]
        name { label: "a person's name" 
        } age
        "#;
        assert!(parse_pairs(input).is_err());
    }

    #[test]
    fn test_option_end_with_entity_is_err() {
        let input = r#"
        [Person]
        name { label: "a person's name" 
        } [Group]
        id
        "#;
        assert!(parse_pairs(input).is_err());
    }

    #[test]
    fn test_option_end_with_rel_is_err() {
        let input = r##"
        [Person]
        name { label: "a person's name" 
        } [Group]
        id {
            bgcolor: "#ff33ff",
        } Person *--* Group
        "##;
        assert!(parse_pairs(input).is_err());
    }

    #[test]
    fn test_option_end_with_directive_is_err() {
        let input = r##"
        title { "main title" 
        } header { bgcolor: "#deedee" }
        "##;
        assert!(parse_pairs(input).is_err());
    }

    /// Per the original haskell erd - option values must be quoted, even if
    /// they are numbers. They must use double quotes.
    #[test]
    fn test_option_val_must_use_double_quotes() {
        let double_quote = r#"title { label: "123" }"#; // Good
        parse_pairs(double_quote).unwrap();

        let single_quote = r#"title { label: '123' }"#; // Bad
        assert!(parse_pairs(single_quote).is_err());

        let unquoted = r#"title { label: 123 }"#; // Bad
        assert!(parse_pairs(unquoted).is_err());
    }

    /// This is really just to check that the `#` in the string value doesn't
    /// get parsed as a comment.
    #[test]
    fn test_option_val_can_contain_pound() {
        let input = r##"header { bgcolor: "#663399" }"##; // Good

        // FIXME: check the parsed value to confirm the bgcolor value is expected.
        parse_pairs(input).unwrap();
    }

    #[test]
    #[ignore]
    fn test_option_can_belong_to_directive() {
        let input = r#"
        title { label: "Main title" }
        
        Person 1--* Group { label: "A person belongs to zero or more groups" }
        
        [Person]
        name {label: "A person's name" }
        
        [Group]
        id
        "#;
        // FIXME: check that label is actually associated with directive
        parse_pairs(input).unwrap();
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn test_option_can_belong_to_entity() {
        let input = r#"
        title { label: "Main title" }
        
        Person 1--* Group { label: "A person belongs to zero or more groups" }
        
        [Person]
        name {label: "A person's name" }
        
        [Group]
        id
        "#;
        // FIXME: check that label is actually associated with entity
        parse_pairs(input).unwrap();
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn test_option_can_belong_to_attr() {
        let input = r#"
        title { label: "Main title" }
        
        Person 1--* Group { label: "A person belongs to zero or more groups" }
        
        [Person]
        name {label: "A person's name" }
        
        [Group]
        id
        "#;
        // FIXME: check that label is actually associated with attr
        parse_pairs(input).unwrap();
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn test_option_can_belong_to_rel() {
        let input = r#"
        title { label: "Main title" }
        
        Person 1--* Group { label: "A person belongs to zero or more groups" }
        [Person]
        name
        [Group]
        id
        "#;
        // FIXME: check that label is actually associated with rel
        parse_pairs(input).unwrap();
        unimplemented!();
    }

    #[test]
    fn test_leading_whitespace_is_fine() {
        let input = r##"
                header      {
                                label: "explore the space"
                    }
        [A]
         b
            c {
                label:              "really explore the space"
                    }
             [D]
         e {
 label: "I gotta fever"
     }
                    A *--*              D {
                    
                    label: "Get the As and Ds together" }
        "##;
        parse_pairs(input).unwrap();
    }

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
        parse_pairs(input).unwrap();
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
        parse_pairs(input).unwrap();
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
        parse_pairs(input).unwrap();
    }

    /// This is an example noted in the original erd (haskell) readme.
    /// Mainly this is concerned with the framing of options.
    /// The start/stop, the trailing commas.
    #[test]
    fn test_option_edge_case_example_from_readme() {
        let input = r##"
        [Person]
          name {
            label: "string",
            color: "#3366ff", # i like bright blue
          }
          weight {
            label: "int",}
        "##;
        parse_pairs(input).unwrap();
    }
}
