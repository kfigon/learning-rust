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
}