use super::parse_variable_name;
use winnow::ascii::alpha1;
use winnow::combinator::delimited;
use winnow::combinator::peek;
use winnow::combinator::preceded;
use winnow::error::IResult;
use winnow::stream::AsChar;
use winnow::token::take_while;
use winnow::PResult;
use winnow::Parser;

pub fn parse_simple_variable<'s>(input: &mut &'s str) -> PResult<&'s str> {
    preceded('$', parse_variable_name).parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_variable1() {
        let mut input = "$foo";
        assert_eq!(parse_simple_variable(&mut input), Ok("foo"));
        assert_eq!(input, "");
    }

    #[test]
    fn test_parse_simple_variable2() {
        let mut input = "$foo/";
        assert_eq!(parse_simple_variable(&mut input), Ok("foo"));
        assert_eq!(input, "/");
    }
}
