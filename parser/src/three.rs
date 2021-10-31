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
    }
}
