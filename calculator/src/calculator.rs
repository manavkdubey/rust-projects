#[derive(Debug, PrtialEq, Eq, PArtialOrd, Ord)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}
#[derive(Debug, PrtialEq, Eq, PArtialOrd, Ord)]
pub enum Token {
    Number(u32),
    Op(Operator),
    Bracket(char),
}

pub struct Calculator {}

pub enum Error {
    BadToken(char),
    MismatchedParens,
}
