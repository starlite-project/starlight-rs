#![allow(dead_code)]
use std::{borrow::Cow, error::Error as StdError, fmt, marker::PhantomData, str::FromStr};
use uwl::Stream;

#[derive(Debug)]
#[non_exhaustive]
pub enum ArgError<E> {
    Eos,
    Parse(E),
}

impl<E> From<E> for ArgError<E> {
    fn from(e: E) -> Self {
        Self::Parse(e)
    }
}

impl<E: fmt::Display> fmt::Display for ArgError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ArgError::*;

        match *self {
            Eos => write!(f, "ArgError(\"end of string\")"),
            Parse(ref e) => write!(f, "ArgError(\"{}\")", e),
        }
    }
}

impl<E: fmt::Debug + fmt::Display> StdError for ArgError<E> {}

type ArgResult<T, E> = std::result::Result<T, ArgError<E>>;

#[derive(Debug, Clone)]
pub enum Delimiter {
    Single(char),
    Multiple(String),
}

impl Delimiter {
    #[inline]
    fn to_str(&self) -> Cow<'_, str> {
        match self {
            Self::Single(c) => Cow::Owned(c.to_string()),
            Self::Multiple(s) => Cow::Borrowed(s),
        }
    }
}

impl From<char> for Delimiter {
    #[inline]
    fn from(c: char) -> Self {
        Self::Single(c)
    }
}

impl From<String> for Delimiter {
    #[inline]
    fn from(s: String) -> Self {
        Self::Multiple(s)
    }
}

impl<'a> From<&'a String> for Delimiter {
    #[inline]
    fn from(s: &'a String) -> Self {
        Self::Multiple(s.clone())
    }
}

impl<'a> From<&'a str> for Delimiter {
    #[inline]
    fn from(s: &'a str) -> Self {
        Self::Multiple(s.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenKind {
    Argument,
    QuotedArgument,
}

#[derive(Debug, Clone, Copy)]
struct Token {
    kind: TokenKind,
    span: (usize, usize),
}

impl Token {
    #[inline]
    fn new(kind: TokenKind, start: usize, end: usize) -> Self {
        Self {
            kind,
            span: (start, end),
        }
    }
}

fn lex(stream: &mut Stream<'_>, delims: &[Cow<'_, str>]) -> Option<Token> {
    if stream.is_empty() {
        return None;
    }

    let start = stream.offset();
    if stream.current()? == b'"' {
        stream.next();

        stream.take_until(|b| b == b'"');

        let is_quote = stream.current().map_or(false, |b| b == b'"');
        stream.next();

        let end = stream.offset();

        for delim in delims {
            stream.eat(delim);
        }

        return Some(if is_quote {
            Token::new(TokenKind::QuotedArgument, start, end)
        } else {
            Token::new(TokenKind::Argument, start, stream.len())
        });
    }

    let mut end = start;

    'outer: while !stream.is_empty() {
        for delim in delims {
            end = stream.offset();

            if stream.eat(&delim) {
                break 'outer;
            }
        }

        stream.next_char();
        end = stream.offset();
    }

    Some(Token::new(TokenKind::Argument, start, end))
}

fn remove_quotes(s: &str) -> &str {
    if s.starts_with('"') && s.ends_with('"') {
        return &s[1..s.len() - 1];
    }

    s
}

#[derive(Debug, Clone, Copy)]
enum State {
    None,
    Quoted,
    Trimmed,
    QuotedTrimmed,
    TrimmedQuoted,
}

#[derive(Debug, Clone)]
pub struct Args {
    message: String,
    args: Vec<Token>,
    offset: usize,
    state: State,
}

impl Args {
    pub fn new(message: &str, possible_delimiters: &[Delimiter]) -> Self {
        let delims = possible_delimiters
            .iter()
            .filter(|d| match d {
                Delimiter::Single(c) => message.contains(*c),
                Delimiter::Multiple(s) => message.contains(s),
            })
            .map(|delim| delim.to_str())
            .collect::<Vec<_>>();

        let args = if delims.is_empty() && !message.is_empty() {
            let kind = if message.starts_with('"') && message.ends_with('"') {
                TokenKind::QuotedArgument
            } else {
                TokenKind::Argument
            };

            vec![Token::new(kind, 0, message.len())]
        } else {
            let mut args = Vec::new();
            let mut stream = Stream::new(message);

            while let Some(token) = lex(&mut stream, &delims) {
                if message[token.span.0..token.span.1].is_empty() {
                    continue;
                }

                args.push(token);
            }

            args
        };

        Args {
            args,
            message: message.to_string(),
            offset: 0,
            state: State::None,
        }
    }

