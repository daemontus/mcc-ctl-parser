extern crate ctl;

use ctl::*;
use ctl::parser::*;
use ctl::Formula::*;
use ctl::Value::*;

///Testing No, AU, And, LE
#[test]
fn cardinality_0() {
    let formula_list = read_formula_list_file("tests/inputs/CTLCardinality.xml");
    let f = Not(Box::new(
        AU(Box::new(
            LE(Const(1), Value::new_ref("KdStarG"))
        ), Box::new(
            And(vec![
                LE(Value::new_ref("Gab1"), Value::new_ref("AktP3")),
                LE(Const(1), Value::new_ref("Enz"))
            ])
        ))
    ));
    assert_eq!(f, formula_list[0]);
}

//Testing LE
#[test]
fn cardinality_1() {
    let formula_list = read_formula_list_file("tests/inputs/CTLCardinality.xml");
    let f = LE(Const(1), Value::new_ref("GStarP3kP3"));
    assert_eq!(f, formula_list[1]);
}

///Testing EF, Not, And, LE
#[test]
fn cardinality_2() {
    let formula_list = read_formula_list_file("tests/inputs/CTLCardinality.xml");
    let f = EG(Box::new(
        Not(Box::new(
            Not(Box::new(
                And(vec![
                    LE(Const(3), Value::new_ref("GStarP3")),
                    LE(Value::new_ref("AktStar"), Value::new_ref("KdStarGStarPgP3"))
                ])
            ))
        ))
    ));
    assert_eq!(f, formula_list[2]);
}

///Testing EG, AX, And, EU, Not, LE
#[test]
fn cardinality_3() {
    let formula_list = read_formula_list_file("tests/inputs/CTLCardinality.xml");
    let f = And(vec![
        EG(Box::new(
            AX(Box::new(
                LE(Value::new_ref("KdStarGStar"), Value::new_ref("GStarPgP3"))
            ))
        )),
        And(vec![
            EU(Box::new(
                LE(Value::new_ref("PtP3"), Value::new_ref("KdStarGStarP3kStarP3P2"))
            ), Box::new(
                LE(Value::new_ref("AktP3"), Value::new_ref("KdStarGStarPgStarP3P2"))
            )),
            Not(Box::new(
                LE(Value::new_ref("KdStarGStarPgStar"), Value::new_ref("KdStarGP3"))
            ))
        ])
    ]);
    assert_eq!(f, formula_list[3]);
}

///Testing AF, Not, EX, LE
#[test]
fn cardinality_4() {
    let formula_list = read_formula_list_file("tests/inputs/CTLCardinality.xml");
    let f = AF(Box::new(
        Not(Box::new(
            EX(Box::new(
                LE(Const(2), Value::new_ref("KdStarGStarPg"))
            ))
        ))
    ));
    assert_eq!(f, formula_list[4]);
}

///Testing And, EU, AG, Not, And, LE
#[test]
fn cardinality_5() {
    let formula_list = read_formula_list_file("tests/inputs/CTLCardinality.xml");
    let f = And(vec![
        EU(Box::new(
            LE(Const(2), Value::new_ref("GStarP3kP3"))
        ), Box::new(
            LE(Value::new_ref("KdStarGStarPgStar"), Value::new_ref("GP3"))
        )),
        AG(Box::new(
            Not(Box::new(
                And(vec![
                    LE(Const(2), Value::new_ref("Pip3")),
                    LE(Value::new_ref("KdStarPgStarP2"), Value::new_ref("KdStarGStarP3k")),
                ])
            ))
        ))
    ]);
    assert_eq!(f, formula_list[5]);
}

