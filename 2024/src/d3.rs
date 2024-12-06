use logos::Logos;
#[derive(Debug, Logos, Clone, Copy, PartialEq)]
#[logos(skip r"")]
enum Token {
    #[token("do")]
    FuncDo,
    #[token("don't")]
    FuncDont,
    #[token("mul")]
    FuncMul,
    #[token("(")]
    ParenOpen,

    #[regex(r"\d+", |lex| lex.slice().parse::<u32>().unwrap())]
    Number(u32),

    #[token(",")]
    Comma,

    #[token(")")]
    ParenClose,

    #[regex("_", priority = 1000)]
    Mess,
    #[regex(" \t\r\n]+")]
    Space,
}

pub fn solve1(raw_input: &str) -> String {
    let input: Vec<String> = {
        raw_input
            .split("\n")
            .filter(|f| !f.is_empty())
            .map(|f| f.to_owned())
            .collect()
    };

    let sum: u32 = input
        .iter()
        .map(|f| {
            let rtokens = Token::lexer(f);

            let tokens: Vec<Token> = rtokens.map(|f| f.unwrap_or(Token::Mess)).collect();

            return tokens.windows(6).map(|tt| match tt[0..6] {
                    [
                        Token::FuncMul,
                        Token::ParenOpen,
                        Token::Number(a),
                        Token::Comma,
                        Token::Number(b),
                        Token::ParenClose
                    ] => a*b,
                    _ => 0,

        }).sum::<u32>();
        })
        .sum();

    return sum.to_string();
}
pub fn solve2(f: &str) -> String {
    let rtokens = Token::lexer(&f);

    let tokens: Vec<Token> = rtokens.map(|f| f.unwrap_or(Token::Mess)).collect();

    let mut active = true;
    let mut count = 0;

    let mut itr = tokens.iter();

    let mut buffer: [Token; 6] = [Token::Mess; 6];

    while let Some(val) = itr.next() {
        buffer.rotate_left(1);
        buffer[buffer.len() - 1] = *val;

        match buffer {
            [Token::FuncMul, Token::ParenOpen, Token::Number(a), Token::Comma, Token::Number(b), Token::ParenClose] => {
                if active {
                    count += a * b
                }
            }
            [Token::FuncDo, Token::ParenOpen, Token::ParenClose, ..] => {
                active = true;
            }
            [Token::FuncDont, Token::ParenOpen, Token::ParenClose, ..] => {
                active = false;
            }
            _ => {}
        }
    }

    return count.to_string();
}
