//! Contains definitions of CTL formulas and propositions
//!

extern crate xml;

pub mod parser;
mod xml_util;

use std::fmt;
use Formula::*;
use Value::*;

/// An evaluable value, either a constant or a place name
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Value {
    Const(u32),
    Ref(String),
}

impl Value {

    pub fn new_ref(name: &str) -> Value {
        Ref(name.to_string())
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Const(ref v) => write!(f, "{}", v),
            &Ref(ref v) => write!(f, "{}", v),
        }
    }
}

/// A CTL formula represented by an operator and nested formulas / values.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Formula {
    True,
    False,
    LT(Value, Value),
    LE(Value, Value),
    GT(Value, Value),
    GE(Value, Value),
    Fireable(String),
    Not(Box<Formula>),
    EX(Box<Formula>),
    AX(Box<Formula>),
    EF(Box<Formula>),
    AF(Box<Formula>),
    EG(Box<Formula>),
    AG(Box<Formula>),
    And(Vec<Formula>),
    Or(Vec<Formula>),
    EU(Box<Formula>, Box<Formula>),
    AU(Box<Formula>, Box<Formula>),
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &False => write!(f, "false"),
            &True => write!(f, "true"),
            &LT(ref left, ref right) => write!(f, "{} < {}", left, right),
            &GT(ref left, ref right) => write!(f, "{} > {}", left, right),
            &LE(ref left, ref right) => write!(f, "{} <= {}", left, right),
            &GE(ref left, ref right) => write!(f, "{} >= {}", left, right),
            &Fireable(ref name) => write!(f, "fire {}", name),
            &Not(ref inner) => write!(f, "!({})", inner),
            &EX(ref inner) => write!(f, "EX({})", inner),
            &AX(ref inner) => write!(f, "AX({})", inner),
            &EF(ref inner) => write!(f, "EF({})", inner),
            &AF(ref inner) => write!(f, "AF({})", inner),
            &EG(ref inner) => write!(f, "EG({})", inner),
            &AG(ref inner) => write!(f, "AG({})", inner),
            &EU(ref left, ref right) => write!(f, "(E {} U {})", left, right),
            &AU(ref left, ref right) => write!(f, "(A {} U {})", left, right),
            &And(ref inner) => print_list(f, "&&", inner),
            &Or(ref inner) => print_list(f, "||", inner),
        }
    }
}

fn print_list(f: &mut fmt::Formatter, op: &str, items: &Vec<Formula>) -> fmt::Result {
    if items.is_empty() {
        write!(f, "({})", op)
    } else {
        let r1 = write!(f, "({}", items[0]);
        let r2 = items.into_iter().skip(1).fold(r1, |acc, child| acc.and_then(|_| write!(f, " {} {}", op, child)));
        r2.and_then(|_| write!(f, ")"))
    }
}
