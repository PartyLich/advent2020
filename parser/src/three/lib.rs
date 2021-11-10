//! [3-3. Adding position and context to error messages](https://fsharpforfunandprofit.com/posts/understanding-parser-combinators-3/#3-adding-position-and-context-to-error-messages)
use std::fmt;
use std::iter::FromIterator;
use std::rc::Rc;

use super::{ParserError, ParserLabel};

#[derive(Debug, Default, Clone, Copy)]
struct Position {
    line: usize,
    column: usize,
}

/// increment the column number
fn incr_col(pos: Position) -> Position {
    Position {
        column: pos.column + 1,
        ..pos
    }
}

/// increment the line number and set the column to 0
fn incr_line(pos: Position) -> Position {
    Position {
        line: pos.line + 1,
        column: 0,
    }
}

/// Define the current parser input state
#[derive(Debug, Default, Clone)]
pub struct InputState<'a> {
    lines: Vec<&'a str>,
    position: Position,
}

/// parse an InputState from a str
impl<'a> From<&'a str> for InputState<'a> {
    fn from(input: &'a str) -> Self {
        if input.is_empty() {
            return InputState {
                lines: vec![],
                ..Default::default()
            };
        }

        InputState {
            lines: input.lines().collect::<Vec<&'a str>>(),
            ..Default::default()
        }
    }
}

/// return the current line
fn current_line<'a>(input: &InputState<'a>) -> &'a str {
    let line_pos = input.position.line;
    if line_pos < input.lines.len() {
        input.lines[line_pos]
    } else {
        "end of file"
    }
}

/// Get the next character from the input, if any, otherwise return None.
/// Also return the updated InputState
/// InputState -> InputState * char option
fn next_char(input: InputState<'_>) -> (InputState<'_>, Option<char>) {
    let line_pos = input.position.line;
    let col_pos = input.position.column;
    // three cases
    // 1) if line >= maxLine ->
    //       return EOF
    // 2) if col less than line length ->
    //       return char at colPos, increment colPos
    // 3) if col at line length ->
    //       return NewLine, increment linePos
    if line_pos >= input.lines.len() {
        (input, None)
    } else {
        let current_line = current_line(&input);
        if col_pos < current_line.len() {
            let ch = current_line.chars().nth(col_pos);
            let new_pos = incr_col(input.position);
            let new_state = InputState {
                position: new_pos,
                ..input
            };

            (new_state, ch)
        } else {
            // end of line, so return LF and move to next line
            let ch = '\n';
            let new_pos = incr_line(input.position);
            let new_state = InputState {
                position: new_pos,
                ..input
            };

            (new_state, Some(ch))
        }
    }
}

#[allow(dead_code)]
fn read_all_chars(input: InputState<'_>) -> Vec<char> {
    let mut result = vec![];
    let (remaining_input, char_opt) = next_char(input);
    match char_opt {
        None => {}
        Some(ch) => {
            // return first character
            result.push(ch);
            // return the remaining characters
            result.extend(read_all_chars(remaining_input));
        }
    }

    result
}

/// Stores information about the parser position for error messages
#[derive(Debug)]
struct ParserPosition<'a> {
    /// Current line as a str
    current_line: &'a str,
    /// Current line number
    line: usize,
    /// Current column within the current line
    column: usize,
}

// We’ll need some way to convert a InputState into a ParserPosition:
impl<'a> From<InputState<'a>> for ParserPosition<'a> {
    fn from(input: InputState<'a>) -> Self {
        Self {
            current_line: current_line(&input),
            line: input.position.line,
            column: input.position.column,
        }
    }
}

#[derive(Debug)]
pub struct ParseErr<'a>(ParserLabel, ParserError, ParserPosition<'a>);

pub type ParseResult<'a, O> = Result<(InputState<'a>, O), ParseErr<'a>>;

// In addition, the Parser type needs to change from string to InputState:
type ParseFn<'a, O> = dyn Fn(InputState<'a>) -> ParseResult<'a, O> + 'a;

