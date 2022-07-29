use crate::expr::*;

/// parse the standarded input into the vector of elements as well as the deminiters.
///
/// here by "standard input", we mean the FullForm of expression in Mathematica like `Sin[Plus[Multiply[2,x],1]]`
pub fn lexer(standard_input: &str)->Vec<String> {
    let mut token_stream: Vec<String> = Vec::new();

    let mut iter = standard_input.chars();
    let mut token  = String::new(); // initialize the token
    loop {
        let c = iter.next();
        match c {
            Some(x) => {
                if x == '[' || x == ']' || x == ',' {
                    token_stream.push(token);
                    token = String::new(); // renew the token

                    token_stream.push(x.to_string());
                } else if x == ' ' || x== '\n' || x == '\t' {
                        // do nothing for whitespaces
                } else {
                    token.push(c.unwrap());
                }    
            },
            None => { break; }
        }
    };
    token_stream.into_iter().filter(|x| !x.is_empty()).collect() // remove some left empty element from the parsed result
}

// pub struct Parser {
//     head: i32,
//     tail: i32,
//     contents: 
// }
pub fn parse(lexical_input: Vec<String>)->Expr {
    todo!()
    // for elm in lexical_input.iter() {
    //     if elm == "[" {
    //         parse
    //     }
    // }
}