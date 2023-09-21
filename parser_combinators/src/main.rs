fn main() {
    println!("Hello, world!");
}


#[derive(Debug, PartialEq)]
enum ParsingErr {
    NotFound,
    Error(String)
}

#[derive(Debug, PartialEq)]
struct ParserResult;

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
            Ok(ParserResult)
        } else {
            Err(ParsingErr::NotFound)
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
    }

    #[test]
    fn str_parser() {
        let s = StringParser::new("hello");
        assert_eq!(s.parse("hello w"), Ok(ParserResult));
        assert_eq!(s.parse("hello"), Ok(ParserResult))
    }
}