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

    /// apply a function to the value inside a parser
    pub fn map<U>(self, f: impl Fn(O) -> U + 'a) -> Parser<'a, I, U> {
        let Self { parse, label } = self;
        Parser {
            label,
            parse: Rc::new(move |input: &[I]| {
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
            parse: Rc::new(move |input: &[I]| {
                (parse)(input).map_err(|(_, err)| (label.clone(), err))
            }),
        }
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

/// Choose any of a list of parsers
pub fn choice<'a, I: 'a, O: 'a>(
    parsers: impl IntoIterator<Item = Parser<'a, I, O>>,
) -> Parser<'a, I, O> {
    parsers.into_iter().reduce(or_else).unwrap()
}

// 3-2. Replacing “pchar” with “satisfy”

/// Match an input token if the predicate is satisfied
pub fn satisfy<'a, I>(predicate: impl Fn(&I) -> bool + 'a, label: String) -> Parser<'a, I, I>
where
    I: std::fmt::Debug + Copy,
{
    Parser {
        label: label.clone(),
        parse: Rc::new(move |input: &[I]| {
            let first = input
                .iter()
                .next()
                .ok_or_else(|| (label.clone(), "No more input".to_string()))?;
            if !predicate(first) {
                return Err((label.clone(), format!("Unexpected {:?}", first)));
            }

            Ok((&input[1..], *first))
        }),
    }
}

/// Parse a single character
pub fn p_char<'a>(char_to_match: char) -> Parser<'a, char, char> {
    let predicate = move |ch: &char| *ch == char_to_match;
    let label = format!("{}", char_to_match);
    satisfy(predicate, label)
}

/// Choose any of a list of characters
pub fn any_of<'a, I>(char_list: I) -> Parser<'a, char, char>
where
    I: IntoIterator<Item = char> + std::fmt::Debug,
{
    let label = format!("any of {:?}", char_list);
    let parsers = char_list.into_iter().map(p_char).collect::<Vec<_>>();
    let mut p = choice(parsers);
    p.label = label;

    p
}

/// parse a single digit
pub fn parse_digit<'a>(base: u32) -> Parser<'a, char, char> {
    // any_of('0'..='9')
    let predicate = move |ch: &char| ch.is_digit(base);
    let label = "digit".to_string();
    satisfy(predicate, label)
}

/// parse a single whitespace character
pub fn whitespace_char<'a>() -> Parser<'a, char, char> {
    let predicate = |ch: &char| ch.is_whitespace();
    let label = "whitespace".to_string();
    satisfy(predicate, label)
}

fn of<'a, I, O: 'a>(value: O) -> Parser<'a, I, O>
where
    O: Clone,
{
    Parser {
        label: "unlabeled".to_string(),
        parse: Rc::new(move |input: &[I]| {
            // ignore the input and return value
            Ok((input, value.clone()))
        }),
    }
}

// more idiomatic than `of` in Rust
impl<'a, I, O: 'a> From<O> for Parser<'a, I, O>
where
    O: Clone,
{
    fn from(value: O) -> Self {
        of(value)
    }
}

pub mod three {
    //! [3-3. Adding position and context to error messages](https://fsharpforfunandprofit.com/posts/understanding-parser-combinators-3/#3-adding-position-and-context-to-error-messages)
    use std::fmt;
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
    struct InputState<'a> {
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
    fn next_char<'a>(input: InputState<'a>) -> (InputState<'a>, Option<char>) {
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
    fn print_result<O>(result: ParseResult<O>) -> String
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
            O: Clone + fmt::Debug,
        {
            Parser {
                label: format!("{:?}", value),
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
    }

    // more idiomatic than `of` in Rust
    impl<'a, O: 'a> From<O> for Parser<'a, O>
    where
        O: Clone + fmt::Debug,
    {
        fn from(value: O) -> Self {
            Parser::of(value)
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
            parse: Rc::new(move |input: InputState| {
                zero_or_more(parser.clone()).parse_input(input)
            }),
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

    // 3-4. Adding some standard parsers to the library

    /// Parse a single character
    pub fn p_char<'a>(char_to_match: char) -> Parser<'a, char> {
        let predicate = move |ch: char| ch == char_to_match;
        let label = format!("{}", char_to_match);
        satisfy(predicate, label)
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
            let actual = print_result(ex_err);
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
            assert_eq!(print_result(actual), expected, "{}", msg);
        }
    }
}
