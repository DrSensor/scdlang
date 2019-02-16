use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ScdlParser;

#[cfg(test)]
mod syntax {
    use super::*;
    // use pest::{parses_to, consumes_to};

    // #[test]
    // #[ignore]
    // fn transition_to() {
    //     parses_to! {
    //         parser: ScdlParser,
    //         input:  "A -> B",
    //         rule:   Rule::complete_syntax,
    //         tokens: [
    //             complete_syntax(0,6, [
    //                 state_name(0,1, [PASCAL_CASE(0,1)]),
    //                 transition_to(2,4),
    //                 state_name(5,6, [PASCAL_CASE(5,6)]),
    //                 EOI(6,6)
    //             ])
    //         ]
    //     };
    // }

    // #[test]
    // #[ignore]
    // fn transition_from() {
    //     parses_to! {
    //         parser: ScdlParser,
    //         input:  "'a' <- 'ba as'",
    //         rule:   Rule::complete_syntax,
    //         tokens: [
    //             complete_syntax(0,14, [
    //                 state_name(0,3, [
    //                     SINGLE_QUOTE(0,1),
    //                     SINGLE_QUOTE(2,3)
    //                 ]),
    //                 transition_from(4,6),
    //                 state_name(7,14, [
    //                     SINGLE_QUOTE(7,8),
    //                     SINGLE_QUOTE(13,14)
    //                 ]),
    //                 EOI(14,14)
    //             ])
    //         ]
    //     };
    // }

    // #[test]
    // #[ignore]
    // fn transition_toggle() {
    //     parses_to! {
    //         parser: ScdlParser,
    //         input:  "A <-> B @ C",
    //         rule:   Rule::complete_syntax,
    //         tokens: [
    //             complete_syntax(0,11, [
    //                 state_name(0,1, [PASCAL_CASE(0,1)]),
    //                 transition_toggle(2,5),
    //                 state_name(6,7, [PASCAL_CASE(6,7)]),
    //                 at_event(8,9),
    //                 event_name(10,11, [PASCAL_CASE(10,11)]),
    //                 EOI(11,11)
    //             ])
    //         ]
    //     };
    // }

    // #[test]
    // #[ignore]
    // fn at_event() {
    //     parses_to! {
    //         parser: ScdlParser,
    //         input:  "A <-> B @ 'sd ad'",
    //         rule:   Rule::complete_syntax,
    //         tokens: [
    //             complete_syntax(0,17, [
    //                 state_name(0,1, [PASCAL_CASE(0,1)]),
    //                 transition_toggle(2,5),
    //                 state_name(6,7, [PASCAL_CASE(6,7)]),
    //                 at_event(8,9),
    //                 event_name(10,17, [
    //                     SINGLE_QUOTE(10,11),
    //                     SINGLE_QUOTE(16,17)
    //                 ]),
    //                 EOI(17,17)
    //             ])
    //         ]
    //     };
    // }

    mod should_fail_when {
        use super::*;
        // use pest::{fails_with};

        // #[test]
        // #[ignore]
        // fn transition_toggle_without_event() {
        //     fails_with! {
        //         parser: ScdlParser,
        //         input: "A <-> B",
        //         rule: Rule::transition,
        //         positives: vec![Rule::at_event],
        //         negatives: vec![],
        //         pos: 7
        //     };
        // }

        #[test]
        fn quoted_name_with_trailing_whitespace() -> Result<(), &'static str> {
            test::wrong_expressions(&[
                r#"'a ' -> B"#,
                r#"" a" -> B"#,
                r#"A -> "b ""#,
                r#"A -> ' b'"#,
                r#"A -> B @ 'c '"#,
                r#"A -> B @ " c""#,
            ])
        }
    }
}

#[cfg(test)]
pub mod test { // helper module
    use super::*;
    use pest::{Parser, error::Error};

    pub fn expression<'a>(expression: &'a str) -> Result<&str, Error<Rule>> {
        Ok(ScdlParser::parse(Rule::complete_syntax, expression)?.as_str())
    }

    pub fn wrong_expressions<'a>(expr_list: &[&'a str]) -> Result<(), &'a str> {
        for expression in expr_list {
            if let Ok(expr) = test::expression(expression) {
                return Err(expr)
            }
        }
        Ok(())
    }

    pub mod parse {
        use super::*;
        use crate::error::Error;
        use pest::iterators::Pair;
        type ParseResult = Result<(), Error>;

        pub fn expression<'a>(text: &'a str, callback: fn(Pair<'a, Rule>) -> ParseResult) -> ParseResult {
            let declaration = ScdlParser::parse(Rule::complete_syntax, text).unwrap();
            for expression in declaration {
                if let Rule::EOI = expression.as_rule() { return Ok(()) }
                callback(expression)?
            }
            Ok(())
        } 
    }
}