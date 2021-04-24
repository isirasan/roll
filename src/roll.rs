pub(crate) mod parser {
    use rand::Rng;
    use std::convert::TryInto;
    use std::ops::{Index, Deref};
    use std::borrow::Borrow;

    pub fn parse(args: &[String]) {
        print!("input {:?}\n", args);
        let split_args = prepare_args(args);
        print!("split {:?}\n", split_args);

        let mut tokens : Vec<Token> = parse_tokens(&split_args);
        print!("token count {:?}\n", tokens);

        let roll_results = roll(&mut tokens);
        print!("roll results {:?}\n", tokens);
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
                    tokens.push(Token::operator_add)
                }
                x if x == "-" => {
                    tokens.push(Token::opperator_sub)
                }
                x if x == "*" => {
                    tokens.push(Token::opperator_mul)
                }
                x if x == "/" => {
                    tokens.push(Token::opperator_div)
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

    #[derive(Debug, PartialEq, Copy, Clone)]
    enum Token {
        number(u64),
        roll((u64,u64)),
        operator_add,
        opperator_mul,
        opperator_sub,
        opperator_div,
        braces_open,
        braces_close
    }
}