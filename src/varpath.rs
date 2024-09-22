use crate::environment::Environment;
use crate::variable::parse_variable;
use crate::variable::Variable;
use eyre::Result;
use serde::de::{self, Deserializer, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;
use thiserror::Error;
use winnow::ascii::alpha1;
use winnow::combinator::alt;
use winnow::combinator::delimited;
use winnow::combinator::opt;
use winnow::combinator::peek;
use winnow::combinator::repeat;
use winnow::error::IResult;
use winnow::stream::AsChar;
use winnow::token::take_while;
use winnow::PResult;
use winnow::Parser;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VarPath {
    parts: Vec<PathPart>,
    leading_slash: bool,
    trailing_slash: bool,
}

impl<'de> Deserialize<'de> for VarPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VarPathVisitor;

        impl<'de> Visitor<'de> for VarPathVisitor {
            type Value = VarPath;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a VarPath")
            }

            fn visit_str<E>(self, value: &str) -> Result<VarPath, E>
            where
                E: de::Error,
            {
                VarPath::from_str(value).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(VarPathVisitor)
    }
}

impl Serialize for VarPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl VarPath {
    fn new(parts: Vec<PathPart>, leading_slash: bool, trailing_slash: bool) -> Self {
        Self {
            parts,
            leading_slash,
            trailing_slash,
        }
    }

    fn parts(&self) -> &[PathPart] {
        &self.parts
    }

    fn leading_slash(&self) -> bool {
        self.leading_slash
    }

    fn trailing_slash(&self) -> bool {
        self.trailing_slash
    }

    pub fn eval(&self, env: &Environment) -> Result<PathBuf> {
        let mut path = if self.leading_slash {
            PathBuf::from("/")
        } else {
            PathBuf::new()
        };
        for part in self.parts() {
            match part {
                PathPart::Word(word) => {
                    path.push(word.value());
                }
                PathPart::Variable(var) => {
                    let value = var.eval(&env)?;
                    path.push(value);
                }
            }
        }
        Ok(path)
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum VarPathError {
    #[error("Unable to parse varpath")]
    ParseError,
}

impl FromStr for VarPath {
    type Err = VarPathError; // TODO: maybe have our own internal error type
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_varpath.parse(s).map_err(|_| VarPathError::ParseError)
    }
}

impl Display for VarPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!();
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Word(String);
impl Word {
    pub fn new(word: &str) -> Self {
        Self(word.to_string())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

fn word_special_chars<'s>(c: char) -> bool {
    match c {
        '_' | '-' => true,
        _ => false,
    }
}

pub fn parse_word_str<'s>(input: &mut &'s str) -> PResult<&'s str> {
    // TODO: a word can contain escaped characters but variable names (which call this function) should not
    take_while(1.., (AsChar::is_alphanum, word_special_chars)).parse_next(input)
}

pub fn parse_word<'s>(input: &mut &'s str) -> PResult<Word> {
    parse_word_str
        .parse_next(input)
        .map(|s| Word(s.to_string()))
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PathPart {
    Word(Word),
    Variable(Variable),
}

fn parse_word_enum<'s>(input: &mut &'s str) -> PResult<PathPart> {
    parse_word.parse_next(input).map(|s| PathPart::Word(s))
}

fn parse_variable_enum<'s>(input: &mut &'s str) -> PResult<PathPart> {
    parse_variable
        .parse_next(input)
        .map(|s| PathPart::Variable(s))
}

fn parse_path_part<'s>(input: &mut &'s str) -> PResult<PathPart> {
    alt((parse_word_enum, parse_variable_enum)).parse_next(input)
}

// TODO: need to handle '.'s as in '../../' and such

fn parse_path_separator<'s>(input: &mut &'s str) -> PResult<()> {
    '/'.parse_next(input)?;
    Ok(())
}

fn parse_path_part_with_leading_separator<'s>(input: &mut &'s str) -> PResult<PathPart> {
    let _ = parse_path_separator.parse_next(input)?;
    parse_path_part.parse_next(input)
}

fn parse_path_parts<'s>(input: &mut &'s str) -> PResult<(bool, Vec<PathPart>, bool)> {
    // If this is a relative path there is no leading path separator. Grab the first part, then parse the rest as usual
    let first = opt(parse_path_part).parse_next(input)?;

    let mut parts: Vec<PathPart> =
        repeat(0.., parse_path_part_with_leading_separator).parse_next(input)?;

    let leading_slash = if let Some(first) = first {
        parts.insert(0, first);
        false
    } else {
        true
    };

    // Ignore any trailing slash
    let trailing_slash = match opt(parse_path_separator).parse_next(input)? {
        Some(_) => true,
        None => false,
    };

    Ok((leading_slash, parts, trailing_slash))
}

