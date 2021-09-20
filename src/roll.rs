use std::panic;
use rand::Rng;

use input_parser::join_args;


#[derive(Debug, PartialEq, Copy, Clone)]
pub(self) enum Token {
    Number(i64),
    Roll((u64, u64)),
    Operator(char),
    BracesOpen,
    BracesClose,
}

pub fn execute(args: &[String]) -> i64 {
    panic::set_hook(Box::new(|panic_info| {
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            println!("{:?}", s);
        } else {
            println!("panic occurred");
        }
    }));

    let split_args = input_parser::prepare_args(&mut join_args(args));

    let mut tokens: Vec<Token> = input_parser::parse_tokens(&split_args);

    roll(&mut tokens);

    input_parser::fix_negative_numbers(&mut tokens);

    calculator::transfer_notation(&mut tokens);

    return calculator::calculate(&mut tokens);
}

mod input_parser {

    // build string from commandline args
    pub(crate) fn join_args(args: &[String]) -> String {
        let mut joined_args = "".to_string();

        for (i, entry) in args.iter().enumerate() {
            if i > 0 {
                joined_args.push_str(&entry);
            }
        }
        return joined_args;
    }

    pub(super) fn prepare_args(joined_args: &mut String) -> Vec<String> {
        *joined_args = joined_args.to_lowercase()
            .replace(" ", "")
            .replace("w", "d")
            .replace("d%", "d100")
            .replace("+", " + ")
            .replace("-", " - ")
            .replace("*", " * ")
            .replace("/", " / ")
            .replace("c", " c ")
            .replace("f", " f ")
            .replace(")", " ) ")
            .replace("(", " ( ")
            .replace(" d", " 1d");

        let split: Vec<&str> = joined_args.as_str().split_whitespace().collect();
        let result: Vec<String> = split.iter().map(|s| s.to_string()).collect();
        return result;
    }

    // parse arguments string to tokens
    pub(super) fn parse_tokens(args_list: &Vec<String>) -> Vec<super::Token> {
        let mut tokens: Vec<super::Token> = Vec::new();

        for (_, entry) in args_list.iter().enumerate() {
            match entry {
                x if x == "+" => {
                    tokens.push(super::Token::Operator('+'))
                }
                x if x == "-" => {
                    tokens.push(super::Token::Operator('-'))
                }
                x if x == "*" => {
                    tokens.push(super::Token::Operator('*'))
                }
                x if x == "/" => {
                    tokens.push(super::Token::Operator('/'))
                }
                x if x == "c" => {
                    tokens.push(super::Token::Operator('c'))
                }
                x if x == "f" => {
                    tokens.push(super::Token::Operator('f'))
                }
                x if x == "(" => {
                    tokens.push(super::Token::BracesOpen)
                }
                x if x == ")" => {
                    tokens.push(super::Token::BracesClose)
                }
                _ => {
                    match entry.parse::<i64>() {
                        Ok(i) => {
                            tokens.push(super::Token::Number(i))
                        }
                        Err(_) => {
                            let split: Vec<&str> = entry.split("d").collect();

                            match split.len() == 2 {
                                true => {
                                    let mut dice: (u64, u64) = (split[0].parse().unwrap_or(0), split[1].parse().unwrap_or(0));
                                    if dice.0 == 0 {
                                        dice = (1, dice.1);
                                    } else if dice.1 == 0 {
                                        dice = (1, dice.0);
                                    }

                                    tokens.push(super::Token::Roll(dice))
                                }
                                false => {
                                    panic!("can not parse argument {:?} !", entry)
                                }
                            };
                        }
                    };
                }
            }
        }

        return tokens;
    }

    // workaround for negative numbers
    // because minus in args is used as operator
    pub(super) fn fix_negative_numbers(tokens: &mut Vec<super::Token>) {
        let mut fixed_tokens: Vec<super::Token> = Vec::new();
        let mut last_token: &super::Token = &super::Token::BracesOpen;
        let mut close_brace: bool = false;
        let mut apply_fix: bool;
        for (_, token) in tokens.iter().enumerate() {
            if let super::Token::Operator('-') = token {
                if let super::Token::Number(_) = last_token {
                    apply_fix = false;
                } else {
                    apply_fix = true;
                }
            } else {
                apply_fix = false;
            }

            if apply_fix {
                fixed_tokens.push(super::Token::BracesOpen);
                fixed_tokens.push(super::Token::Number(0));
                fixed_tokens.push(*token);
                close_brace = true;
            } else {
                fixed_tokens.push(*token);
                if close_brace {
                    fixed_tokens.push(super::Token::BracesClose);
                    close_brace = false;
                }
            }

            last_token = token;
        }

        *tokens = fixed_tokens;
    }
}

