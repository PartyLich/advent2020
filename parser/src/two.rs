//! Understanding Parser Combinators - pt 2
//!
//! [Building a useful set of parser combinators](https://fsharpforfunandprofit.com/posts/understanding-parser-combinators-2/)
use std::rc::Rc;

type ParseFn<'a, T> = dyn Fn(&'a str) -> Result<(&'a str, T), String> + 'a;

pub struct Parser<'a, T> {
    parse: Rc<ParseFn<'a, T>>,
}

impl<'a, T: 'a> Parser<'a, T> {
    /// Run a parser with some input
    pub fn parse(&self, input: &'a str) -> Result<(&'a str, T), String> {
        (self.parse)(input)
    }
}

/// Parse a single character
pub fn p_char<'a>(char_to_match: char) -> Parser<'a, char> {
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
pub fn and_then<'a, T: 'a, U: 'a>(p1: Parser<'a, T>, p2: Parser<'a, U>) -> Parser<'a, (T, U)> {
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
pub fn or_else<'a, T: 'a>(p1: Parser<'a, T>, p2: Parser<'a, T>) -> Parser<'a, T> {
    Parser {
        parse: Rc::new(move |input: &str| p1.parse(input).or_else(|_| p2.parse(input))),
    }
}

/// Choose any of a list of parsers
pub fn choice<'a, T: 'a>(parsers: impl IntoIterator<Item = Parser<'a, T>>) -> Parser<'a, T> {
    parsers.into_iter().reduce(or_else).unwrap()
}

/// Choose any of a list of characters
fn any_of<'a>(char_list: impl IntoIterator<Item = char>) -> Parser<'a, char> {
    let parsers = char_list.into_iter().map(p_char).collect::<Vec<_>>();
    choice(parsers)
}

pub fn parse_lowercase<'a>() -> Parser<'a, char> {
    any_of('a'..='z')
}

pub fn parse_digit<'a>() -> Parser<'a, char> {
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

// 2-2. Lifting functions to the world of Parsers
trait Pointed<T> {
    /// Lift a value to a context
    fn of(value: T) -> Self;
}

impl<'a, T: 'a> Pointed<T> for Parser<'a, T>
where
    T: Clone,
{
    fn of(value: T) -> Self {
        Parser {
            parse: Rc::new(move |input: &str| {
                // ignore the input and return value
                Ok((input, value.clone()))
            }),
        }
    }
}

// 2-4. Matching a parser multiple times

impl<T> Clone for Parser<'_, T> {
    fn clone(&self) -> Self {
        Parser {
            parse: self.parse.clone(),
        }
    }
}

/// (helper) match zero or more occurrences of the specified parser
fn zero_or_more<'a, T: 'a>(parser: Parser<'a, T>) -> Parser<'a, Vec<T>> {
    Parser {
        parse: Rc::new(move |input: &str| {
            // run parser with the input
            let first_result = parser.parse(input);
            // test the result for Failure/Success
            match first_result {
                // if parse fails, return empty list
                Err(_err) => Ok((input, vec![])),
                // if parse succeeds, call recursively to get the subsequent values
                Ok((input_after_first_parse, first_value)) => {
                    let (remaining_input, mut subsequent_values) = zero_or_more(parser.clone())
                        .parse(input_after_first_parse)
                        .unwrap();
                    let mut values = vec![first_value];
                    values.append(&mut subsequent_values);
                    Ok((remaining_input, values))
                }
            }
        }),
    }
}

/// match zero or more occurrences of the specified parser
pub fn many<'a, T: 'a>(parser: Parser<'a, T>) -> Parser<'a, Vec<T>> {
    Parser {
        parse: Rc::new(move |input: &str| zero_or_more(parser.clone()).parse(input)),
    }
}

/// match one or more occurrences of the specified parser
pub fn one_or_more<'a, T: 'a>(parser: Parser<'a, T>) -> Parser<'a, Vec<T>> {
    Parser {
        parse: Rc::new(move |input: &str| {
            // run parser with the input
            let (input_after_first_parse, first_value) = parser.parse(input)?;

            // if first found, look for zeroOrMore now
            let (remaining_input, mut subsequent_values) = zero_or_more(parser.clone())
                .parse(input_after_first_parse)
                .unwrap();
            let mut values = vec![first_value];
            values.append(&mut subsequent_values);
            Ok((remaining_input, values))
        }),
    }
}

/// parse an integer (without sign support)
pub fn parse_int<'a>() -> Parser<'a, isize> {
    // helper
    fn result_to_int(digits: Vec<char>) -> isize {
        digits
            .into_iter()
            .collect::<String>()
            // ignore int overflow for now
            .parse::<isize>()
            .unwrap()
    }

    // define parser for one digit
    let digit = parse_digit();
    // define parser for one or more digits
    let digits = one_or_more(digit);

    // map the digits to an int
    digits.map(result_to_int)
}

// 2-5. Matching a parser zero or one time

/// Parses an optional occurrence of p and returns an option value.
pub fn optional<'a, T: 'a + Clone>(parser: Parser<'a, T>) -> Parser<'a, Option<T>> {
    let some = parser.map(Option::from);
    let none = Parser::of(None);
    or_else(some, none)
}

/// parse an integer (with sign support)
fn parse_int2<'a>() -> Parser<'a, isize> {
    // helper
    fn result_to_int((sign, digits): (Option<char>, Vec<char>)) -> isize {
        let i = digits
            .into_iter()
            .collect::<String>()
            // ignore int overflow for now
            .parse::<isize>()
            .unwrap();
        match sign {
            Some(_) => -i,
            None => i,
        }
    }

    // define parser for one digit
    let digit = parse_digit();
    // define parser for one or more digits
    let digits = one_or_more(digit);

    // map the digits to an int
    let int = and_then(optional(p_char('-')), digits);
    int.map(result_to_int)
}

// 2-6. Throwing results away
/// Keep only the result of the left side parser
pub fn keep_first<'a, T: 'a, U: 'a>(p1: Parser<'a, T>, p2: Parser<'a, U>) -> Parser<'a, T> {
    // create a pair
    let both = and_then(p1, p2);
    // then only keep the first value
    both.map(|(a, _b)| a)
}

/// Keep only the result of the right side parser
pub fn keep_second<'a, T: 'a, U: 'a>(p1: Parser<'a, T>, p2: Parser<'a, U>) -> Parser<'a, U> {
    // create a pair
    let both = and_then(p1, p2);
    // then only keep the second value
    both.map(|(_a, b)| b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn many_matches() {
        let msg = "should ";

        let many_a = many(p_char('A'));

        let expected = ("BCD", vec!['A']);
        let actual = many_a.parse("ABCD").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = ("CD", vec!['A', 'A']);
        let actual = many_a.parse("AACD").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = ("D", vec!['A', 'A', 'A']);
        let actual = many_a.parse("AAAD").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        // test a case with no matches
        let expected = ("|BCD", vec![]);
        let actual = many_a.parse("|BCD").unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn one_plus() {
        let msg = "should ";

        let digits = one_or_more(parse_digit());

        let expected = ("ABC", vec!['1']);
        let actual = digits.parse("1ABC").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = ("BC", vec!['1', '2']);
        let actual = digits.parse("12BC").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = ("C", vec!['1', '2', '3']);
        let actual = digits.parse("123C").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = ("", vec!['1', '2', '3', '4']);
        let actual = digits.parse("1234").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        // failure case
        let expected = "Expected '9', found 'A'".to_string();
        let actual = digits.parse("ABC").unwrap_err();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn integer() {
        let msg = "should parse an integer";

        let parse_int = parse_int();

        let expected = ("ABC", 1);
        let actual = parse_int.parse("1ABC").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = ("BC", 12);
        let actual = parse_int.parse("12BC").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = ("C", 123);
        let actual = parse_int.parse("123C").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = ("", 1234);
        let actual = parse_int.parse("1234").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        // failure case
        let expected = "Expected '9', found 'A'".to_string();
        let actual = parse_int.parse("ABC").unwrap_err();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn optionals() {
        let msg = "should ";

        let digit = parse_digit();
        let digit_then_semicolon = and_then(digit, optional(p_char(';')));

        let expected = ("", ('1', Some(';')));
        let actual = digit_then_semicolon.parse("1;").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = ("", ('1', None));
        let actual = digit_then_semicolon.parse("1").unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn signed_integer() {
        let msg = "should parse an integer";

        let parse_int = parse_int2();

        let expected = ("C", 123);
        let actual = parse_int.parse("123C").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = ("C", -123);
        let actual = parse_int.parse("-123C").unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn keeps_first() {
        let msg = "should keep the results of the first parser";

        let digit = parse_digit();
        let digit_then_semicolon = keep_first(digit, optional(p_char(';')));

        let expected = ("", '1');
        let actual = digit_then_semicolon.parse("1;").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let actual = digit_then_semicolon.parse("1").unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }
}
