pub(crate) mod parser {
    use rand::Rng;
    use std::convert::TryInto;
    use std::ops::{Index, Deref};
    use std::borrow::Borrow;
    use std::panic::panic_any;
    use std::collections::HashMap;

    pub fn parse(args: &[String]) {
        print!("input {:?}\n", args);
        let split_args = prepare_args(args);
        print!("split {:?}\n", split_args);

        let mut tokens : Vec<Token> = parse_tokens(&split_args);
        print!("token count {:?}\n", tokens);

        roll(&mut tokens);
        print!("roll results {:?}\n", tokens);

        transfer_notation(&mut tokens);
        print!("postfix notation {:?}\n", tokens);
    }

    fn prepare_args(args: &[String]) -> Vec<String> {
        let mut joined_args = "".to_string();

        for (i, entry) in args.iter().enumerate() {
            if i > 0 {
                joined_args.push_str(&entry);
            }
        }
        joined_args = joined_args.replace(" ", "")
            .replace("+", " + ")
            .replace("-", " - ")
            .replace("*", " * ")
            .replace("/", " / ")
            .replace(")", " ) ")
            .replace("(", " ( ")
            .to_lowercase();

        let split: Vec<&str> = joined_args.as_str().split_whitespace().collect();
        let result: Vec<String> = split.iter().map(|s| s.to_string()).collect();
        return result;
    }

    fn parse_tokens(args_list: &Vec<String>) -> Vec<Token>{
        let mut tokens: Vec<Token> = Vec::new();

        for (i, entry) in args_list.iter().enumerate() {
            match entry {
                x if x == "+" => {
                    tokens.push(Token::operator('+'))
                }
                x if x == "-" => {
                    tokens.push(Token::operator('-'))
                }
                x if x == "*" => {
                    tokens.push(Token::operator('*'))
                }
                x if x == "/" => {
                    tokens.push(Token::operator('/'))
                }
                x if x == "(" => {
                    tokens.push(Token::braces_open)
                }
                x if x == ")" => {
                    tokens.push(Token::braces_close)
                }
                _ => {
                    let _ = match entry.parse::<u64>() {
                        Ok(i) => {
                            tokens.push(Token::number(i))
                        },
                        Err(_e) => {
                            let split: Vec<&str> = entry.split("d").collect();
                            let dice: (u64,u64) = (split[0].parse().unwrap_or(0),split[1].parse().unwrap_or(0));
                            tokens.push(Token::roll(dice))
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
            if let Token::roll((count,dice_size)) = entry {
                let mut random: u64 = 0;
                let mut rolls: Vec<u64> = Vec::new();
                for i in 0..*count {
                    random = rng.gen_range(0..*dice_size) + 1;
                    rolls.push(random);
                }

                let sum = rolls.iter().sum();
                print!("{}: {:?}  = {}\n", format!("{}d{}", count.to_string(), dice_size.to_string()), rolls, sum);
                *entry = Token::number(sum);
            }
        }
    }

    fn transfer_notation(tokens:&mut Vec<Token>) {
        let mut stack: Vec<Token> = Vec::new();
        let mut output: Vec<Token> = Vec::new();

        for entry in tokens {
            if let Token::number(num) = entry {
                output.push(*entry)
            } else if let Token::operator(op) = entry {
                while let Some(Token::operator(c)) =  stack.last() {
                    if compare_opertor_prevalenz(*op,*c) {
                        let stack_element = stack.pop();
                        match stack_element {
                            Some(x) => {
                                output.push(x);
                            }
                            None => {
                                panic!()
                            }
                        }
                    }
                }
                stack.push(*entry)
            } else if let Token::braces_open = entry {
                stack.push(*entry)
            } else if let Token::braces_close = entry {
                while stack.last() != Some(&Token::braces_open) {
                    let stack_element = stack.pop();
                    match stack_element {
                        Some(x) => {
                            output.push(x);
                        }
                        None => {
                            panic!()
                        }
                    }

                    if  stack.last() == Some(&Token::braces_open) {
                        stack.pop();
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
                    panic!()
                }
            }
        }
        print!("{:?}\n",output);
    }

    fn compare_opertor_prevalenz(a:char, b:char) -> bool {
        let mut map : HashMap<char, u8> = HashMap::new();
        map.insert('+', 1);
        map.insert('-', 1);
        map.insert('*', 2);
        map.insert('/', 2);

        return map[&a] <= map[&b];
    }

    #[derive(Debug, PartialEq, Copy, Clone)]
    enum Token {
        number(u64),
        roll((u64,u64)),
        operator(char),
        braces_open,
        braces_close
    }
}