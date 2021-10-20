//! Understanding Parser Combinators - pt 2
//!
//! [Building a useful set of parser combinators](https://fsharpforfunandprofit.com/posts/understanding-parser-combinators-2/)
use std::rc::Rc;

type ParseFn<'a, T> = dyn Fn(&'a str) -> Result<(&'a str, T), String> + 'a;

struct Parser<'a, T> {
    parse: Rc<ParseFn<'a, T>>,
}

impl<'a, T: 'a> Parser<'a, T> {
    /// Run a parser with some input
    pub fn parse(&self, input: &'a str) -> Result<(&'a str, T), String> {
        (self.parse)(input)
    }
}

/// Parse a single character
fn p_char<'a>(char_to_match: char) -> Parser<'a, char> {
    Parser {
        parse: Rc::new(move |input: &str| {
            let first = input
                .chars()
                .next()
                .ok_or_else(|| "Empty input string".to_string())?;
            if first != char_to_match {
                return Err(format!("Expected '{}', found '{}'", char_to_match, first));
            }

            Ok((&input[1..], first))
        }),
    }
}

/// Combine two parsers as "A andThen B"
fn and_then<'a, T: 'a, U: 'a>(p1: Parser<'a, T>, p2: Parser<'a, U>) -> Parser<'a, (T, U)> {
    Parser {
        parse: Rc::new(move |input: &str| {
            let (remaining, result1) = p1.parse(input)?;
            let (remaining, result2) = p2.parse(remaining)?;
            let new_value = (result1, result2);
            Ok((remaining, new_value))
        }),
    }
}

/// Combine two parsers as "A orElse B"
fn or_else<'a, T: 'a>(p1: Parser<'a, T>, p2: Parser<'a, T>) -> Parser<'a, T> {
    Parser {
        parse: Rc::new(move |input: &str| p1.parse(input).or_else(|_| p2.parse(input))),
    }
}

/// Choose any of a list of parsers
fn choice<'a, T: 'a>(parsers: impl IntoIterator<Item = Parser<'a, T>>) -> Parser<'a, T> {
    parsers.into_iter().reduce(or_else).unwrap()
}

/// Choose any of a list of characters
fn any_of<'a>(char_list: impl IntoIterator<Item = char>) -> Parser<'a, char> {
    let parsers = char_list.into_iter().map(p_char).collect::<Vec<_>>();
    choice(parsers)
}

fn parse_lowercase<'a>() -> Parser<'a, char> {
    any_of('a'..='z')
}

fn parse_digit<'a>() -> Parser<'a, char> {
    any_of('0'..='9')
}

// 2-1. Transforming the contents of a parser with “map”

// add map for Parser
impl<'a, T: 'a> Parser<'a, T> {
    /// apply a function to the value inside a parser
    pub fn map<U>(self, f: impl Fn(T) -> U + 'a) -> Parser<'a, U> {
        Parser {
            parse: Rc::new(move |input: &str| {
                let (remaining, result) = (self.parse)(input)?;
                let mapped_value = f(result);
                Ok((remaining, mapped_value))
            }),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

}