fn parse_varpath<'s>(input: &mut &'s str) -> PResult<VarPath> {
    let (leading_slash, parts, trailing_slash) = parse_path_parts.parse_next(input)?;
    Ok(VarPath::new(parts, leading_slash, trailing_slash))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_parse_word1() {
        let mut input = "abcd";
        assert_eq!(parse_word_str(&mut input), Ok("abcd"));
        assert_eq!(input, "");
    }

    #[test]
    fn test_parse_word2() {
        let mut input = "aaa bbb";
        assert_eq!(parse_word_str(&mut input), Ok("aaa"));
        assert_eq!(input, " bbb");
    }

    #[test]
    fn test_parse_word3() {
        let mut input = "_a-";
        assert_eq!(parse_word_str(&mut input), Ok("_a-"));
        assert_eq!(input, "");
    }

    #[test]
    fn test_parse_path_parts_relative_path() {
        let mut input = "a/b/c";
        assert_eq!(
            parse_path_parts(&mut input),
            Ok((
                false,
                vec![
                    PathPart::Word(Word::new("a")),
                    PathPart::Word(Word::new("b")),
                    PathPart::Word(Word::new("c"))
                ],
                false
            ))
        );
        assert_eq!(input, "");
    }

    #[test]
    fn test_parse_path_parts_trailing_slash() {
        let mut input = "a/b/c/";
        assert_eq!(
            parse_path_parts(&mut input),
            Ok((
                false,
                vec![
                    PathPart::Word(Word::new("a")),
                    PathPart::Word(Word::new("b")),
                    PathPart::Word(Word::new("c"))
                ],
                true
            ))
        );
        assert_eq!(input, "");
    }

    #[test]
    fn test_parse_path_parts_leading_and_trailing_slashes() {
        let mut input = "/a/b/c/";
        assert_eq!(
            parse_path_parts(&mut input),
            Ok((
                true,
                vec![
                    PathPart::Word(Word::new("a")),
                    PathPart::Word(Word::new("b")),
                    PathPart::Word(Word::new("c"))
                ],
                true
            ))
        );
        assert_eq!(input, "");
    }

    #[test]
    fn test_parse_path_parts_relative_path_with_variable() {
        let mut input = "a/${b}/c";
        assert_eq!(
            parse_path_parts(&mut input),
            Ok((
                false,
                vec![
                    PathPart::Word(Word::new("a")),
                    PathPart::Variable(Variable::new("b")),
                    PathPart::Word(Word::new("c"))
                ],
                false
            ))
        );
        assert_eq!(input, "");
    }

    #[test]
    fn test_eval() {
        let key = String::from("TEST");
        let value = String::from("test");
        let input = format!("a/${{{}}}/c", key);
        let output = format!("a/{}/c", value);
        let mut env = Environment::new();
        env.insert(key, value);
        let varpath = VarPath::from_str(&input).unwrap();
        assert_eq!(
            PathBuf::from(varpath.eval(&env).unwrap()),
            std::path::PathBuf::from(output)
        );
    }

    #[test]
    fn test_eval_leading_and_trailing_slash() {
        let key = String::from("TEST");
        let value = String::from("test");
        let input = format!("/a/${{{}}}/c/", key);
        let output = format!("/a/{}/c/", value);
        let varpath = VarPath::from_str(&input).unwrap();
        let mut env = Environment::new();
        env.insert(key, value);
        assert_eq!(
            PathBuf::from(varpath.eval(&env).unwrap()),
            std::path::PathBuf::from(output)
        );
    }

    #[test]
    fn test_eval_leading_slash() {
        let key = String::from("TEST");
        let value = String::from("test");
        let input = format!("/a/${{{}}}/c", key);
        let output = format!("/a/{}/c", value);
        let mut env = Environment::new();
        env.insert(key, value);
        let varpath = VarPath::from_str(&input).unwrap();
        assert_eq!(
            PathBuf::from(varpath.eval(&env).unwrap()),
            std::path::PathBuf::from(output)
        );
    }

    #[test]
    fn test_eval_trailing_slash() {
        let key = String::from("TEST");
        let value = String::from("test");
        let input = format!("a/${{{}}}/c/", key);
        let output = format!("a/{}/c/", value);
        let mut env = Environment::new();
        env.insert(key, value);
        let varpath = VarPath::from_str(&input).unwrap();
        assert_eq!(
            PathBuf::from(varpath.eval(&env).unwrap()),
            std::path::PathBuf::from(output)
        );
    }
}
