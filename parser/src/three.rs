//! Understanding Parser Combinators - pt 3
//!
//! [Improving the parser library](https://fsharpforfunandprofit.com/posts/understanding-parser-combinators-3/)
use std::rc::Rc;

// 3-1. Labelling a Parser
type ParserLabel = String;
type ParserError = String;
type ParseResult<'a, I, O> = Result<(&'a [I], O), (ParserLabel, ParserError)>;

type ParseFn<'a, I, O> = dyn Fn(&'a [I]) -> ParseResult<'a, I, O> + 'a;

pub struct Parser<'a, I, O> {
    parse: Rc<ParseFn<'a, I, O>>,
    /// Displayable description of this parser
    pub label: String,
}

impl<I, O> std::fmt::Display for Parser<'_, I, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parser {}", self.label)
    }
}

impl<I, O> std::fmt::Debug for Parser<'_, I, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Parser")
            .field("label", &self.label)
            .finish()
    }
}

impl<'a, I: 'a, O: 'a> Parser<'a, I, O> {
    /// Run a parser with some input
    pub fn parse(&self, input: &'a [I]) -> ParseResult<'a, I, O> {
        (self.parse)(input)
    }

    /// return a parser that combines this parser and then other parser
    pub fn and_then<U: 'a>(self, p2: Parser<'a, I, U>) -> Parser<'a, I, (O, U)> {
        and_then(self, p2)
    }

    /// return a parser that combines this parser or else other parser
    pub fn or_else(self, p2: Parser<'a, I, O>) -> Self {
        or_else(self, p2)
    }
}

impl<I, O> Clone for Parser<'_, I, O> {
    fn clone(&self) -> Self {
        Parser {
            parse: self.parse.clone(),
            label: self.label.clone(),
        }
    }
}

/// Print a ParseResult to std out
pub fn print_result<I, O: std::fmt::Debug>(result: &ParseResult<I, O>) {
    match result {
        Ok((_remaining, value)) => {
            println!("{:?}", value);
        }
        Err((label, error)) => {
            println!("Error parsing {}\n\t{}", label, error);
        }
    }
}

/// Combine two parsers as "A andThen B"
pub fn and_then<'a, I: 'a, T: 'a, U: 'a>(
    p1: Parser<'a, I, T>,
    p2: Parser<'a, I, U>,
) -> Parser<'a, I, (T, U)> {
    Parser {
        label: format!("{} and then {}", p1.label, p2.label),
        parse: Rc::new(move |input: &'a [I]| {
            let (remaining, result1) = p1.parse(input)?;
            let (remaining, result2) = p2.parse(remaining)?;
            let new_value = (result1, result2);
            Ok((remaining, new_value))
        }),
    }
}

/// Combine two parsers as "A orElse B"
pub fn or_else<'a, I: 'a, O: 'a>(p1: Parser<'a, I, O>, p2: Parser<'a, I, O>) -> Parser<'a, I, O> {
    Parser {
        label: format!("{} or else {}", p1.label, p2.label),
        parse: Rc::new(move |input: &[I]| p1.parse(input).or_else(|_| p2.parse(input))),
    }
}
