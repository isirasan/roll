pub(crate) mod parser {
    use rand::Rng;

    pub fn parse(args: &[String]) {
        print!("input {:?}\n", args);
        let split_args = prepare_args(args);

        print!("split {:?}\n", split_args);
        let mut tokens : Vec<Token> = parse_tokens(&split_args);

        print!("token count {:?}\n", tokens.len());
    }

    fn prepare_args(args: &[String]) -> Vec<String> {
        let mut joined_args = "".to_string();
        joined_args = joined_args.replace(" ", "");

        for (i, entry) in args.iter().enumerate() {
            if i > 0 {
                joined_args.push_str(&entry);
            }
        }

        joined_args = joined_args.replace("+", " + ");
        joined_args = joined_args.replace("*", " * ");
        joined_args = joined_args.replace(")", " ) ");
        joined_args = joined_args.replace("(", " ( ");
        joined_args = joined_args.to_lowercase();

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
                            print!("{}",i);
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

    fn roll(tokens: Vec<Token>) -> Vec<(String, u64)> {
        let mut result: Vec<(String, u64)> = Vec::new();
        let mut rng = rand::thread_rng();
        for entry in tokens {
            if entry == Token::roll {
                let mut r: u64 =0;
                for i in 0..entry.0 {
                    r += rng.gen_range(1..entry.1);
                    print!("{}",r)
                }

            }
        }

        return result;
    }

}