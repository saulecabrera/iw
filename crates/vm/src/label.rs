pub enum Label {
    If(usize),
    Block(usize), 
    Ret,
    // (arity, continuation)
    // The continuation of a loop is a given instruction index;
    // which is effectively the index of the loop instruction.
    // TODO: Replace instruction index with right abstraction
    Loop(usize, usize),
}
