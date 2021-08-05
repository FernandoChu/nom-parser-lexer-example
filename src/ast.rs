#[derive(PartialEq, Debug, Clone)]
pub enum Expression<'a> {
    Number(u32),
    Identifier(&'a str),
    Sum(Box<Expression<'a>>, Box<Expression<'a>>),
    Mult(Box<Expression<'a>>, Box<Expression<'a>>),
}
