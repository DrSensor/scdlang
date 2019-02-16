use pest::Parser;
use parser_scdlang::*;

fn main() {
    let pairs = ScdlParser::parse(Rule::complete_syntax, r#"
    A <- B
    A -> B
    "#).unwrap_or_else(|e| panic!("{}", e));

    // Because ident_list is silent, the iterator will contain idents
    // TODO: use pairs.filter(|expression| expression.into_transition
    for pair in pairs {

        let span = pair.clone().into_span();
        // A pair is a combination of the rule which matched and a span of input
        // println!("Rule:    {:?}", pair.as_rule());
        // println!("Span:    {:?}", span);
        println!("Declaration:    {}", span.as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
        // for expr in pair.into_inner() {
        //     println!("\nText:         {}", expr.as_str().trim());
        //     for inner_pair in expr.into_inner() {
        //         let inner_span = inner_pair.clone().into_span();
        //         match inner_pair.as_rule() {
        //             Rule::state_name => println!("State:        {:?}", inner_span),
        //             Rule::event_name => println!("Event:        {:?}", inner_span),
        //             Rule::transition_toggle | Rule::transition_to | Rule::transition_from => println!("Transition:   {:?}", inner_span),
        //             _ => {}
        //         };
        //     }
        // }
    }
}