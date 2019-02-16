use pest::iterators::Pairs;
use core::iter::*;
use crate::grammar::*;

pub trait ConvertExpression: Iterator {
    // WARN: returning Vec<Self::Item> because neither Pairs nor Pair implement FromIterator
    fn filter_current_state(self, current_state: &str) -> Vec<Self::Item>;
    // TODO: use filter map because Kind::HasTwo(OutputData, ...)
    // flatten it first then filter it 👈
    // fn group_by_state() -> HasMao<Self>;
}

use Rule::*;
impl<'s> ConvertExpression for Pairs<'s, Rule> {
    // type List = Filter<Self, fn(Pair<'s, Rule>) -> bool>;
    fn filter_current_state(self, current_state: &str) -> Vec<Self::Item> {
        self.filter(|expression| {
            let mut lhs = "";
            let mut is_lhs = true;
            
            // PERF: is it possible to not clone? 🤔 
            for span in expression.clone().into_inner() {
                let token = span.as_str();
                match span.as_rule() {
                    StateName => if is_lhs { lhs = token },
                    transition_toggle | transition_to => is_lhs = false,
                    transition_from => is_lhs = true, // flip lhs to rhs
                    _ => {}
                };
            }

            if lhs == current_state { true } else { false }
        }).collect()
    }
}


#[cfg(test)]
mod pairs {
    use super::*;
    use crate::grammar::*;
    use pest::{Parser, error::Error};

    #[test]
    fn filter_current_state() {
        let declaration = ScdlParser::parse(Rule::complete_syntax, r#"
            A -> B
            B <- A
            B <-> D @ C
            S <-> L @ C
        "#).unwrap();
        let c = declaration.filter_current_state("S");
        println!("{:#?}", c);
    }
}