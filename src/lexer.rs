use crate::token::Token;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, digit1, multispace0},
    combinator::recognize,
    multi::{many0, many1},
    sequence::{pair, terminated},
    IResult,
};

pub fn number(input: &str) -> IResult<&str, Token> {
    let (input, digits) = terminated(digit1, multispace0)(input)?;
    Ok((input, Token::Number(digits.parse().unwrap())))
}

pub fn plus(input: &str) -> IResult<&str, Token> {
    let (input, _) = terminated(tag("+"), multispace0)(input)?;
    Ok((input, Token::Plus))
}

pub fn mult(input: &str) -> IResult<&str, Token> {
    let (input, _) = terminated(tag("*"), multispace0)(input)?;
    Ok((input, Token::Mult))
}

pub fn lparen(input: &str) -> IResult<&str, Token> {
    let (input, _) = terminated(tag("("), multispace0)(input)?;
    Ok((input, Token::LParen))
}

pub fn rparen(input: &str) -> IResult<&str, Token> {
    let (input, _) = terminated(tag(")"), multispace0)(input)?;
    Ok((input, Token::RParen))
}

pub fn identifier(input: &str) -> IResult<&str, Token> {
    let (input, ident) = terminated(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        multispace0,
    )(input)?;
    Ok((input, Token::Ident(ident)))
}

pub fn lexer(input: &str) -> IResult<&str, Vec<Token>> {
    many1(alt((number, plus, mult, lparen, rparen, identifier)))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_expression() {
        assert_eq!(
            lexer("1 + (2 + 3) * MyInt"),
            Ok((
                "",
                vec![
                    Token::Number(1),
                    Token::Plus,
                    Token::LParen,
                    Token::Number(2),
                    Token::Plus,
                    Token::Number(3),
                    Token::RParen,
                    Token::Mult,
                    Token::Ident("MyInt")
                ]
            ))
        );
    }
}