fn roll(tokens: &mut Vec<Token>) {
    let mut rng = rand::thread_rng();
    for entry in tokens {
        if let Token::Roll((count, dice_size)) = entry {
            let mut random: i64;
            let mut rolls: Vec<i64> = Vec::new();
            for _ in 0..*count {
                random = (rng.gen_range(0..*dice_size) + 1) as i64;
                rolls.push(random);
            }

            let sum = rolls.iter().sum();
            print!("{}: {:?}  = {}\n", format!("{}d{}", count.to_string(), dice_size.to_string()), rolls, sum);

            *entry = Token::Number(sum);
        }
    }
}

mod calculator {
    use std::collections::HashMap;

    // convert expression from infix to postfix notation
    pub(super) fn transfer_notation(tokens: &mut Vec<super::Token>) {
        let mut stack: Vec<super::Token> = Vec::new();
        let mut output: Vec<super::Token> = Vec::new();

        for entry in &*tokens {
            if let super::Token::Number(_) = entry {
                output.push(*entry)
            } else if let super::Token::Operator(op) = entry {
                while let Some(super::Token::Operator(c)) = stack.first() {
                    if compare_operator_prevalence(*op, *c) {
                        let stack_element = stack.pop();
                        match stack_element {
                            Some(x) => {
                                output.push(x);
                            }
                            None => {
                                panic!("operation failed")
                            }
                        }
                    }
                }
                stack.push(*entry)
            } else if let super::Token::BracesOpen = entry {
                stack.push(*entry)
            } else if let super::Token::BracesClose = entry {
                while stack.len() > 0 {
                    let stack_element = stack.pop();
                    match stack_element {
                        Some(x) => {
                            if x == super::Token::BracesOpen {
                                break;
                            } else {
                                output.push(x);
                            }
                        }
                        None => {
                            break;
                        }
                    }
                }
            }
        }

        while stack.len() > 0 {
            let stack_element = stack.pop();
            match stack_element {
                Some(x) => {
                    output.push(x);
                }
                None => {
                    break;
                }
            }
        }

        *tokens = output;
    }

    pub(super) fn calculate(tokens: &mut Vec<super::Token>) -> i64 {
        let mut stack: Vec<super::Token> = Vec::new();

        for entry in &*tokens {
            if let super::Token::Number(_) = entry {
                stack.push(*entry)
            } else if let super::Token::Operator(op) = entry {
                match op {
                    '+' => {
                        let right = get_stack_number(&mut stack);
                        let left = get_stack_number(&mut stack);
                        stack.push(super::Token::Number(left + right));
                    }
                    '-' => {
                        let right = get_stack_number(&mut stack);
                        let left = get_stack_number(&mut stack);
                        stack.push(super::Token::Number(left - right));
                    }
                    '*' => {
                        let right = get_stack_number(&mut stack);
                        let left = get_stack_number(&mut stack);
                        stack.push(super::Token::Number(left * right));
                    }
                    '/' => {
                        let right = get_stack_number(&mut stack);
                        let left = get_stack_number(&mut stack);
                        let result = ((left as f64) / (right as f64)).round() as i64;
                        stack.push(super::Token::Number(result));
                    }
                    'c' => {
                        let right = get_stack_number(&mut stack);
                        let left = get_stack_number(&mut stack);
                        let result = ((left as f64) / (right as f64)).ceil() as i64;
                        stack.push(super::Token::Number(result));
                    }
                    'f' => {
                        let right = get_stack_number(&mut stack);
                        let left = get_stack_number(&mut stack);
                        let result = ((left as f64) / (right as f64)).floor() as i64;
                        stack.push(super::Token::Number(result));
                    }
                    _ => {}
                }
            }
        }

        let stack_element = stack.pop();
        match stack_element {
            Some(x) => {
                if let super::Token::Number(result) = x {
                    return result;
                }
            }
            None => {
                panic!("invalid arguments!");
            }
        }

        panic!("calculation failed");
    }

    fn get_stack_number(stack: &mut Vec<super::Token>) -> i64 {
        let stack_element = stack.pop();

        match stack_element {
            Some(x) => {
                if let super::Token::Number(result) = x {
                    return result;
                }
            }
            None => {
                panic!("missing numeric value!");
            }
        }
        panic!("failed to get stack number")
    }

    fn compare_operator_prevalence(a: char, b: char) -> bool {
        let mut map: HashMap<char, u8> = HashMap::new();
        map.insert('+', 1);
        map.insert('-', 1);
        map.insert('*', 2);
        map.insert('/', 2);
        map.insert('c', 2);
        map.insert('f', 2);
        return map[&a] <= map[&b];
    }
}
