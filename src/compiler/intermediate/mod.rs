/*
* This is going to be a intermediate compiler
* to allow for very complex code generation that wouldn't be natural or as easy converting raw bf to C
*/

pub mod tokenizer;

use self::tokenizer::Token;
use self::tokenizer::Token::*;

use std::mem;

#[derive(Debug)]
struct Opt {
    opt_type: Token,
    opt: String,
    takes_value: bool,
    value: u32
}

impl Opt {
    pub fn new(token: &Token) -> Opt {
        let mut takes_value = false;
        let opt = {
            match token {
                Add => {
                    takes_value = true;
                    String::from("add")
                },
                Sub => {
                    takes_value = true;
                    String::from("sub")
                },
                Left => {
                    takes_value = true;
                    String::from("lshift")
                },
                Right => {
                    takes_value = true;
                    String::from("rshift")
                },
                Read => {
                    String::from("in")
                },
                Write => {
                    String::from("out")
                },
                BeginLoop => {
                    String::from("bloop")
                },
                EndLoop => {
                    String::from("eloop")
                }
            }
        };

        Opt {
            opt_type: *token,
            opt: opt,
            takes_value: takes_value,
            value: 1
        }
    }
}

pub fn tokenize_input(input: &str) -> Vec<Token> {
    tokenizer::tokenize(input)
}

pub fn generate_intermediate_data(tokens: &mut Vec<Token>) -> String {
    fn recursive_process(current: &mut Opt, current_token: &Option<Token>, output: &mut String) {
        /*
        * Tokens need to be processed here and managed so that the output is
        * properly correct.
        */

        // If token is None then we can assume that there are no more tokens
        if current_token.is_none() {
            if current.takes_value {
                output.push_str(format!("{} {}\r\n", current.opt, current.value).as_str());
            } else {
                output.push_str(format!("{}\r\n", current.opt).as_str());
            }

            return;
        }

        // The first thing that needs to be done is to identify if the opt
        // takes a value or not.
        if current.takes_value {
            // We now need to check if this type is the same as the new token.
            if current.opt_type != current_token.unwrap() {
                // If this type is different, then this opt is finished.
                // Now we have to push the value to the output.
                output.push_str(format!("{} {}\r\n", current.opt, current.value).as_str());

                // Now that the value is used and finished, we cant safely and
                // effectively replace with mem::replace.
                mem::replace(current, Opt::new(&current_token.unwrap()));

                // This marks the end of this function, there should be no other
                // operation needed.
            } else {
                // If the type is the same, we are reassured that this opt will
                // take a value, thus only needing one addition.
                current.value += 1;
            }
        } else {
            // If this value doesn't take a value, then there is no need for extra processing.
            // This can all be handled very quickly.
            output.push_str(format!("{}\r\n", current.opt).as_str());
            mem::replace(current, Opt::new(&current_token.unwrap()));
        }
    }

    let mut output = String::new();
    let mut current_opt: Opt = Opt::new(&tokens.remove(0));
    for token in tokens {
        recursive_process(&mut current_opt, &Some(*token), &mut output);
    }

    // This is for the last element.
    recursive_process(&mut current_opt, &None, &mut output);

    output
}