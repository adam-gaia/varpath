use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;
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
mod variable;
use eyre::Result;
use serde::de::{self, Deserializer, Visitor};
use serde::ser::Serializer;
use thiserror::Error;
use variable::parse_variable;
use variable::Variable;

pub mod environment;
use environment::Environment;
use environment::EnvironmentBuilder;

mod varpath;
use varpath::PathPart;
pub use varpath::VarPath;

#[cfg(test)]
mod tests {
    use super::*;
    use homedir::my_home;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_no_lookup() {
        let env = EnvironmentBuilder::default().build();
        let actual = VarPath::from_str("/a/b/c").unwrap().eval(&env).unwrap();
        let expected = PathBuf::from("/a/b/c");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_lookup_no_braces() {
        let key = String::from("key");
        let value = String::from("b");
        let env = EnvironmentBuilder::default().set(&key, &value).build();
        let actual = VarPath::from_str("/a/$key/c").unwrap().eval(&env).unwrap();
        let expected = PathBuf::from("/a/b/c");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_lookup_braces() {
        let key = String::from("key");
        let value = String::from("b");
        let env = EnvironmentBuilder::default().set(&key, &value).build();
        let actual = VarPath::from_str("/a/${key}/c")
            .unwrap()
            .eval(&env)
            .unwrap();
        let expected = PathBuf::from("/a/b/c");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_process_env() {
        let input = "/a/${PWD}/c";
        let env = EnvironmentBuilder::default().with_process_env().build();
        let actual = VarPath::from_str(input).unwrap().eval(&env).unwrap();
        assert_ne!(PathBuf::from(input), actual);
    }

    #[test]
    fn test_undefined_var() {
        let env = EnvironmentBuilder::default().build();
        let result = VarPath::from_str("/a/${BAD_BAD_BAD}/c").unwrap().eval(&env);
        assert!(result.is_err());
    }

    #[test]
    fn test_home() {
        let input = "${HOME}";
        let env = EnvironmentBuilder::default().with_process_env().build();
        let actual = VarPath::from_str(input).unwrap().eval(&env).unwrap();
        let expected = homedir::my_home().unwrap().unwrap();
        assert_eq!(expected, actual);
    }
}
