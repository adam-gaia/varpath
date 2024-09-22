use super::parse_variable_name;
use winnow::ascii::alpha1;
use winnow::combinator::delimited;
use winnow::combinator::peek;
use winnow::error::IResult;
use winnow::stream::AsChar;
use winnow::token::take_while;
use winnow::PResult;
use winnow::Parser;

// TODO: we could do fun stuff like shell parameter expansion

fn parse_open_tag<'s>(input: &mut &'s str) -> PResult<&'s str> {
    "${".parse_next(input)
}

fn parse_close_tag<'s>(input: &mut &'s str) -> PResult<&'s str> {
    "}".parse_next(input)
}

pub fn parse_complex_variable<'s>(input: &mut &'s str) -> PResult<&'s str> {
    delimited(parse_open_tag, parse_variable_name, parse_close_tag).parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_open_tag() {
        let mut input = "${";
        assert_eq!(parse_open_tag(&mut input), Ok("${"));
        assert_eq!(input, "");
    }

    #[test]
    fn test_parse_close_tag() {
        let mut input = "}";
        assert_eq!(parse_close_tag(&mut input), Ok("}"));
        assert_eq!(input, "");
    }

    #[test]
    fn test_parse_complex_variable() {
        let mut input = "${foo}";
        assert_eq!(parse_complex_variable(&mut input), Ok("foo"));
        assert_eq!(input, "");
    }
}
