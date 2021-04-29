pub(crate) mod parser {
    use rand::Rng;
    use std::collections::HashMap;
    use std::panic;

    pub fn parse(args: &[String]) {

        panic::set_hook(Box::new(|panic_info| {
            if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                println!("{:?}", s);
            } else {
                println!("panic occurred");
            }
        }));

        let split_args = prepare_args(args);

        let mut tokens: Vec<Token> = parse_tokens(&split_args);

        roll(&mut tokens);

        transfer_notation(&mut tokens);

        let result = calculate(&mut tokens);
        print!("Total: {}", result);

    }

    fn prepare_args(args: &[String]) -> Vec<String> {
        let mut joined_args = "".to_string();

        for (i, entry) in args.iter().enumerate() {
            if i > 0 {
                joined_args.push_str(&entry);
            }
        }
        joined_args = joined_args.to_lowercase()
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

    fn parse_tokens(args_list: &Vec<String>) -> Vec<Token>{
        let mut tokens: Vec<Token> = Vec::new();

        for (_, entry) in args_list.iter().enumerate() {
            match entry {
                x if x == "+" => {
                    tokens.push(Token::Operator('+'))
                }
                x if x == "-" => {
                    tokens.push(Token::Operator('-'))
                }
                x if x == "*" => {
                    tokens.push(Token::Operator('*'))
                }
                x if x == "/" => {
                    tokens.push(Token::Operator('/'))
                }
                x if x == "c" => {
                    tokens.push(Token::Operator('c'))
                }
                x if x == "f" => {
                    tokens.push(Token::Operator('f'))
                }
                x if x == "(" => {
                    tokens.push(Token::BracesOpen)
                }
                x if x == ")" => {
                    tokens.push(Token::BracesClose)
                }
                _ => {
                    let _ = match entry.parse::<i64>() {
                        Ok(i) => {
                            tokens.push(Token::Number(i))
                        },
                        Err(_e) => {

                            let split: Vec<&str> = entry.split("d").collect();

                            let _ = match split.len() == 2 {
                                true => {
                                    let dice: (u64, u64) = (split[0].parse().unwrap_or(0), split[1].parse().unwrap_or(0));
                                    if dice.0 == 0 || dice.1 == 0 {
                                        panic!("invalid dice!")
                                    }
                                    tokens.push(Token::Roll(dice))
                                },
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

    fn roll(tokens:&mut Vec<Token>) {
        let mut rng = rand::thread_rng();
        for entry in tokens {
            if let Token::Roll((count,dice_size)) = entry {
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

    fn transfer_notation(tokens:&mut Vec<Token>) {
        let mut stack: Vec<Token> = Vec::new();
        let mut output: Vec<Token> = Vec::new();

        for entry in &*tokens {
            if let Token::Number(_) = entry {
                output.push(*entry)
            } else if let Token::Operator(op) = entry {
                while let Some(Token::Operator(c)) =  stack.first() {
                    if compare_opertor_prevalenz(*op,*c) {
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
            } else if let Token::BracesOpen = entry {
                stack.push(*entry)
            } else if let Token::BracesClose = entry {
                while stack.len() > 0 {
                    let stack_element = stack.pop();
                    match stack_element {
                        Some(x) => {
                            if x == Token::BracesOpen {
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

        while stack.len() > 0  {
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

    fn calculate(tokens:&mut Vec<Token>) -> i64 {
        let mut stack: Vec<Token> = Vec::new();

        for entry in &*tokens {
            if let Token::Number(_) = entry {
                stack.push(*entry)
            } else if let Token::Operator(op) = entry {
                match op {
                    '+' => {
                        let left = get_stack_number(&mut stack);
                        let right = get_stack_number(&mut stack);
                        stack.push(Token::Number(left + right));
                    }
                    '-' => {
                        let left = get_stack_number(&mut stack);
                        let right = get_stack_number(&mut stack);
                        stack.push(Token::Number(left - right));
                    }
                    '*' => {
                        let left = get_stack_number(&mut stack);
                        let right = get_stack_number(&mut stack);
                        stack.push(Token::Number(left * right));
                    }
                    '/' => {
                        let right = get_stack_number(&mut stack);
                        let left = get_stack_number(&mut stack);
                        let result =  ((left as f64) / (right as f64)).round() as i64;
                        stack.push(Token::Number(result));
                    }
                    'c' => {
                        let right = get_stack_number(&mut stack);
                        let left = get_stack_number(&mut stack);
                        let result =  ((left as f64) / (right as f64)).ceil() as i64;
                        stack.push(Token::Number(result));
                    }
                    'f' => {
                        let right = get_stack_number(&mut stack);
                        let left = get_stack_number(&mut stack);
                        let result =  ((left as f64) / (right as f64)).floor() as i64;
                        stack.push(Token::Number(result));
                    }
                    _ => {}


                }
            }
        }

        let stack_element = stack.pop();
        match stack_element {
            Some(x) => {
                if let Token::Number(result) = x {
                    return result;
                }
            }
            None => {
                panic!("invalid arguments!");
            }
        }

        panic!()
    }

    fn get_stack_number(stack: &mut Vec<Token>) -> i64 {
        let stack_element = stack.pop();

        match stack_element {
            Some(x) => {
                if let Token::Number(result) = x {
                    return result;
                }
            }
            None => {
                panic!("missing numeric value!");
            }
        }
        panic!()
    }

    fn compare_opertor_prevalenz(a:char, b:char) -> bool {
        let mut map : HashMap<char, u8> = HashMap::new();
        map.insert('+', 1);
        map.insert('-', 1);
        map.insert('*', 2);
        map.insert('/', 2);
        map.insert('c', 2);
        map.insert('f', 2);
        return map[&a] <= map[&b];
    }

    #[derive(Debug, PartialEq, Copy, Clone)]
    enum Token {
        Number(i64),
        Roll((u64, u64)),
        Operator(char),
        BracesOpen,
        BracesClose
    }
}