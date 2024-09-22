use crate::environment::Environment;
use crate::varpath::parse_word;
use crate::varpath::parse_word_str;
use crate::varpath::PathPart;
use eyre::Result;
use std::env;
use winnow::ascii::alpha1;
use winnow::combinator::alt;
use winnow::combinator::delimited;
use winnow::combinator::peek;
use winnow::error::IResult;
use winnow::stream::AsChar;
use winnow::token::take_while;
use winnow::PResult;
use winnow::Parser;
mod simple;
use simple::parse_simple_variable;
mod complex;
use complex::parse_complex_variable;
use eyre::bail;

#[derive(Debug, Eq, PartialEq)]
pub struct Variable(String);
impl Variable {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }

    pub fn eval(&self, env: &Environment) -> Result<String> {
        let key = &self.0;
        let Some(value) = env.get(key) else {
            bail!("Key '{}' not found in environment", key);
        };
        Ok(value.clone())
    }
}

fn parse_variable_name<'s>(input: &mut &'s str) -> PResult<&'s str> {
    // First char must be alphabetic
    peek(alpha1).parse_next(input)?;
    parse_word_str.parse_next(input)
}

pub fn parse_variable<'s>(input: &mut &'s str) -> PResult<Variable> {
    alt((parse_simple_variable, parse_complex_variable))
        .parse_next(input)
        .map(|s| Variable(s.to_string()))
}

#[cfg(test)]
mod tests {
    use crate::environment::EnvironmentBuilder;
    use pretty_assertions::assert_eq;
    use pretty_assertions::assert_ne;

    use super::*;
    use homedir::my_home;

    #[test]
    fn test_parse_variable_name1() {
        let mut input = "foo}";
        assert_eq!(parse_variable_name(&mut input), Ok("foo"));
        assert_eq!(input, "}");
    }

    #[test]
    fn test_parse_variable_name2() {
        let mut input = "1foo";
        assert_ne!(parse_variable_name(&mut input), Ok("foo"));
    }

    #[test]
    fn test_parse_variable1() {
        let mut input = "$foo";
        assert_eq!(parse_variable(&mut input), Ok(Variable::new("foo")));
        assert_eq!(input, "");
    }

    #[test]
    fn test_parse_variable2() {
        let mut input = "${foo}";
        assert_eq!(parse_variable(&mut input), Ok(Variable::new("foo")));
        assert_eq!(input, "");
    }

    #[test]
    fn test_eval() {
        let key = "TEST";
        let value = "test";
        let mut env = Environment::new();
        env.insert(key.to_string(), value.to_string());
        let var = Variable::new(key);
        assert_eq!(var.eval(&env).unwrap(), value);
    }

    #[test]
    fn test_home() {
        let var = Variable::new("HOME");
        let mut env = EnvironmentBuilder::default().with_process_env().build();
        let expected = my_home().unwrap().unwrap().display().to_string();
        let actual = var.eval(&env).unwrap();
        assert_eq!(expected, actual);
    }
}
