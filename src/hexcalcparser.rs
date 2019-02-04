// Copyright 2019 Kurt Kanzenbach <kurt@kmk-computers.de>
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice,
//    this list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
//    this list of conditions and the following disclaimer in the documentation
//    and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

use std::i64;
use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{PrecClimber, Assoc, Operator};

#[derive(Parser)]
#[grammar = "calc.pest"]
pub struct CalcParser;

pub struct HexCalcParser {
    climber: PrecClimber<Rule>,
}

///
/// HexCalcParser. The parser is based on pest and on the example
/// provided by the Docs:
///
///  -> https://pest.rs/book/
///
impl HexCalcParser {
    pub fn new() -> HexCalcParser {
        let climber = PrecClimber::new(vec![
            Operator::new(Rule::add,      Assoc::Left) |
            Operator::new(Rule::subtract, Assoc::Left),
            Operator::new(Rule::multiply, Assoc::Left) |
            Operator::new(Rule::divide,   Assoc::Left),
            Operator::new(Rule::and,      Assoc::Left) |
            Operator::new(Rule::or,       Assoc::Left) |
            Operator::new(Rule::xor,      Assoc::Left),
        ]);

        HexCalcParser { climber }
    }

    fn eval(&self, expression: Pairs<Rule>) -> i64 {
        self.climber.climb(
            expression,
            |pair: Pair<Rule>| match pair.as_rule() {
                Rule::hex_num => i64::from_str_radix(
                    pair.as_str().trim_left_matches("0x"), 16).unwrap(),
                Rule::oct_num => i64::from_str_radix(
                    pair.as_str().trim_left_matches("o"), 8).unwrap(),
                Rule::bin_num => i64::from_str_radix(
                    pair.as_str().trim_left_matches("b"), 2).unwrap(),
                Rule::dec_num => pair.as_str().parse::<i64>().unwrap(),
                Rule::expr => self.eval(pair.into_inner()),
                _ => unreachable!(),
            },
            |lhs: i64, op: Pair<Rule>, rhs: i64| match op.as_rule() {
                Rule::add      => lhs + rhs,
                Rule::subtract => lhs - rhs,
                Rule::multiply => lhs * rhs,
                Rule::divide   => lhs / rhs,
                Rule::and      => lhs & rhs,
                Rule::or       => lhs | rhs,
                Rule::xor      => lhs ^ rhs,
                _ => unreachable!(),
            },
        )
    }

    pub fn parse(&self, line: &str) -> Result<i64, String> {
        match CalcParser::parse(Rule::calculation, &line) {
            Ok(calculation) => Ok(self.eval(calculation)),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[test]
fn test_ops() {
    let parser = HexCalcParser::new();
    let res0 = parser.parse("1 + 2").unwrap();
    let res1 = parser.parse("2 - 1").unwrap();
    let res2 = parser.parse("1 * 2").unwrap();
    let res3 = parser.parse("8 / 4").unwrap();
    assert_eq!(res0, 3);
    assert_eq!(res1, 1);
    assert_eq!(res2, 2);
    assert_eq!(res3, 2);
}

#[test]
fn test_bitops() {
    let parser = HexCalcParser::new();
    let res0 = parser.parse("0xff & 0x01").unwrap();
    let res1 = parser.parse("0x1 | 0xfe").unwrap();
    let res2 = parser.parse("0xff ^ 0xff").unwrap();
    assert_eq!(res0, 0x01);
    assert_eq!(res1, 0xff);
    assert_eq!(res2, 0x00);
}

#[test]
fn test_prec() {
    let parser = HexCalcParser::new();
    let res = parser.parse("0xff & 0x02 * (3 + 4)").unwrap();
    assert_eq!(res, 14);
}
