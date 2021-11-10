//! Understanding Parser Combinators part 1
//!
//! [Parser combinator article](https://fsharpforfunandprofit.com/posts/understanding-parser-combinators/)
//! (un)Organized in the same sequence presented in the articles

// Parsing a specified character
// fn p_char(char_to_match: char, input: &str) -> Result<(&str, char), String> {
// Switching to a curried implementation
pub fn p_char(char_to_match: char) -> impl Fn(&str) -> Result<(&str, char), String> {
    move |input| {
        let first = input
            .chars()
            .next()
            .ok_or_else(|| "Empty input string".to_string())?;
        if first != char_to_match {
            return Err(format!("Expected '{}', found '{}'", char_to_match, first));
        }

        Ok((&input[1..], first))
    }
}

// Encapsulating the parsing function in a type
type ParseFn<'a, T> = dyn Fn(&'a str) -> Result<(&'a str, T), String> + 'a;

pub struct Parser<'a, T>(Box<ParseFn<'a, T>>);

pub fn p_char2<'a>(char_to_match: char) -> Parser<'a, char> {
    Parser(Box::new(move |input: &str| {
        let first = input
            .chars()
            .next()
            .ok_or_else(|| "Empty input string".to_string())?;
        if first != char_to_match {
            return Err(format!("Expected '{}', found '{}'", char_to_match, first));
        }

        Ok((&input[1..], first))
    }))
}

// Combining two parsers in sequence

/// Combine two parsers as "A andThen B"
pub fn and_then<'a, T: 'a, U: 'a>(p1: Parser<'a, T>, p2: Parser<'a, U>) -> Parser<'a, (T, U)> {
    Parser(Box::new(move |input: &str| {
        let (remaining, result1) = p1.0(input)?;
        let (remaining, result2) = p2.0(remaining)?;
        let new_value = (result1, result2);
        Ok((remaining, new_value))
    }))
}

// Choosing between two parsers

/// Combine two parsers as "A orElse B"
pub fn or_else<'a, T: 'a>(p1: Parser<'a, T>, p2: Parser<'a, T>) -> Parser<'a, T> {
    Parser(Box::new(move |input: &str| {
        p1.0(input).or_else(|_| p2.0(input))
    }))
}

// Choosing from a list of parsers

/// Choose any of a list of parsers
pub fn choice<'a, T: 'a>(parsers: impl IntoIterator<Item = Parser<'a, T>>) -> Parser<'a, T> {
    parsers.into_iter().reduce(or_else).unwrap()
}

/// Choose any of a list of characters
pub fn any_of<'a>(char_list: impl IntoIterator<Item = char>) -> Parser<'a, char> {
    let parsers = char_list.into_iter().map(p_char2).collect::<Vec<_>>();
    choice(parsers)
}

pub fn parse_lowercase<'a>() -> Parser<'a, char> {
    any_of('a'..='z')
}

pub fn parse_digit<'a>() -> Parser<'a, char> {
    any_of('0'..='9')
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pchar() {
        let msg = "should parse an 'a'";
        let expected = ("bc", 'a');
        let actual = p_char('a')("abc").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = "Expected 'a', found 'z'";
        let actual = p_char('a')("zbc").unwrap_err();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn pchar2() {
        let msg = "should parse an 'a'";
        let expected = ("bc", 'a');
        let parse_a = p_char2('a');
        let actual = parse_a.0("abc").unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn comb_and_then() {
        let msg = "should parse an 'a' then a 'b'";

        let parse_a = p_char2('a');
        let parse_b = p_char2('b');
        let parse_a_then_b = and_then(parse_a, parse_b);

        let expected = ("c", ('a', 'b'));
        let actual = parse_a_then_b.0("abc").unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn comb_or_else() {
        let msg = "should parse an 'A' or 'B'";

        let parse_a = p_char2('A');
        let parse_b = p_char2('B');
        let parse_a_or_else_b = or_else(parse_a, parse_b);

        let expected = ("ZZ", 'A');
        let actual = parse_a_or_else_b.0("AZZ").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = ("ZZ", 'B');
        let actual = parse_a_or_else_b.0("BZZ").unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn combine_combinators() {
        // With these two basic combinators, we can build more complex ones,
        // such as “A and then (B or C)”.
        let msg = "should parse an 'A' then ('C' or 'B')";

        let parse_a = p_char2('A');
        let parse_b = p_char2('B');
        let parse_c = p_char2('C');
        let b_or_c = or_else(parse_b, parse_c);
        let a_then_b_or_c = and_then(parse_a, b_or_c);

        let expected = ("Z", ('A', 'B'));
        let actual = a_then_b_or_c.0("ABZ").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = ("Z", ('A', 'C'));
        let actual = a_then_b_or_c.0("ACZ").unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn any_of_char() {
        let msg = "should parse any of the specified characters";

        let parse_lowercase = parse_lowercase();
        let parse_digit = parse_digit();

        let expected = ("BC", 'a');
        let actual = parse_lowercase.0("aBC").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = ("ABC", '1');
        let actual = parse_digit.0("1ABC").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = ("ABC", '9');
        let actual = parse_digit.0("9ABC").unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }
}
