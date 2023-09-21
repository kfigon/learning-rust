fn main() {
    println!("Hello, world!");
}


#[derive(Debug, PartialEq)]
enum ParsingErr {
    NotFound,
    Error(String)
}

#[derive(Debug, PartialEq)]
struct ParserResult{
    res: String
}
impl ParserResult {
    fn new(res: String) -> Self {
        Self { res }
    }
}

trait Parser {
    fn parse(&self, s: &str) -> Result<ParserResult, ParsingErr>;
}

struct StringParser<'a>{
    pattern: &'a str
}

impl<'a> StringParser<'a> {
    fn new(s: &'a str) -> Self {
        Self { pattern: s }
    }
}

impl<'a> Parser for StringParser<'a> {
    fn parse(&self, s: &str) -> Result<ParserResult, ParsingErr> {
        if s.starts_with(self.pattern) {
            Ok(ParserResult::new(self.pattern.to_string()))
        } else {
            Err(ParsingErr::NotFound)
        }
    }
}

struct DigitParser;
impl Parser for DigitParser {
    fn parse(&self, s: &str) -> Result<ParserResult, ParsingErr> {
        match s.chars().next() {
            Some(v) if v.is_digit(10) => Ok(ParserResult::new(v.to_string())),
            _ => Err(ParsingErr::NotFound),
        }
    }
}

struct Maybe<P: Parser> {
    p: P
}
impl<P: Parser> Maybe<P> {
    fn new(p: P) -> Self {
        Self { p }
    }
}

impl<P: Parser> Parser for Maybe<P>{
 fn parse(&self, s: &str) -> Result<ParserResult, ParsingErr> {
        match self.p.parse(s) {
            Ok(v) => Ok(v),
            Err(_) => Ok(ParserResult::new("".to_string())),
        }
    }
}

struct Many {
    parsers: Vec<Box<dyn Parser>>
}

impl Many {
    fn new(parsers: Vec<Box<dyn Parser>>) -> Self {
        Self { parsers }
    }
}

impl Parser for Many {
    fn parse(&self, s: &str) -> Result<ParserResult, ParsingErr> {
        let mut to_search = s;
        let mut result = String::new();

        for p in &self.parsers {
            let res = p.parse(to_search)?;
            result.push_str(&res.res);

            to_search = &to_search[res.res.len()..]
        }

        Ok(ParserResult::new(result))
    }
}

struct FloatParser;
impl Parser for FloatParser {
    fn parse(&self, s: &str) -> Result<ParserResult, ParsingErr> {
        let p = Many::new(vec![
                Box::new(While::new(DigitParser)),
                Box::new(
                    Many::new(vec![
                        Box::new(StringParser::new(".")),
                        Box::new(While::new(DigitParser)),
                    ])
                )
            ]);
        
        p.parse(s)
    }
}

struct Until<P: Parser> {
    p: P
}
impl<P: Parser> Until<P> {
    fn new(p: P) -> Self {
        Self { p }
    }
}

impl<P: Parser> Parser for Until<P> {
    fn parse(&self, s: &str) -> Result<ParserResult, ParsingErr> {
        let mut out = String::new();
        let mut left = s;
        for c in s.chars() {
            match self.p.parse(left) {
                Err(ParsingErr::NotFound) => {
                    left = &left[1..];
                    out.push(c);
                },
                Err(e) => return Err(e),
                Ok(_) => break,
            }
        }

        if out.is_empty() {
            Err(ParsingErr::NotFound)
        } else {
            Ok(ParserResult::new(out))
        }
    }
}

struct While<P: Parser> {
    p: P
}
impl<P: Parser> While<P> {
    fn new(p: P) -> Self {
        Self { p }
    }
}