pub struct Parser<'a, O> {
    parse: Rc<ParseFn<'a, O>>,
    /// Displayable description of this parser
    pub label: String,
}

impl std::error::Error for ParseErr<'_> {}

impl fmt::Display for ParseErr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ParseErr(label, error, parser_pos) = self;
        let error_line = parser_pos.current_line;
        let col_pos = parser_pos.column;
        let line_pos = parser_pos.line;
        // pad and right align caret
        let failure_caret = format!("{:>width$}^{}", "", error, width = col_pos,);
        write!(
            f,
            "Line:{} Col:{} Error parsing {}\n{}\n{}",
            line_pos, col_pos, label, error_line, failure_caret
        )
    }
}

/// Return the String representation of a ParseResult
pub fn print_result<O>(result: &ParseResult<O>) -> String
where
    O: std::fmt::Debug,
{
    match result {
        Ok((_input, value)) => format!("{:?}", value),
        Err(err) => format!("{}", err),
    }
}

impl<O> std::fmt::Display for Parser<'_, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parser {}", self.label)
    }
}

impl<O> std::fmt::Debug for Parser<'_, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Parser")
            .field("label", &self.label)
            .finish()
    }
}

impl<O> Clone for Parser<'_, O> {
    fn clone(&self) -> Self {
        Parser {
            parse: self.parse.clone(),
            label: self.label.clone(),
        }
    }
}

// fixing up the `run` function
impl<'a, O: 'a> Parser<'a, O> {
    /// Run the parser on a InputState
    pub fn parse(&self, input: &'a str) -> ParseResult<'a, O> {
        (self.parse)(input.into())
    }

    /// Run the parser on a string
    fn parse_input(&self, input: InputState<'a>) -> ParseResult<'a, O> {
        (self.parse)(input)
    }

    /// Lift a value to a context
    pub fn of(value: O) -> Self
    where
        O: Clone,
    {
        Parser {
            label: "unknown".to_string(),
            parse: Rc::new(move |input: InputState| {
                // ignore the input and return value
                Ok((input, value.clone()))
            }),
        }
    }

    /// apply a function to the value inside a parser
    pub fn map<U>(self, f: impl Fn(O) -> U + 'a) -> Parser<'a, U> {
        let Self { parse, label } = self;
        Parser {
            label,
            parse: Rc::new(move |input: InputState| {
                let (remaining, result) = (parse)(input)?;
                let mapped_value = f(result);
                Ok((remaining, mapped_value))
            }),
        }
    }

    /// Update the label in the parser
    pub fn with_label(self, label: String) -> Self {
        let Self { parse, label: _ } = self;
        Parser {
            label: label.clone(),
            parse: Rc::new(move |input: InputState| {
                (parse)(input)
                    .map_err(|ParseErr(_old_label, err, pos)| ParseErr(label.clone(), err, pos))
            }),
        }
    }

    /// return a parser that combines this parser and then other parser
    pub fn and_then<U: 'a>(self, other: Parser<'a, U>) -> Parser<'a, (O, U)> {
        and_then(self, other)
    }

    /// return a parser that combines this parser or else other parser
    pub fn or_else(self, other: Parser<'a, O>) -> Self {
        or_else(self, other)
    }

    pub fn bind<U: 'a>(self, f: impl Fn(O) -> Parser<'a, U> + 'a) -> Parser<'a, U> {
        bind(self, f)
    }
}

// more idiomatic than `of` in Rust
impl<'a, O: 'a> From<O> for Parser<'a, O>
where
    O: Clone,
{
    fn from(value: O) -> Self {
        Parser::of(value)
    }
}

/// apply the function contents of one functor to the value contents of another functor
pub fn apply<'a, A: 'a, B: 'a>(
    f: Parser<'a, Rc<impl Fn(A) -> B + 'a + ?Sized>>,
    x: Parser<'a, A>,
) -> Parser<'a, B> {
    let fx = and_then(f, x);
    fx.map(|(f, x)| f(x))
}

