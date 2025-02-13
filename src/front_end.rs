use std::fs;
use std::error::Error;
use crate::Inst::*;

mod lexer;
mod inst_seperation;

pub fn run(filename: &String) -> Result<Vec<Inst>, Box<dyn Error>> {
	// read file
	let file = fs::read_to_string(filename)?;
    println!("input file contents:\n{}", file);
    //inst_seperation(&lex(&file)[0]);
    let mut instructions: Vec<Inst> = vec![];
    if let Err(e) = parse(&mut lexer::lex(&file).iter(), &mut instructions, 0, None) {return Err(e);}
    print_ast(&mut instructions.iter(), 0);
    Ok(instructions)
    
}

fn print_ast(ast: &mut std::slice::Iter<'_, Inst>, indent: i32) -> () {
    for t in 0..indent {
        print!("    ");
    }
    match ast.next() {
        None => {return;}
        Some(i) => {
            match &i.statement {
                Statement::While{comparison: c, body: b} => {
                    println!("Inst {{ alive_stacks: {:?}, statement: While {{ comparison: {:?}, body: ", i.alive_stacks, c);
                    print_ast(&mut b.iter(),indent + 1);
                    println!("}} }}");
                }
                Statement::If{comparison: c, body: b} => {
                    println!("Inst {{ alive_stacks: {:?}, statement: If {{ comparison: {:?}, body: ", i.alive_stacks, c);
                    print_ast(&mut b.iter(),indent + 1);
                    println!("}} }}");
                }
                _ => {println!("{:?}",i);}
            }
        }
    }
    print_ast(ast,indent);
}


//PARSER
// mut program: std::slice::Iter<'_, Vec<Token>, >, body
// parses instruction tokens and verifies they are correct
pub fn parse( program: &mut std::slice::Iter<'_, Vec<Token>, >, body: &mut Vec<Inst>, line: usize, bracket: Option<u64>) -> Result<(), Box<dyn Error>> {

    //let mut instructions: Vec<Inst> = vec![];
        // seperate instruction tokens from non-instruction stacks(lines)
        let sep_result;
        if let Some(p) = program.next() {
            sep_result = inst_seperation::seperate(p);
        } else {return Ok(()) };

        if let Err(e) = sep_result {
            return Err(e);
        }
        let (alive_stacks, inst_tokens) = sep_result.unwrap();

         let mut inst_result: Result<Inst, Box<dyn Error>> = Err("parser messed up ".into());

        match inst_tokens.len() {
            0 => { // instructions can have no statement if it changes the alive stacks 
                inst_result = Ok( Inst{alive_stacks: alive_stacks, statement: Statement::None}) 
            }
            1 => { // branches and returns
                match inst_tokens[0] {
                    Token::LOOP(b) => {
                        let mut new_body: Vec<Inst> = vec![];
                        parse(program, &mut new_body, line + 1, Some(b));
                        inst_result = Ok( Inst{alive_stacks: alive_stacks, statement: Statement::While{comparison: b, body: new_body}}) 
                    }
                    Token::IF(b) => {
                        let mut new_body: Vec<Inst> = vec![];
                        parse(program, &mut new_body, line + 1, Some(b));
                        inst_result = Ok( Inst{alive_stacks: alive_stacks, statement: Statement::If{comparison: b, body: new_body}}) 
                    }
                    Token::BRACKETCLOSE(b) => {
                        if let Some(brack) = bracket {
                            if b == brack {return Ok(());}
                            else { return Err("Parser Error! mismatched or wrong number of brackets".into()) }
                        } else { return Err("Parser Error! misplaced BRACKETCLOSE token".into()) }
                    }
                    Token::RETURN(s) => {
                        inst_result = Ok( Inst{alive_stacks: alive_stacks, statement: Statement::Return{src: s}})
                    }
                    _ => {}
                }
            }
            _ => { // assignments and calls
                assert!(inst_tokens.len() > 1);
                inst_result = parse_multitoken((alive_stacks, inst_tokens));
            }
        }

        if let Err(e) = inst_result {return Err(e);}

        body.push(inst_result?);


    return parse(program, body, line + 1, bracket);
}


// function to parse seperated instructions that span multiple tokens i.e non branch or return instructions
fn parse_multitoken(sep_result: (Vec<u64>,Vec<Token>)) -> Result<Inst, Box<dyn Error>> {
    let (alive_stacks, inst_tokens) = sep_result;


        let mut dest: Option<&u64> = None;
        let mut src_token: Option<&Token> = None;
        let mut op_token: Option<&Token> = None;
        // get rid of directionality
        match &inst_tokens[..] {
            // pattern for dest op src assignments
            [ Token::STACK(d), Token::LEFT, Token::LEFT | Token::OP(_), Token::NUM(_) | Token::STACK(_)] => {
                dest = Some(d);
                src_token = Some(&inst_tokens[3]);
                op_token = Some(&inst_tokens[2]);
            }
            // pattern for src op dest assignments
            [ Token::NUM(_) | Token::STACK(_), Token::RIGHT | Token::OP(_), Token::RIGHT, Token::STACK(d)] => {
                dest = Some(d);
                src_token = Some(&inst_tokens[0]);
                op_token = Some(&inst_tokens[1]);
            }
            _ => {println!("parser: no pattern found in {:?}",&inst_tokens)}
        }
        // now the instruction can finally be built!

        // check if the instruction is an assignent
        if let Some(d) = dest {
            if let Some(op_tok) = op_token {
                // convert directions to pop operations
                let op = match op_tok {
                    Token::LEFT | Token::RIGHT => Op::POP,
                    Token::OP(o) => o.clone(),
                    _ => {
                        return Err("Parser error! unexpected op token".into());
                    }
                };
                if let Some(s_tok) = src_token {
                    // convert from token to value structs to get rid of tokens post parser phase
                    let s = match s_tok {
                        Token::STACK(n) => Value::STACK(n.clone()),
                        Token::NUM(n) => Value::CONST(n.clone()),
                        _ => {
                            return Err("Parser error! invalid src token".into());
                        }
                    };

                    return Ok( Inst {
                        alive_stacks: alive_stacks,
                        statement: Statement::Assign{dest: d.clone(), expression: Exp {
                            src: s,
                            op: op
                        }}
                    });
                }
            }
        }
        return Err("Parser error! unknown multi-token instruction".into())
}

/*
// TESTS
#[cfg(tests)]
mod tests {
    use super::*;
}
*/
