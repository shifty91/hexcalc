// Copyright 2019-2023 Kurt Kanzenbach <kurt@kmk-computers.de>
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
use pest::iterators::Pairs;
use pest::pratt_parser::{PrattParser, Assoc, Op};

#[derive(Parser)]
#[grammar = "calc.pest"]
pub struct CalcParser;

pub struct HexCalcParser {
    parser: PrattParser<Rule>,
}

///
/// HexCalcParser. The parser is based on pest and on the example
/// provided by the Docs:
///
///  -> https://pest.rs/book/
///
impl HexCalcParser {
    pub fn new() -> HexCalcParser {
        let parser = PrattParser::new()
            .op(Op::infix(Rule::add,        Assoc::Left) |
                Op::infix(Rule::subtract,   Assoc::Left))
            .op(Op::infix(Rule::multiply,   Assoc::Left) |
                Op::infix(Rule::divide,     Assoc::Left))
            .op(Op::infix(Rule::and,        Assoc::Left) |
                Op::infix(Rule::or,         Assoc::Left) |
                Op::infix(Rule::xor,        Assoc::Left) |
                Op::infix(Rule::shiftleft,  Assoc::Left) |
                Op::infix(Rule::shiftright, Assoc::Left));

        HexCalcParser { parser }
    }

    fn eval(&self, expression: Pairs<Rule>) -> i64 {
        self.parser.map_primary(|primary| match primary.as_rule() {
            Rule::hex_num => i64::from_str_radix(
                primary.as_str().trim_start_matches("0x"), 16).unwrap(),
            Rule::oct_num => i64::from_str_radix(
                primary.as_str().trim_start_matches("o"), 8).unwrap(),
            Rule::bin_num => i64::from_str_radix(
                primary.as_str().trim_start_matches("b"), 2).unwrap(),
            Rule::dec_num => primary.as_str().parse::<i64>().unwrap(),
            Rule::expr => self.eval(primary.into_inner()),
            _ => unreachable!(),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::add        => lhs + rhs,
            Rule::subtract   => lhs - rhs,
            Rule::multiply   => lhs * rhs,
            Rule::divide     => lhs / rhs,
            Rule::and        => lhs & rhs,
            Rule::or         => lhs | rhs,
            Rule::xor        => lhs ^ rhs,
            Rule::shiftleft  => lhs << rhs,
            Rule::shiftright => lhs >> rhs,
            _ => unreachable!(),
        })
        .parse(expression)
    }

    pub fn parse(&self, line: &str) -> Result<i64, String> {
        match CalcParser::parse(Rule::calculation, &line) {
            Ok(mut calculation) => Ok(self.eval(calculation.next().unwrap().into_inner())),
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
    let res3 = parser.parse("2 << 1").unwrap();
    let res4 = parser.parse("4 >> 2").unwrap();
    assert_eq!(res0, 0x01);
    assert_eq!(res1, 0xff);
    assert_eq!(res2, 0x00);
    assert_eq!(res3, 0x04);
    assert_eq!(res4, 0x01);
}

#[test]
fn test_prec() {
    let parser = HexCalcParser::new();
    let res = parser.parse("0xff & 0x02 * (3 + 4)").unwrap();
    assert_eq!(res, 14);
}