/// takes a parser-producing function `f` and a parser `p1`, and passes the output of `p1` into
/// `f` to create a new parser
pub fn bind<'a, T: 'a, U: 'a>(
    p1: Parser<'a, T>,
    f: impl Fn(T) -> Parser<'a, U> + 'a,
) -> Parser<'a, U> {
    Parser {
        label: "unknown".to_string(),
        parse: Rc::new(move |input: InputState| {
            let (remaining, result1) = p1.parse_input(input)?;
            // apply f to get a new parser
            let p2 = f(result1);
            // run parser with remaining input
            p2.parse_input(remaining)
        }),
    }
}

/// Combine two parsers as "A andThen B"
pub fn and_then<'a, T: 'a, U: 'a>(p1: Parser<'a, T>, p2: Parser<'a, U>) -> Parser<'a, (T, U)> {
    Parser {
        label: format!("{} and then {}", p1.label, p2.label),
        parse: Rc::new(move |input: InputState| {
            let (remaining, result1) = p1.parse_input(input)?;
            let (remaining, result2) = p2.parse_input(remaining)?;
            let new_value = (result1, result2);
            Ok((remaining, new_value))
        }),
    }
}

/// Combine two parsers as "A orElse B"
pub fn or_else<'a, O: 'a>(p1: Parser<'a, O>, p2: Parser<'a, O>) -> Parser<'a, O> {
    Parser {
        label: format!("{} or else {}", p1.label, p2.label),
        parse: Rc::new(move |input: InputState| {
            p1.parse_input(input.clone())
                .or_else(|_| p2.parse_input(input))
        }),
    }
}