///Testing Not, Or, EX, And, EG, LE
#[test]
fn cardinality_6() {
    let formula_list = read_formula_list_file("tests/inputs/CTLCardinality.xml");
    let f = Not(Box::new(
        Or(vec![
            EX(Box::new(
                And(vec![
                    LE(Value::new_ref("KdStarGStarP3kStarP3"), Value::new_ref("DAGE")),
                    LE(Const(3), Value::new_ref("KdStarG")),
                ])
            )),
            Or(vec![
                EG(Box::new(
                    LE(Const(2), Value::new_ref("KdStar"))
                )),
                Or(vec![
                    Or(vec![
                        LE(Value::new_ref("KdStarPgStarP2"), Value::new_ref("KdStarPgStar")),
                        LE(Value::new_ref("KdStarPg"), Value::new_ref("GP3")),
                    ]),
                    And(vec![
                        LE(Value::new_ref("KdStarGStarPgStarP2"), Value::new_ref("DAGE")),
                        LE(Const(3), Value::new_ref("GStarPgP3")),
                    ]),
                ])
            ])
        ])
    ));
    assert_eq!(f, formula_list[6]);
}

///Testing AX, And, Or
#[test]
fn cardinality_8() {
    let formula_list = read_formula_list_file("tests/inputs/CTLCardinality.xml");
    let f = AX(Box::new(
        And(vec![
            Or(vec![
                LE(Value::new_ref("Pip3"), Value::new_ref("KdStarGStarPgStarP2")),
                And(vec![
                    LE(Const(1), Value::new_ref("AktStar")),
                    LE(Value::new_ref("PtP3"), Value::new_ref("KdStarGStarPgP3"))
                ])
            ]),
            And(vec![
                LE(Value::new_ref("KdStarGStarP3kStarP2"), Value::new_ref("KdStarGStarPgStarP3P2")),
                Or(vec![
                    LE(Value::new_ref("KdStarGStarP3kStarP3"), Value::new_ref("Pip2")),
                    LE(Value::new_ref("KdStarGStarP3kStarP3P2"), Value::new_ref("P3k")),
                ])
            ]),
        ])
    ));
    assert_eq!(f, formula_list[8]);
}

///Testing EF, AF, Or, LE
#[test]
fn cardinality_9() {
    let formula_list = read_formula_list_file("tests/inputs/CTLCardinality.xml");
    let f = EF(Box::new(
        AF(Box::new(
            Or(vec![
                LE(Const(1), Value::new_ref("KdStar")),
                LE(Const(3), Value::new_ref("KdStarGStarP3kStarP3P2")),
            ])
        ))
    ));
    assert_eq!(f, formula_list[9]);
}

///Testing Not, EG, LE
#[test]
fn cardinality_10() {
    let formula_list = read_formula_list_file("tests/inputs/CTLCardinality.xml");
    let f = Not(Box::new(
        Not(Box::new(
            EG(Box::new(
                LE(Value::new_ref("AktStar"), Value::new_ref("DAG"))
            ))
        ))
    ));
    assert_eq!(f, formula_list[10]);
}

///Testing And, Or, AF, Fire
#[test]
fn fireablility_15() {
    let formula_list = read_formula_list_file("tests/inputs/CTLFireability.xml");
    let f = And(vec![
        Fireable(vec!["k5".to_string()]),
        Not(Box::new(
            AF(Box::new(
                Not(Box::new(
                    Or(vec![
                        Fireable(vec!["k17".to_string(), "k20".to_string(), "k33".to_string()]),
                        Fireable(vec!["k52".to_string()]),
                    ])
                ))
            ))
        )),
    ]);
    assert_eq!(f, formula_list[15]);
}

///Testing AG, EF, And, Or, Not, Fire
#[test]
fn fireablility_14() {
    let formula_list = read_formula_list_file("tests/inputs/CTLFireability.xml");
    let f = AG(Box::new(
        EF(Box::new(
            And(vec![
                    Not(Box::new(
                        Fireable(vec!["k13".to_string()])
                    )),
                    Or(vec![
                        Fireable(vec!["k45".to_string()]),
                        Fireable(vec!["k11".to_string()]),
                    ])
            ])
        ))
    ));
    assert_eq!(f, formula_list[14]);
}
