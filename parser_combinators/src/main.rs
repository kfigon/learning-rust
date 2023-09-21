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
}