/// Choose any of a list of parsers
pub fn choice<'a, O: 'a>(parsers: impl IntoIterator<Item = Parser<'a, O>>) -> Parser<'a, O> {
    parsers.into_iter().reduce(or_else).unwrap()
}

/// (helper) match zero or more occurrences of the specified parser
fn zero_or_more<'a, O: 'a>(parser: Parser<'a, O>) -> Parser<'a, Vec<O>> {
    let label = format!("zero or more {}", parser.label);
    Parser {
        label,
        parse: Rc::new(move |input: InputState| {
            // run parser with the input
            let first_result = parser.parse_input(input.clone());
            // test the result for Failure/Success
            match first_result {
                // if parse fails, return empty list
                Err(_err) => Ok((input, vec![])),
                // if parse succeeds, call recursively to get the subsequent values
                Ok((input_after_first_parse, first_value)) => {
                    let (remaining_input, subsequent_values) = zero_or_more(parser.clone())
                        .parse_input(input_after_first_parse)
                        .unwrap();
                    let mut values = vec![first_value];
                    values.extend(subsequent_values);
                    Ok((remaining_input, values))
                }
            }
        }),
    }
}

/// match zero or more occurrences of the specified parser
pub fn many<'a, T: 'a>(parser: Parser<'a, T>) -> Parser<'a, Vec<T>> {
    let label = format!("many {}", parser.label);
    Parser {
        label,
        parse: Rc::new(move |input: InputState| zero_or_more(parser.clone()).parse_input(input)),
    }
}

/// match one or more occurrences of the specified parser
pub fn one_or_more<'a, T: 'a>(parser: Parser<'a, T>) -> Parser<'a, Vec<T>> {
    let label = format!("one or more {}", parser.label);
    Parser {
        label: label.clone(),
        parse: Rc::new(move |input: InputState| {
            // run parser with the input
            let (input_after_first_parse, first_value) = parser.parse_input(input)?;

            // if first found, look for zeroOrMore now
            let (remaining_input, mut subsequent_values) = zero_or_more(parser.clone())
                .parse_input(input_after_first_parse)
                .unwrap();
            let mut values = vec![first_value];
            values.append(&mut subsequent_values);
            Ok((remaining_input, values))
        }),
    }
    .with_label(label)
}

/// Convert a list of Parsers into a Parser of a list
pub fn sequence<'a, O: 'a>(list: &[Parser<'a, O>]) -> Parser<'a, Vec<O>>
where
    O: Clone + fmt::Debug,
{
    list.iter().cloned().fold(Parser::of(vec![]), |acc, next| {
        and_then(acc, next).map(|(mut a, b)| {
            a.push(b);
            a
        })
    })
}

/// Parses an optional occurrence of parser and returns an Option value.
pub fn optional<'a, O>(parser: Parser<'a, O>) -> Parser<'a, Option<O>>
where
    O: 'a + Clone + fmt::Debug,
{
    let some = parser.map(Option::from);
    let none = Parser::of(None);
    or_else(some, none)
}

/// Match an input token if the predicate is satisfied
pub fn satisfy<'a>(predicate: impl Fn(char) -> bool + 'a, label: String) -> Parser<'a, char> {
    Parser {
        label: label.clone(),
        parse: Rc::new(move |input: InputState| {
            let (remaining_input, char_opt) = next_char(input.clone());

            match char_opt {
                None => {
                    let err = "No more input".to_string();
                    let pos = input.into();

                    Err(ParseErr(label.clone(), err, pos))
                }
                Some(first) => {
                    if !predicate(first) {
                        let err = format!("Unexpected {:?}", first);
                        let pos = input.into();

                        return Err(ParseErr(label.clone(), err, pos));
                    }

                    Ok((remaining_input, first))
                }
            }
        }),
    }
}

/// Keep only the result of the left side parser
pub fn keep_first<'a, T: 'a, U: 'a>(p1: Parser<'a, T>, p2: Parser<'a, U>) -> Parser<'a, T> {
    // create a pair
    and_then(p1, p2)
        // then only keep the first value
        .map(|(a, _b)| a)
}

/// Keep only the result of the right side parser
pub fn keep_second<'a, T: 'a, U: 'a>(p1: Parser<'a, T>, p2: Parser<'a, U>) -> Parser<'a, U> {
    // create a pair
    and_then(p1, p2)
        // then only keep the second value
        .map(|(_a, b)| b)
}

/// Keep only the result of the middle parser
pub fn between<'a, T: 'a, U: 'a, V: 'a>(
    p1: Parser<'a, T>,
    p2: Parser<'a, U>,
    p3: Parser<'a, V>,
) -> Parser<'a, U> {
    keep_first(keep_second(p1, p2), p3)
}

/// Parses one or more occurrences of parser separated by separator
pub fn sep_by_one<'a, T: 'a, U: 'a>(
    parser: Parser<'a, T>,
    separator: Parser<'a, U>,
) -> Parser<'a, Vec<T>> {
    let sep_then_p = keep_second(separator, parser.clone());
    parser.and_then(many(sep_then_p)).map(|(first, mut rest)| {
        // prepend
        rest.splice(0..0, [first]);
        rest
    })
}

/// Parses zero or more occurrences of parser separated by separator
pub fn sep_by<'a, T: 'a + Clone, U: 'a>(
    parser: Parser<'a, T>,
    separator: Parser<'a, U>,
) -> Parser<'a, Vec<T>> {
    sep_by_one(parser, separator).or_else(Parser::of(vec![]))
}

// 3-4. Adding some standard parsers to the library

/// Parse a single character
pub fn p_char<'a>(char_to_match: char) -> Parser<'a, char> {
    let predicate = move |ch: char| ch == char_to_match;
    let label = format!("{}", char_to_match);
    satisfy(predicate, label)
}

/// Choose any of a list of characters
pub fn any_of<'a>(char_list: impl IntoIterator<Item = char> + fmt::Debug) -> Parser<'a, char> {
    let label = format!("anyOf {:?}", char_list);
    let parsers = char_list.into_iter().map(p_char).collect::<Vec<_>>();
    choice(parsers).with_label(label)
}

