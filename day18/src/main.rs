use std::io::{self, Read};
#[derive(Clone, PartialEq, Debug)]
enum Token {
    Num(u64),
    LParen,
    RParen,
    Plus,
    Mul,
}

fn operand(tokens: &[Token], pos: usize) -> (u64, usize) {
    match tokens[pos] {
        Token::Num(n) => (n, pos + 1),
        Token::LParen => evaluate(tokens, pos + 1),
        _ => panic!("Expected operand!"),
    }
}

fn evaluate(tokens: &[Token], pos: usize) -> (u64, usize) {
    let (mut exp_value, mut pos) = operand(&tokens, pos);

    while pos < tokens.len() {
        match tokens[pos] {
            Token::Plus => {
                let (rhs, new_pos) = operand(&tokens, pos + 1);
                exp_value += rhs;
                pos = new_pos;
            }
            Token::Mul => {
                let (rhs, new_pos) = operand(&tokens, pos + 1);
                exp_value *= rhs;
                pos = new_pos;
            }
            _ => break,
        }
    }

    (exp_value, pos + 1)
}

fn parse(line: &str) -> u64 {
    let mut tokens: Vec<Token> = Vec::new();

    for char in line.chars() {
        match char {
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '+' => tokens.push(Token::Plus),
            '*' => tokens.push(Token::Mul),
            n if n.is_numeric() => tokens.push(Token::Num(n.to_digit(10).unwrap() as u64)),
            _ => continue,
        }
    }

    let (v, _) = evaluate(&tokens, 0);
    v
}

fn run1(input: &str) -> u64 {
    let mut sum: u64 = 0;

    for line in input.lines() {
        sum += parse(line);
    }

    sum
}

fn run2(input: &str) -> u64 {
    let mut sum: u64 = 0;

    for line in input.lines() {
        let line = format!(
            "({})",
            line.replace("(", "((")
                .replace(")", "))")
                .replace(" * ", ") * (")
        );

        println!("{:?}", line);
        sum += parse(&line);
    }

    sum
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{:?}", run1(&input));
    println!("{:?}", run2(&input));
}
