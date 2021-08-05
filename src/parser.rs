use crate::ast::Expression;
use crate::lexer::lexer;
use crate::token::{Token, Tokens};
use nom::{
    branch::alt,
    bytes::complete::take,
    combinator::peek,
    error::{Error, ErrorKind},
    sequence::{delimited, tuple},
    Err, IResult,
};
use std::ops::RangeFrom;

pub fn token<'a, I>(t: Token<'a>) -> impl Fn(I) -> IResult<I, Token<'a>>
where
    I: nom::Slice<RangeFrom<usize>> + nom::InputIter<Item = &'a Token<'a>>,
{
    move |input: I| match (input).iter_elements().next().map(|tt| {
        let b = *tt == t;
        (&t, b)
    }) {
        Some((t, true)) => Ok((input.slice(1..), t.clone())),
        _ => Err(Err::Error(Error {
            input: input,
            code: ErrorKind::Char,
        })),
    }
}

pub fn identifier(input: Tokens) -> IResult<Tokens, Expression> {
    println! {"identifier input is:  {:?}", input}
    let (input, ret) = take(1usize)(input)?;
    match ret[0] {
        Token::Ident(s) => Ok((input, Expression::Identifier(s))),
        _ => Err(Err::Error(Error {
            input: input,
            code: ErrorKind::Tag,
        })),
    }
}

pub fn sum(input: Tokens) -> IResult<Tokens, Expression> {
    println! {"bin input is:  {:?}", input}
    let (input, (left_exp, _, right_exp)) =
        tuple((expression, token(Token::Plus), expression))(input)?;
    Ok((
        input,
        Expression::Sum(Box::new(left_exp), Box::new(right_exp)),
    ))
}

pub fn mult(input: Tokens) -> IResult<Tokens, Expression> {
    println! {"bin input is:  {:?}", input}
    let (input, (left_exp, _, right_exp)) =
        tuple((expression, token(Token::Mult), expression))(input)?;
    Ok((
        input,
        Expression::Sum(Box::new(left_exp), Box::new(right_exp)),
    ))
}

pub fn parens(input: Tokens) -> IResult<Tokens, Expression> {
    println! {"paren input is:  {:?}", input}
    let (input, _) = peek(token(Token::LParen))(input)?;
    delimited(token(Token::LParen), expression, token(Token::RParen))(input)
}

pub fn expression(input: Tokens) -> IResult<Tokens, Expression> {
    alt((parens, sum, mult, identifier))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_expression() {
        let (_, token_vec) = lexer("1 + (2 + 3) * MyInt").unwrap();
        let (_, parsed_expression) = expression(Tokens::new(&token_vec)).unwrap();
        assert_eq!(
            parsed_expression,
            Expression::Sum(
                Box::new(Expression::Number(1)),
                Box::new(Expression::Mult(
                    Box::new(Expression::Sum(
                        Box::new(Expression::Number(2)),
                        Box::new(Expression::Number(3))
                    )),
                    Box::new(Expression::Identifier("MyInt"))
                ))
            )
        );
    }
}