    #[inline]
    fn span(&self) -> (usize, usize) {
        self.args[self.offset].span
    }

    #[inline]
    fn slice(&self) -> &str {
        let (start, end) = self.span();

        &self.message[start..end]
    }

    pub fn advance(&mut self) -> &mut Self {
        if self.is_empty() {
            return self;
        }

        self.offset += 1;

        self
    }

    #[inline]
    pub fn rewind(&mut self) -> &mut Self {
        if self.offset == 0 {
            return self;
        }

        self.offset -= 1;

        self
    }

    #[inline]
    pub fn restore(&mut self) {
        self.offset = 0;
    }

    fn apply<'a>(&self, s: &'a str) -> &'a str {
        fn trim(s: &str) -> &str {
            let trimmed = s.trim();

            let start = s.find(trimmed).unwrap_or(0);
            let end = start + trimmed.len();

            &s[start..end]
        }

        let mut s = s;

        match self.state {
            State::None => {}
            State::Quoted => {
                s = remove_quotes(s);
            }
            State::Trimmed => {
                s = trim(s);
            }
            State::QuotedTrimmed => {
                s = remove_quotes(s);
                s = trim(s);
            }
            State::TrimmedQuoted => {
                s = trim(s);
                s = remove_quotes(s);
            }
        }

        s
    }

    #[inline]
    pub fn current(&self) -> Option<&str> {
        if self.is_empty() {
            return None;
        }

        let mut s = self.slice();
        s = self.apply(s);

        Some(s)
    }

    pub fn trimmed(&mut self) -> &mut Self {
        match self.state {
            State::None => self.state = State::Trimmed,
            State::Quoted => self.state = State::QuotedTrimmed,
            _ => {}
        }

        self
    }

    pub fn untrimmed(&mut self) -> &mut Self {
        match self.state {
            State::Trimmed => self.state = State::None,
            State::QuotedTrimmed | State::TrimmedQuoted => self.state = State::Quoted,
            _ => {}
        }

        self
    }

    pub fn quoted(&mut self) -> &mut Self {
        if self.is_empty() {
            return self;
        }

        let is_quoted = self.args[self.offset].kind == TokenKind::QuotedArgument;

        if is_quoted {
            match self.state {
                State::None => self.state = State::Quoted,
                State::Trimmed => self.state = State::TrimmedQuoted,
                _ => {}
            }
        }

        self
    }

    pub fn unquoted(&mut self) -> &mut Self {
        match self.state {
            State::Quoted => self.state = State::None,
            State::QuotedTrimmed | State::TrimmedQuoted => self.state = State::Trimmed,
            _ => {}
        }

        self
    }

    #[inline]
    pub fn parse<T: FromStr>(&self) -> ArgResult<T, T::Err> {
        T::from_str(self.current().ok_or(ArgError::Eos)?).map_err(ArgError::Parse)
    }

    #[inline]
    pub fn single<T: FromStr>(&mut self) -> ArgResult<T, T::Err> {
        let p = self.parse::<T>()?;
        self.advance();
        Ok(p)
    }

    #[inline]
    pub fn single_quoted<T: FromStr>(&mut self) -> ArgResult<T, T::Err> {
        let p = self.quoted().parse::<T>()?;
        self.advance();
        Ok(p)
    }

    #[inline]
    pub fn iter<T: FromStr>(&mut self) -> Iter<'_, T> {
        Iter {
            args: self,
            state: State::None,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn raw(&self) -> RawArguments<'_> {
        RawArguments {
            tokens: &self.args,
            msg: &self.message,
            quoted: false,
        }
    }

    #[inline]
    pub fn raw_quoted(&self) -> RawArguments<'_> {
        let mut raw = self.raw();
        raw.quoted = true;
        raw
    }

    pub fn find<T: FromStr>(&mut self) -> ArgResult<T, T::Err> {
        if self.is_empty() {
            return Err(ArgError::Eos);
        }

        let before = self.offset;
        self.restore();

        let pos = match self.iter::<T>().quoted().position(|res| res.is_ok()) {
            Some(p) => p,
            None => {
                self.offset = before;
                return Err(ArgError::Eos);
            }
        };

        self.offset = pos;
        let parsed = self.single_quoted::<T>()?;

        self.args.remove(pos);
        self.offset = before;
        self.rewind();

        Ok(parsed)
    }

    pub fn find_n<T: FromStr>(&mut self) -> ArgResult<T, T::Err> {
        if self.is_empty() {
            return Err(ArgError::Eos);
        }

        let before = self.offset;
        self.restore();

        let pos = match self.iter::<T>().quoted().position(|res| res.is_ok()) {
            Some(p) => p,
            None => {
                self.offset = before;
                return Err(ArgError::Eos);
            }
        };

        self.offset = pos;
        let parsed = self.quoted().parse::<T>()?;

        self.offset = before;

        Ok(parsed)
    }

    #[inline]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[inline]
    pub fn rest(&self) -> &str {
        self.remains().unwrap_or_default()
    }

    #[inline]
    pub fn remains(&self) -> Option<&str> {
        if self.is_empty() {
            return None;
        }

        let (start, _) = self.span();
        Some(&self.message[start..])
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.args.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.offset >= self.len()
    }

    #[inline]
    pub fn remaining(&self) -> usize {
        if self.is_empty() {
            return 0;
        }

        self.len() - self.offset
    }
}

