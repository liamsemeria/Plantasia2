use crate::Inst::*;
// LEXER

// converts from the the raw file text to vecs of tokens
pub fn lex(file_text: &str) -> Vec<Vec<Token>> {
    let mut tokens = Vec::new();
    for line in file_text.lines() {

        let mut v = Vec::new();
        let mut num_string: String = "".to_string();
        for (i,c) in line.chars().enumerate() {

            // checks for numerical constants
            if c.is_numeric() {num_string.push(c)}
            else {
                if &num_string != "" {
                    v.push(Token::NUM(num_string.parse::<i32>().unwrap()));
                    num_string = "".to_string();
                }
                // checks for simple one char tokens
                match c {
                    '|' => v.push(Token::STACK(i.try_into().unwrap())), // TODO CONVERTS TABS TO SPACES
                    '<' => v.push(Token::LEFT),
                    '>' => v.push(Token::RIGHT),
                    '+'|'-'|'#'|'*'|'%'|'=' => v.push(Token::OP(char_to_Op(c))),
                    '?' => v.push(Token::IF(i.try_into().unwrap())),
                    ':' => v.push(Token::LOOP(i.try_into().unwrap())),
                    '[' => v.push(Token::BRACKETOPEN(i.try_into().unwrap())),
                    ']' => v.push(Token::BRACKETCLOSE(i.try_into().unwrap())),
                    '.' => v.push(Token::RETURN(i.try_into().unwrap())),
                    '$' => v.push(Token::INPUT(i.try_into().unwrap())),
                    '_' =>(),
                    ';' => continue,
                    _ => ()
                }
            }
            // checks for function names
            // TODO 
        }
        // covers the case of the number is the last token in the line
        if &num_string != "" {
            v.push(Token::NUM(num_string.parse::<i32>().unwrap()));
        }
        println!("{:?}", v);
        // dont add empty lines
        if v.len() > 0 {tokens.push(v);}
    }

    return tokens;
}

fn char_to_Op(c: char) -> Op {
    match c {
        '#' => Op::PROPAGATE,
        '+' => Op::ADD,
        '-' => Op::SUB,
        '*' => Op::MUL,
        '%' => Op::MOD,
        '=' => Op::EQ,
         _  => todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // tests lexing for basic functionality
    #[test]
    fn lex_basic() {
        let program = "|<<12|\n| 1>>|\n|<__+|";
        let toks = vec![vec![Token::STACK(0),Token::LEFT,Token::LEFT,Token::NUM(12),Token::STACK(5)],
        vec![Token::STACK(0), Token::NUM(1), Token::RIGHT, Token::RIGHT, Token::STACK(5)],
        vec![Token::STACK(0),Token::LEFT, Token::OP(Op::ADD), Token::STACK(5)]
        ];

        let result = lex(program);
        assert_eq!(toks,result)
    }
    // tests lexing for branches
    #[test]
    fn lex_branch_simple() {
        let program = "|<<2|\n[   |\n|<_-|\n]   |";
        let toks = vec![vec![Token::STACK(0),Token::LEFT,Token::LEFT,Token::NUM(2),Token::STACK(4)],
        vec![Token::BRACKETOPEN(0),Token::STACK(4)],
        vec![Token::STACK(0),Token::LEFT, Token::OP(Op::SUB), Token::STACK(4)],
        vec![Token::BRACKETCLOSE(0),Token::STACK(4)]
        ];

        let result = lex(program);
        assert_eq!(toks,result)
    }
}