impl<P: Parser> Parser for While<P> {
    fn parse(&self, s: &str) -> Result<ParserResult, ParsingErr> {
        let mut out = String::new();
        let mut left = s;
        while !left.is_empty() {
            match self.p.parse(left) {
                Err(ParsingErr::NotFound) => break,
                Err(e) => return Err(e),
                Ok(v) => {
                    out.push_str(&v.res);
                    left = &left[v.res.len()..];
                },
            }
        }
        if out.is_empty() {
            Err(ParsingErr::NotFound)
        } else {
            Ok(ParserResult::new(out))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn str_parser_invalid() {
        let s = StringParser::new("hello");
        assert_eq!(s.parse("foo"), Err(ParsingErr::NotFound));
        assert_eq!(s.parse("hell"), Err(ParsingErr::NotFound));
        assert_eq!(s.parse(""), Err(ParsingErr::NotFound));
    }

    #[test]
    fn str_parser() {
        let s = StringParser::new("hello");
        assert_eq!(s.parse("hello w"), Ok(ParserResult::new("hello".to_string())));
        assert_eq!(s.parse("hello"), Ok(ParserResult::new("hello".to_string())))
    }

    #[test]
    fn digit_parser_invalid() {
        let d = DigitParser;
        assert_eq!(d.parse("foo"), Err(ParsingErr::NotFound));
        assert_eq!(d.parse("hell"), Err(ParsingErr::NotFound));
        assert_eq!(d.parse(""), Err(ParsingErr::NotFound));
    }

    #[test]
    fn digit_parser() {
        let d = DigitParser;
        assert_eq!(d.parse("5"),   Ok(ParserResult::new("5".to_string())));
        assert_eq!(d.parse("123"), Ok(ParserResult::new("1".to_string())));
    }

    #[test]
    fn float_parser() {
        let p = FloatParser;
        assert_eq!(p.parse("123.4"), Ok(ParserResult::new("123.4".to_string())));
        assert_eq!(p.parse("1.45a"), Ok(ParserResult::new("1.45".to_string())));
        assert_eq!(p.parse("5"),   Err(ParsingErr::NotFound));
        assert_eq!(p.parse("123"), Err(ParsingErr::NotFound));
        assert_eq!(p.parse(".45"), Err(ParsingErr::NotFound));
        assert_eq!(p.parse("1234."), Err(ParsingErr::NotFound));
        assert_eq!(p.parse("1234.a"), Err(ParsingErr::NotFound));
    }

    #[test]
    fn many_parser_digits() {
        let d = Many::new(vec![Box::new(DigitParser), Box::new(DigitParser), Box::new(DigitParser)]);
        assert_eq!(d.parse("123"),   Ok(ParserResult::new("123".to_string())));
        assert_eq!(d.parse("1234"), Ok(ParserResult::new("123".to_string())));
    }

    #[test]
    fn many_parser_combined() {
        let d = Many::new(vec![Box::new(DigitParser), Box::new(StringParser::new(" foobar ")), Box::new(DigitParser)]);
        assert_eq!(d.parse("1 foobar 3"),   Ok(ParserResult::new("1 foobar 3".to_string())));
        assert_eq!(d.parse("1 foobar 3456798 asdf"), Ok(ParserResult::new("1 foobar 3".to_string())));
    }

    #[test]
    fn many_parser_combined_failed() {
        let d = Many::new(vec![Box::new(DigitParser), Box::new(StringParser::new(" foobar ")), Box::new(DigitParser)]);
        assert_eq!(d.parse("1 foobar x"),  Err(ParsingErr::NotFound));
        assert_eq!(d.parse("1 foobar x5"), Err(ParsingErr::NotFound));
    }

    #[test]
    fn maybe_digit_parser() {
        let d = Maybe::new(DigitParser);
        assert_eq!(d.parse("1"),  Ok(ParserResult::new("1".to_string())));
        assert_eq!(d.parse(""),  Ok(ParserResult::new("".to_string())));
        assert_eq!(d.parse("abc"),  Ok(ParserResult::new("".to_string())));
    }

    #[test]
    fn maybe_str_parser() {
        let d = Maybe::new(StringParser::new("foobar"));
        assert_eq!(d.parse("1"),  Ok(ParserResult::new("".to_string())));
        assert_eq!(d.parse(""),  Ok(ParserResult::new("".to_string())));
        assert_eq!(d.parse("foo"),  Ok(ParserResult::new("".to_string())));
        assert_eq!(d.parse("foobarz"),  Ok(ParserResult::new("foobar".to_string())));
    }

    #[test]
    fn until_char() {
        let p = Until::new(StringParser::new("*"));
        assert_eq!(p.parse("1234*"),  Ok(ParserResult::new("1234".to_string())));
        assert_eq!(p.parse("12345asbdc*"),  Ok(ParserResult::new("12345asbdc".to_string())));
        assert_eq!(p.parse("*foo"),  Err(ParsingErr::NotFound));
        assert_eq!(p.parse("foobarz123  xxx*"),  Ok(ParserResult::new("foobarz123  xxx".to_string())));
        assert_eq!(p.parse("foobarz123  "),  Ok(ParserResult::new("foobarz123  ".to_string())));
    }

    #[test]
    fn until_str() {
        let p = Until::new(StringParser::new("foo"));
        assert_eq!(p.parse("1234*"),  Ok(ParserResult::new("1234*".to_string())));
        assert_eq!(p.parse("12345asbdc*"),  Ok(ParserResult::new("12345asbdc*".to_string())));
        assert_eq!(p.parse("foobar"),  Err(ParsingErr::NotFound));
        assert_eq!(p.parse("*foo"),  Ok(ParserResult::new("*".to_string())));
        assert_eq!(p.parse("123faa*foo"),  Ok(ParserResult::new("123faa*".to_string())));
    }

    #[test]
    fn until_digit() {
        let p = Until::new(DigitParser);
        assert_eq!(p.parse("1234*"),  Err(ParsingErr::NotFound));
        assert_eq!(p.parse("12345asbdc*"),  Err(ParsingErr::NotFound));
        assert_eq!(p.parse("*foo"),  Ok(ParserResult::new("*foo".to_string())));
        assert_eq!(p.parse("foobarz123  xxx*"),  Ok(ParserResult::new("foobarz".to_string())));
    }

    #[test]
    fn while_digit() {
        let p = While::new(DigitParser);
        assert_eq!(p.parse("1234*"),  Ok(ParserResult::new("1234".to_string())));
        assert_eq!(p.parse("12345a123"),  Ok(ParserResult::new("12345".to_string())));
        assert_eq!(p.parse("*foo"),  Err(ParsingErr::NotFound));
        assert_eq!(p.parse("foobarz123  xxx*"),  Err(ParsingErr::NotFound));
    }

    #[test]
    fn while_str() {
        let p = While::new(StringParser::new("hi"));
        assert_eq!(p.parse("hihihi4*"),  Ok(ParserResult::new("hihihi".to_string())));
        assert_eq!(p.parse("hihihi"),  Ok(ParserResult::new("hihihi".to_string())));
        assert_eq!(p.parse("hihih"),  Ok(ParserResult::new("hihi".to_string())));
        assert_eq!(p.parse("hjhih"),  Err(ParsingErr::NotFound));
    }

    #[test]
    fn while_many() {
        let p = While::new(Many::new(vec![Box::new(StringParser::new("hi")), Box::new(StringParser::new("!")), Box::new(DigitParser)]));
        assert_eq!(p.parse("hi!1"),  Ok(ParserResult::new("hi!1".to_string())));
        assert_eq!(p.parse("hi!1hi!2hi!3ads"),  Ok(ParserResult::new("hi!1hi!2hi!3".to_string())));
        assert_eq!(p.parse("hi!"),  Err(ParsingErr::NotFound));
    }
}