pub struct Iter<'a, T: FromStr> {
    args: &'a mut Args,
    state: State,
    _marker: PhantomData<T>,
}

impl<'a, T: FromStr> Iter<'a, T> {
    pub fn current(&mut self) -> Option<&str> {
        self.args.state = self.state;
        self.args.current()
    }

    pub fn parse(&mut self) -> ArgResult<T, T::Err> {
        self.args.state = self.state;
        self.args.parse::<T>()
    }

    #[inline]
    pub fn quoted(&mut self) -> &mut Self {
        match self.state {
            State::None => self.state = State::Quoted,
            State::Trimmed => self.state = State::TrimmedQuoted,
            _ => {}
        }

        self
    }

    #[inline]
    pub fn trimmed(&mut self) -> &mut Self {
        match self.state {
            State::None => self.state = State::Trimmed,
            State::Quoted => self.state = State::QuotedTrimmed,
            _ => {}
        }

        self
    }
}

impl<'a, T: FromStr> Iterator for Iter<'a, T> {
    type Item = ArgResult<T, T::Err>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.args.is_empty() {
            None
        } else {
            let arg = self.parse();
            self.args.advance();
            Some(arg)
        }
    }
}

#[derive(Debug)]
pub struct RawArguments<'a> {
    msg: &'a str,
    tokens: &'a [Token],
    quoted: bool,
}

impl<'a> Iterator for RawArguments<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (start, end) = self.tokens.get(0)?.span;

        self.tokens = &self.tokens[1..];

        let mut s = &self.msg[start..end];

        if self.quoted {
            s = remove_quotes(s)
        }

        Some(s)
    }
}

#[cfg(test)]
mod tests {
    use super::Args;

    #[test]
    fn single_with_one_delimiter() {
        let mut args = Args::new("1,2", &[','.into()]);
        assert_eq!(args.single::<String>().unwrap(), "1".to_owned());
        assert_eq!(args.single::<String>().unwrap(), "2".to_owned());
    }

    #[test]
    fn single_with_three_delimiters() {
        let mut args = Args::new("1,2 @3@4 5", &[','.into(), ' '.into(), '@'.into()]);

        assert_eq!(args.single::<String>().unwrap(), "1");
        assert_eq!(args.single::<String>().unwrap(), "2");
        assert_eq!(args.single::<String>().unwrap(), "3");
        assert_eq!(args.single::<String>().unwrap(), "4");
        assert_eq!(args.single::<String>().unwrap(), "5");
    }

    #[test]
    fn single_quoted_with_one_delimiter() {
        let mut args = Args::new(r#""1","2""#, &[','.into()]);

        assert_eq!(args.single_quoted::<String>().unwrap(), "1".to_owned());
    }

    #[test]
    fn iter_with_one_delimiter() {
        let mut args = Args::new("1,2,3,4,5,6,7,8,9,10", &[','.into()]);

        assert_eq!(
            args.iter::<String>()
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
            vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]
                .iter()
                .map(|ch| ch.to_string())
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn iter_with_three_delimiters() {
        let mut args = Args::new(
            "1-2<3,4,5,6,7<8,9,10",
            &[','.into(), '-'.into(), '<'.into()],
        );

        assert_eq!(
            args.iter::<String>()
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
            vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]
                .iter()
                .map(|ch| ch.to_string())
                .collect::<Vec<String>>()
        );
    }

    
}
