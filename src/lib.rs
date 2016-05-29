//! Contains definitions of CTL formulas and propositions
//!

extern crate xml;

pub mod parser;
mod xml_util;

use std::fmt;
use CTLFormula::*;
use Value::*;

/// An evaluable value, either a constant or a place name
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Value {
    Const(u32),
    Ref(String),
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
pub enum CTLFormula {
    True,
    False,
    LT(Value, Value),
    LE(Value, Value),
    GT(Value, Value),
    GE(Value, Value),
    Fireable(String),
    Not(Box<CTLFormula>),
    EX(Box<CTLFormula>),
    AX(Box<CTLFormula>),
    EF(Box<CTLFormula>),
    AF(Box<CTLFormula>),
    EG(Box<CTLFormula>),
    AG(Box<CTLFormula>),
    And(Vec<CTLFormula>),
    Or(Vec<CTLFormula>),
    EU(Box<CTLFormula>, Box<CTLFormula>),
    AU(Box<CTLFormula>, Box<CTLFormula>),
}

impl fmt::Display for CTLFormula {
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

fn print_list(f: &mut fmt::Formatter, op: &str, items: &Vec<CTLFormula>) -> fmt::Result {
    if items.is_empty() {
        write!(f, "({})", op)
    } else {
        let r1 = write!(f, "({}", items[0]);
        let r2 = items.into_iter().skip(1).fold(r1, |acc, child| acc.and_then(|_| write!(f, " {} {}", op, child)));
        r2.and_then(|_| write!(f, ")"))
    }
}