/// Parses a sequence of zero or more chars with the char parser cp.
/// It returns the parsed chars as a string.
pub fn many_chars(cp: Parser<'_, char>) -> Parser<'_, String> {
    many(cp).map(String::from_iter)
}

/// Parses a sequence of one or more chars with the char parser cp.
/// It returns the parsed chars as a string.
pub fn one_or_more_chars(cp: Parser<'_, char>) -> Parser<'_, String> {
    one_or_more(cp).map(String::from_iter)
}

/// Parse a specific string
pub fn p_string<'a>(string: &str) -> Parser<'a, String> {
    let label = string.to_string();
    let parsers = string.chars().map(p_char).collect::<Vec<_>>();
    sequence(&parsers).map(String::from_iter).with_label(label)
}

/// parse a single whitespace character
pub fn whitespace_char<'a>() -> Parser<'a, char> {
    let predicate = |ch: char| ch.is_whitespace();
    let label = "whitespace".to_string();
    satisfy(predicate, label)
}

/// parse zero or more whitespace char
pub fn spaces<'a>() -> Parser<'a, Vec<char>> {
    many(whitespace_char())
}

/// parse one or more whitespace char
pub fn one_or_more_spaces<'a>() -> Parser<'a, Vec<char>> {
    one_or_more(whitespace_char())
}

/// parse a single digit
pub fn digit_char<'a>(base: u32) -> Parser<'a, char> {
    let predicate = move |ch: char| ch.is_digit(base);
    let label = "digit".to_string();
    satisfy(predicate, label)
}

/// parse an integer (with sign support)
pub fn p_int<'a>(base: u32) -> Parser<'a, isize> {
    // helper
    fn result_to_int((sign, digits): (Option<char>, Vec<char>)) -> isize {
        let i = String::from_iter(digits)
            // ignore int overflow for now
            .parse::<isize>()
            .unwrap();
        match sign {
            Some(_) => -i,
            None => i,
        }
    }

    let label = "integer".to_string();
    // define parser for one or more digits
    let digits = one_or_more(digit_char(base));

    // an "int" is optional sign + one or more digits
    optional(p_char('-'))
        .and_then(digits)
        .map(result_to_int)
        .with_label(label)
}

type ParsedFloat = ((Option<char>, Vec<char>), char);

// parse a float
pub fn p_float<'a>(base: u32) -> Parser<'a, f64> {
    // helper
    fn result_to_float((((sign, digits), _point), digits2): (ParsedFloat, Vec<char>)) -> f64 {
        let i = format!(
            "{}.{}",
            String::from_iter(digits),
            String::from_iter(digits2)
        )
        .parse::<f64>()
        .unwrap();
        match sign {
            Some(_) => -i,
            None => i,
        }
    }

    let label = "float".to_string();
    // define parser for one or more digits
    let digits = one_or_more(digit_char(base));

    // a float is sign, digits, point, digits (ignore exponents for now)
    optional(p_char('-'))
        .and_then(digits.clone())
        .and_then(p_char('.'))
        .and_then(digits)
        .map(result_to_float)
        .with_label(label)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn all_chars() {
        let msg = "should return a Vec of char";
        let expected = vec![];
        let actual = read_all_chars("".into());
        assert_eq!(actual, expected, "{}", msg);

        let expected = vec!['a', '\n'];
        let actual = read_all_chars("a".into());
        assert_eq!(actual, expected, "{}", msg);

        let expected = vec!['a', 'b', '\n'];
        let actual = read_all_chars("ab".into());
        assert_eq!(actual, expected, "{}", msg);

        let expected = vec!['a', '\n', 'b', '\n'];
        let actual = read_all_chars("a\nb".into());
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn prints_err() {
        let msg = "should print pleasant error output";

        let ex_err: ParseResult<Vec<char>> = Err(ParseErr(
            "identifier".to_string(),
            "unexpected |".to_string(),
            ParserPosition {
                current_line: "123 ab|cd",
                line: 1,
                column: 6,
            },
        ));

        let expected = r#"Line:1 Col:6 Error parsing identifier
123 ab|cd
      ^unexpected |"#;
        let actual = print_result(&ex_err);
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn parse_char() {
        let msg = "should parse a single char";

        let parse_ab = p_char('A');

        let expected = 'A';
        let (_, actual) = parse_ab.parse("A|C").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = r#"Line:0 Col:0 Error parsing A
B|C
^Unexpected 'B'"#;
        let actual = parse_ab.parse("B|C").unwrap_err();
        assert_eq!(format!("{}", actual), expected, "{}", msg);
    }

    #[test]
    fn and_thens() {
        let msg = "should parse 'A' and then 'B'";

        let parse_ab = p_char('A').and_then(p_char('B'));

        let expected = r#"Line:0 Col:1 Error parsing B
A|C
 ^Unexpected '|'"#;
        let actual = parse_ab.parse("A|C").unwrap_err();
        assert_eq!(format!("{}", actual), expected, "{}", msg);
    }

    #[test]
    fn or_else_t() {
        let msg = "should parse 'A' or else 'B'";

        let parse_ab = p_char('A').or_else(p_char('B'));

        let expected = 'B';
        let (_, actual) = parse_ab.parse("B").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = r#"Line:0 Col:0 Error parsing B
C|C
^Unexpected 'C'"#;
        let actual = parse_ab.parse("C|C").unwrap_err();
        assert_eq!(format!("{}", actual), expected, "{}", msg);
    }

    #[test]
    fn many_matches() {
        let msg = "should parse zero or more 'A' chars";

        let zero_plus_a = many(p_char('A'));

        let expected = vec!['A'];
        let (_, actual) = zero_plus_a.parse("ABCD").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = vec!['A', 'A'];
        let (_, actual) = zero_plus_a.parse("AACD").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = vec!['A', 'A', 'A'];
        let (_, actual) = zero_plus_a.parse("AAAD").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        // test a case with no matches
        let expected = vec![];
        let (_, actual) = zero_plus_a.parse("|BCD").unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn one_plus() {
        let msg = "should parse one or more 'A' chars";

        let one_plus_a = one_or_more(p_char('A'));

        let expected = vec!['A'];
        let (_, actual) = one_plus_a.parse("ABCD").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = vec!['A', 'A'];
        let (_, actual) = one_plus_a.parse("AACD").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = vec!['A', 'A', 'A'];
        let (_, actual) = one_plus_a.parse("AAAD").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        // failure case
        let expected = r#"Line:0 Col:0 Error parsing one or more A
BCD
^Unexpected 'B'"#
            .to_string();
        let actual = one_plus_a.parse("BCD");
        assert_eq!(print_result(&actual), expected, "{}", msg);
    }

    #[test]
    fn labeling() {
        let msg = "should apply a new label to a parser";

        let parse_a = p_char('A').with_label("Foo".to_string());
        println!("{}", parse_a.label);

        let expected = r#"Line:0 Col:0 Error parsing Foo
B|C
^Unexpected 'B'"#;
        let actual = parse_a.parse("B|C").unwrap_err();
        assert_eq!(format!("{}", actual), expected, "{}", msg);
    }

    #[test]
    fn sequences() {
        let msg = "should sequence a list of parsers";

        let parsers = vec![p_char('A'), p_char('B'), p_char('C')];
        let combined = sequence(&parsers);

        let expected = vec!['A', 'B', 'C'];
        let (_, actual) = combined.parse("ABCD").unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn parse_string() {
        let msg = "should parse a string";
        let expected = "\"AB\"";
        let actual = p_string("AB").parse("ABC");
        assert_eq!(print_result(&actual), expected, "{}", msg);

        let expected = r#"Line:0 Col:1 Error parsing AB
A|C
 ^Unexpected '|'"#;
        let actual = p_string("AB").parse("A|C");
        assert_eq!(print_result(&actual), expected, "{}", msg);
    }

    #[test]
    fn many_spaces() {
        let msg = "should parse zero or more whitespace chars";

        let spaces = spaces();

        let expected = vec![' '];
        let (_, actual) = spaces.parse(" ABC").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = vec![];
        let (_, actual) = spaces.parse("A").unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn one_plus_spaces() {
        let msg = "should parse one or more whitespace chars";

        let spaces = one_or_more_spaces();

        let expected = vec![' '];
        let (_, actual) = spaces.parse(" ABC").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = r#"Line:0 Col:0 Error parsing one or more whitespace
A
^Unexpected 'A'"#
            .to_string();
        let actual = spaces.parse("A");
        assert_eq!(print_result(&actual), expected, "{}", msg);
    }

    #[test]
    fn parse_integer() {
        let msg = "should parse an integer";

        let parse_int = p_int(10);

        let expected = 123;
        let (_, actual) = parse_int.parse("123C").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = -123;
        let (_, actual) = parse_int.parse("-123C").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = r#"Line:0 Col:1 Error parsing integer
-Z123
 ^Unexpected 'Z'"#
            .to_string();
        let actual = parse_int.parse("-Z123");
        assert_eq!(print_result(&actual), expected, "{}", msg);
    }

    #[test]
    fn parse_float() {
        let msg = "should parse a float";

        let parse_float = p_float(10);

        let expected = 123.45;
        let (_, actual) = parse_float.parse("123.45C").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = -123.45;
        let (_, actual) = parse_float.parse("-123.45C").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = r#"Line:0 Col:4 Error parsing float
-123Z45
    ^Unexpected 'Z'"#
            .to_string();
        let actual = parse_float.parse("-123Z45");
        assert_eq!(print_result(&actual), expected, "{}", msg);
    }

    #[test]
    fn ap() {
        let msg = "should apply values in context";

        let fx = Parser::from(Rc::new(|a: i32| Rc::new(move |b| a + b)));
        let a = Parser::from(22);
        let b = Parser::from(20);

        let expected = 42;
        let actual = apply(fx, a);
        let (_, actual) = apply(actual, b).parse("").unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn keeps_first() {
        let msg = "should keep the results of the first parser";

        let digit = p_int(10);
        let digit_then_semicolon = keep_first(digit, optional(p_char(';')));

        let expected = 1;
        let (_, actual) = digit_then_semicolon.parse("1;").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let (_, actual) = digit_then_semicolon.parse("1").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let whitespace_char = any_of([' ', '\t', '\n']);
        let whitespace = one_or_more(whitespace_char);

        let ab = p_string("AB");
        let cd = p_string("CD");
        let ab_cd = and_then(keep_first(ab, whitespace), cd);

        let expected = ("AB".to_string(), "CD".to_string());
        let (_, actual) = ab_cd.parse("AB \t\nCD").unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn inbetween() {
        let msg = "should keep the results of the middle parser";

        let double_quote = p_char('"');
        let quoted_integer = between(double_quote.clone(), p_int(10), double_quote);

        let expected = 1234;
        let (_, actual) = quoted_integer.parse("\"1234\"").unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn separators() {
        let msg = "should parse separated values";

        let digit = p_int(10);
        let comma = p_char(',');
        let zero_or_more_digit_list = sep_by(digit.clone(), comma.clone());
        let one_or_more_digit_list = sep_by_one(digit, comma);

        let expected = vec![1];
        let (_, actual) = one_or_more_digit_list.parse("1;").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = vec![1, 2, 3];
        let (_, actual) = one_or_more_digit_list.parse("1,2,3;").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = vec![1, 2, 3];
        let (_, actual) = zero_or_more_digit_list.parse("1,2,3;").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = vec![];
        let (_, actual) = zero_or_more_digit_list.parse("Z;").unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn binding() {
        let msg = "should derive map via bind";

        fn map<'a>(f: impl Fn(char) -> char + 'a) -> Parser<'a, char> {
            let p = p_char('A');
            bind(p.clone(), move |x| Parser::of(f(x)))
        }

        let lower_a = map(|ch: char| ch.to_lowercase().next().unwrap());

        let expected = 'a';
        let (_, actual) = lower_a.parse("A").unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }
}
