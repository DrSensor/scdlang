use pest::iterators::Pair;
use crate::{
    grammar::*,
    error::Error::{*, self},
};

/// TODO: Pairs is different from Pair
/// @see https://docs.rs/pest/1.0.0-beta.2/pest/iterators/struct.Pairs.html

pub trait IntoTransition {
    type OutputResult;
    fn into_transition(self) -> Self::OutputResult;
}

#[derive(Debug)]
/// SCXML representation:
/// ```scxml
/// <state id="from">
///     <transition event="at" target="to"/>
/// </state>
/// ```
pub struct Transition<'s> {
    pub from: &'s str,
    pub at: Option<&'s str>,
    pub to: &'s str,
}

#[derive(Debug)]
pub enum Kind<'s> {
    HasOne(Transition<'s>),
    HasTwo(Transition<'s>, Transition<'s>),
}

use Kind::*;
use Rule::*;
impl<'s> IntoTransition for Pair<'s, Rule> {
    type OutputResult = Result<Kind<'s>, Error>;

    fn into_transition(self) -> Self::OutputResult {
        let rule = self.as_rule();

        let mut lhs = "";
        let mut ops: Option<Rule> = None;
        let mut rhs = "";
        let mut event: Option<&str> = None;

        if let Expression = rule {
            for span in self.into_inner() {
                let token = span.as_str();
                match span.as_rule() {
                    StateName => if let None = ops { lhs = token } else { rhs = token },
                    EventName => event = Some(token),
                    transition_toggle | transition_to | transition_from => ops = Some(span.as_rule()),
                    _ => {}
                };
            }

            match ops {
                Some(operator) => match operator {
                    transition_to => Ok(HasOne(Transition{from: lhs, at: event, to: rhs})),
                    transition_from => Ok(HasOne(Transition{from: rhs, at: event, to: lhs})),
                    transition_toggle => Ok(HasTwo(
                        Transition{from: lhs, at: event, to: rhs},
                        Transition{from: rhs, at: event, to: lhs},
                    )),
                    _ => Err(IllegalToken)
                },
                None => Err(MissingOperator)
            }
        } else { Err(WrongRule) }
    }
}


#[cfg(test)]
mod pair {
    pub use super::*;
    use crate::grammar::test;

    pub type ParseResult = Result<(), Error>;

    #[test]
    fn into_transition() -> ParseResult {
        test::parse::expression(r#"
            A -> B
            B <- A
            A <-> B @ C
        "#, |expression| match expression.as_str() {
            "A -> B" | "B <- A" => match expression.into_transition()? {
                HasOne(_transition) => {
                    assert_eq!(_transition.from, "A");
                    assert_eq!(_transition.to, "B");
                    assert::none(_transition.at)?;
                    Ok(())
                },
                _ => Err(WrongRule)
            },
            "A <-> B @ C" => match expression.into_transition()? {
                HasTwo(transition1, transition2) => {
                    assert_eq!(transition1.from, "A");
                    assert_eq!(transition2.from, "B");
                    assert_eq!(transition1.to, "B");
                    assert_eq!(transition2.to, "A");
                    assert::option(transition1.at, "C")?;
                    assert::option(transition2.at, "C")?;
                    Ok(())
                },
                _ => Err(WrongRule)
            },
            _ => Ok(())
        })
    }

    mod assert {
        use super::pair::*;
        use core::fmt::Debug;

        pub fn option<T: PartialEq+Debug>(lhs: Option<T>, rhs: T) -> ParseResult {
            if let Some(value) = lhs {
                assert_eq!(value, rhs);
                Ok(())
            } else { Err(IllegalToken) }
        }

        pub fn none<T>(value: Option<T>) -> ParseResult {
            if let None = value { Ok(()) } else { Err(IllegalToken) }
        }
    }
}