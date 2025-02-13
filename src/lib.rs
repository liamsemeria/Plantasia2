use std::error::Error;

pub mod Inst;
pub mod front_end;

pub fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.len() < 1 { return Err("No file name provided".into()) }
	// read file
    // DETATCH ARGS FROM FILENAME STRING AND MAKE TEST DIRECTORY
    let ast = front_end::run(&args[1]);
    if let Err(e) = ast { return Err(e)}
    else {
        traversals::interpret(ast.unwrap());
        return Ok(());
    }
	//return front_end::run(args);
}


mod traversals {

use std::collections::HashMap;
use std::error::Error;
use crate::Inst::*;


    pub fn interpret(ast: Vec<Inst>) -> Vec<i32> {
        let mut stacks: HashMap<u64, Vec<i32>> = HashMap::new();
        let mut result: Vec<i32> = vec![];
        if let Ok(Some(r)) = interpret_R(&mut ast.iter(),&mut stacks) {result = r;}
        println!("INTERPRETER RESULT");
        println!("{:?}",result);
        return result;
    }
    fn interpret_R(ast: &mut std::slice::Iter<'_, Inst>, stacks: &mut HashMap<u64, Vec<i32>>) -> Result<Option<Vec<i32>>, Box<dyn Error>> {
        match ast.next() {
            None => {return Ok(None);}
            Some(i) => {
                // remove any stacks that are no longer alive
                stacks.retain(|&k, _| i.alive_stacks.contains(&k)); // this has performance impacts since its O(capacity) not O(len)
                
                // add new stacks if needed
                for s in &i.alive_stacks {
                    if ! stacks.contains_key(s) {
                        stacks.insert(*s, vec![]);
                    }
                }
                match &i.statement {
                    Statement::While{comparison: c, body: b} => {
                        while stacks[c].last() > Some(&0) {
                            interpret_R(&mut b.iter(), stacks);
                        }
                    }
                    Statement::If{comparison: c, body: b} => {
                        if stacks[c].last() > Some(&0) {
                            interpret_R(&mut b.iter(), stacks);
                        }
                    }
                    Statement::Assign{dest: d, expression: Exp{src: s, op: o}} => {
                        // TODO check if dest stack exists, can probably just do it by replacing unwrap with something else
                        // get src value, pop from the src stack if it isnt a const
                        let v = match s {
                            Value::STACK(s) => { stacks.get_mut(s).expect("expecting alive src stack").pop().expect("expecting non-empty src stack") }
                            Value::CONST(c) => { *c }
                        };
                        let mut d_stack = stacks.get_mut(d).expect("expecting alive dest stack");
                        let d_len = d_stack.len();
                        //if let o = Op::POP | Op::PROPAGATE {if d_stack.is_empty() { return Err("expecting non-non empty dest".into()) }}
                        match o {
                            Op::PROPAGATE => {
                                d_stack.push( v ); // add the value back to the src stack
                                if let Value::STACK(s) = s {stacks.get_mut(s).expect("expecting alive src stack").push(v);}
                            }
                            Op::POP => { d_stack.push( v )}
                            o => {
                                if d_stack.is_empty() { return Err("trying to read from empty dest stack".into()) }
                                else {
                                    match o {
                                        Op::ADD => { d_stack[d_len-1] = d_stack[d_len-1] + v; }
                                        Op::SUB => {  d_stack[d_len-1] = d_stack[d_len-1] - v;}
                                        Op::MUL => {  d_stack[d_len-1] = d_stack[d_len-1] * v;}
                                        Op::MOD => {  d_stack[d_len-1] = d_stack[d_len-1] % v;}
                                        Op::EQ => {  d_stack[d_len-1] = (d_stack[d_len-1] == v) as i32;}
                                        _ => {}
                                }
                                }
                            }
                        }
                        
                    }
                    Statement::Return{src: s} => {
                        return Ok(Some(stacks[s].clone()));
                    }
                    _ => {

                    }
                }
            }
        }
        return interpret_R(ast, stacks);
    }
    


#[cfg(test)]
mod tests {
    use super::*;
    use crate::front_end;

    // tests lexing for basic functionality
    #[test]
    fn simple_add() {
        let result = interpret(front_end::run(&"./test_programs/simple_add.pa".into()).unwrap());
        assert_eq!(result,[17])
    }
    #[test]
    fn simple_moves() {
        let result = interpret(front_end::run(&"./test_programs/simple_moves.pa".into()).unwrap());
        assert_eq!(result,[3,9])
    }
    #[test]
    fn simple_branch() {
        let result = interpret(front_end::run(&"./test_programs/simple_branch.pa".into()).unwrap());
        assert_eq!(result,[0,1,11])
    }
    #[test]
    fn fib() {
        let result = interpret(front_end::run(&"./test_programs/fib.pa".into()).unwrap());
        assert_eq!(result,[0,1,1,2,3,5])
    }
}

}