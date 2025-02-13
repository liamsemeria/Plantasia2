// how instructions are stored
#[derive(Debug, PartialEq)]
pub struct Inst {
    pub alive_stacks: Vec<u64>,
    pub statement: Statement
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Assign{dest: u64, expression: Exp},
    If{comparison: u64, body: Vec<Inst>},
    While{comparison: u64, body: Vec<Inst>},
    Return{src: u64},
    None
}

#[derive(Debug, PartialEq)]
pub struct Exp {
    pub src: Value,
    pub op: Op
}

#[derive(Debug, PartialEq)]
pub enum Value {
    STACK(u64),
    CONST(i32)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    ADD,
    SUB,
    MUL,
    MOD,
    EQ,
    PROPAGATE,
    POP
}
// used by the front_end only
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    STACK(u64),
    LEFT,
    RIGHT,
    NUM(i32),
    OP(Op),
    IF(u64),
    LOOP(u64),
    BRACKETOPEN(u64),
    BRACKETCLOSE(u64),
    RETURN(u64)
}