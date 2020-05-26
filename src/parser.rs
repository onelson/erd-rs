use crate::Result;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "er.pest"]
struct ErParser;

/// Parse an er file to get some pairs.
// TODO: Likely this will not be something we offer in the public API, but it's
//   useful to keep the `dump` example compiling for now.
pub fn parse_pairs(input: &str) -> Result<Pairs<Rule>> {
    Ok(ErParser::parse(Rule::document, input)?)
}

/// Parse and build an `ER` from plain text.
pub fn parse(input: &str) -> Result<crate::er::ER> {
    use crate::er::*;
    let mut er = ER::default();
    for token in parse_pairs(input)?.next().unwrap().into_inner() {
        match token.as_rule() {
            Rule::head => build_head(token, &mut er)?,
            Rule::body => build_body(token, &mut er)?,
            Rule::EOI => (),
            _ => (),
        }
    }

    Ok(er)
}

fn build_head(token: Pair<Rule>, er: &mut crate::er::ER) -> Result<()> {
    debug_assert_eq!(Rule::head, token.as_rule());
    for directive in token.into_inner() {
        debug_assert_eq!(Rule::directive, directive.as_rule());
        let mut inner = directive.into_inner();
        let name = inner.next().unwrap().as_str();
        if let Some(opt_list) = inner.next() {
            let options = build_options(HashMap::new(), opt_list)?;
            match name {
                "title" => er.global_opts.title = options,
                "header" => er.global_opts.header = options,
                "entity" => er.global_opts.entity = options,
                "relationship" => er.global_opts.relationship = options,
                _ => unreachable!(),
            }
        }
    }
    Ok(())
}

fn build_body(token: Pair<Rule>, er: &mut crate::er::ER) -> Result<()> {
    debug_assert_eq!(Rule::body, token.as_rule());
    for item in token.into_inner() {
        match item.as_rule() {
            Rule::entity => er.entities.push(build_entity(item)?),
            Rule::rel => er.rels.push(build_relationship(item)?),
            _ => unreachable!(),
        }
    }

    Ok(())
}

fn build_entity(token: Pair<Rule>) -> Result<crate::er::Entity> {
    debug_assert_eq!(Rule::entity, token.as_rule());

    let mut inner = token.into_inner();
    let entity_header = inner.next().unwrap();
    debug_assert_eq!(Rule::entity_name, entity_header.as_rule());
    let mut entity_header = entity_header.into_inner();
    let name = entity_header
        .next()
        .unwrap() // ident rule
        .into_inner()
        .next() // ident quoted or no space (the actual name string)
        .unwrap()
        .as_str();

    // The header for each entity can have options for the entire entity.
    let eoptions = match entity_header.next() {
        Some(item) => build_options(HashMap::new(), item)?,
        None => crate::er::Options::default(),
    };

    let mut attribs = vec![];

    if let Some(raw_attrs) = inner.next() {
        for item in raw_attrs.into_inner() {
            attribs.push(build_attribute(item)?);
        }
    }
    Ok(crate::er::Entity {
        name: name.to_string(),
        attribs,
        hoptions: crate::er::Options::default(), // FIXME header options should be brought in from globals
        eoptions,
    })
}

fn build_attribute(token: Pair<Rule>) -> Result<crate::er::Attribute> {
    debug_assert_eq!(Rule::attr, token.as_rule());
    let mut field = String::new();
    let mut pk = false;
    let mut fk = false;
    let mut options: Option<_> = None;

    for item in token.into_inner() {
        match item.as_rule() {
            Rule::ident => {
                let name = item.into_inner().next().unwrap();
                field.push_str(name.as_str());
            }
            Rule::opt_list => {
                options = Some(build_options(HashMap::new(), item)?);
            }
            Rule::keys => {
                for key in item.into_inner() {
                    match key.as_rule() {
                        Rule::ispk => pk = true,
                        Rule::isfk => fk = true,
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    Ok(crate::er::Attribute {
        field,
        pk,
        fk,
        options: options.unwrap_or_default(),
    })
}

fn build_relationship(token: Pair<Rule>) -> Result<crate::er::Relation> {
    debug_assert_eq!(Rule::rel, token.as_rule());
    let mut entity1 = String::new();
    let mut entity2 = String::new();
    let mut card1: Option<crate::er::Cardinality> = None;
    let mut card2: Option<crate::er::Cardinality> = None;
    let mut options: Option<_> = None;

    for item in token.into_inner() {
        match item.as_rule() {
            Rule::entity1 => {
                entity1.push_str(
                    item.into_inner()
                        .next()
                        .unwrap()
                        .into_inner()
                        .next()
                        .unwrap()
                        .as_str(),
                );
            }
            Rule::card1 => {
                card1 = crate::er::card_by_name(item.as_str().chars().next().unwrap());
            }
            Rule::entity2 => {
                entity2.push_str(
                    item.into_inner()
                        .next()
                        .unwrap()
                        .into_inner()
                        .next()
                        .unwrap()
                        .as_str(),
                );
            }
            Rule::card2 => {
                card2 = crate::er::card_by_name(item.as_str().chars().next().unwrap());
            }
            Rule::opt_list => {
                options.replace(build_options(HashMap::new(), item)?);
            }
            _ => unreachable!(),
        }
    }

    Ok(crate::er::Relation {
        entity1,
        entity2,
        card1: card1.unwrap(),
        card2: card2.unwrap(),
        options: Default::default(),
    })
}

fn build_options(
    mut acc: HashMap<String, crate::er::Opt>,
    token: Pair<Rule>,
) -> Result<crate::er::Options> {
    debug_assert_eq!(Rule::opt_list, token.as_rule());
    let mut inner = token.into_inner();

    // The `opt_list` rules are recursive with one key/value pair, and an
    // optional tail.
    let mut head = inner.next().unwrap().into_inner();
    let maybe_tail = inner.next();

    let opt_name = head.next().unwrap().as_str();
    let opt_val = head.next().unwrap().as_str();
    acc.insert(
        opt_name.to_string(),
        crate::er::option_by_name(opt_name, opt_val)?,
    );

    match maybe_tail {
        Some(tail) => build_options(acc, tail),
        None => Ok(crate::er::Options(acc)),
    }
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
