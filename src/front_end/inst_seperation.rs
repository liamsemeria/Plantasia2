use std::error::Error;
use crate::Inst::*;

// first step in the parser seperate the stacks used in the instruction from stacks being kept alive
// for example (|  | |<#|  | ) -> (|  | [|<#|] |) where the instruction is within []
pub fn seperate(line: &Vec<Token>) -> Result<(Vec<u64>,Vec<Token>), Box<dyn Error>> {

    let mut alive_stacks: Vec<u64> = Vec::new();

    let mut prev_token: &Token = &line[0];
    let mut building_instruction = false;
    let mut instruction_tokens: Vec<Token> = Vec::new();
    let mut instruction_built = false; // used to detect invalid syntax 

    for token in line {
        // all stack tokens are noted as alive stacks
        if let Token::STACK(l) = &token {
            alive_stacks.push(*l)
        }
        // stacks being used for a branch condition ore return are also alive
        if let Token::IF(l) | Token::LOOP(l) | Token::RETURN(l) | Token::INPUT(l) = &token { alive_stacks.push(*l) }

        // seperate instruction tokens from non-instruction tokens
        match token {
            Token::STACK(_) | Token::NUM(_) => {
                // if the instruction is being built this must be the last instruction token
                if building_instruction {
                    instruction_tokens.push(token.clone());
                    building_instruction = false;
                    instruction_built = true;

                }
            },
            
            // if a <,>, or OP comes after a stack or num, the instruction started one token ago
            Token::LEFT | Token::RIGHT | Token::OP(_) => {

                if instruction_built {return Err("Instruction Seperation Error! Instruction Tokens after complete instruction".into())}

                if let Token::STACK(_) | Token::NUM(_) = &prev_token {
                    if ! building_instruction {
                        // start building the instructiom
                        building_instruction = true;
                        instruction_tokens.push(prev_token.clone())
                    }
                } else if ! building_instruction {return Err("Instruction Seperation Error! Op tokens before src or dest specified".into())}

                instruction_tokens.push(token.clone())
            },

            // for branches, the instruction started one token ago when [ appears
            Token::BRACKETOPEN(_) => {
                instruction_tokens.push(prev_token.clone()); // the branch and bracketopen arent both needed for the instruction to be parsed
                instruction_built = true;
            }
            Token::BRACKETCLOSE(_) => {
                instruction_tokens.push(token.clone());
                instruction_built = true;
            }
            Token::RETURN(_) | Token::INPUT(_) => {
                instruction_tokens.push(token.clone());
                instruction_built = true;
            },
            _ => ()
        }
        prev_token = &token;
    }
    //println!("alive stacks: {:?}",alive_stacks);
    //println!("instruction tokens: {:?}",instruction_tokens);
    //Ok(())
    return Ok((alive_stacks, instruction_tokens))
}

#[cfg(test)]
mod tests {
    use super::*;
    // tests for instruction seperation starting left and with a constant
    #[test]
    fn inst_seperation_leftside_const() {
        let toks = vec![Token::STACK(0),Token::LEFT,Token::LEFT,Token::NUM(12),Token::STACK(5),Token::STACK(9)];
        let a: Vec<u64> = vec![0,5,9];
        assert_eq!((a,vec![Token::STACK(0),Token::LEFT,Token::LEFT,Token::NUM(12)]), seperate(&toks).unwrap())
    }
    // tests for seperation starting on the right and with an op
    #[test]
    fn inst_seperation_rightside_op() {
        let toks = vec![Token::STACK(0),Token::STACK(5),Token::OP(Op::PROPAGATE),Token::LEFT,Token::STACK(9)];
        let a: Vec<u64> = vec![0,5,9];
        assert_eq!((a,vec![Token::STACK(5),Token::OP(Op::PROPAGATE),Token::LEFT,Token::STACK(9)]), seperate(&toks).unwrap())
    }
    // tests for detecting invalid instruction syntax |<<2>>|
    #[test]
    fn inst_seperation_invalid1() {
        let toks = vec![Token::STACK(0),Token::LEFT,Token::LEFT,Token::NUM(2),Token::RIGHT,Token::RIGHT,Token::STACK(6)];
        assert!(seperate(&toks).is_err())
    }
    // tests for detecting invalid instruction syntax >>|
    #[test]
    fn inst_seperation_invalid2() {
        let toks = vec![Token::RIGHT,Token::RIGHT,Token::STACK(2)];
        assert!(seperate(&toks).is_err())
    }
}