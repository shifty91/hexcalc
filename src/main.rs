// Copyright 2019,2020 Kurt Kanzenbach <kurt@kmk-computers.de>
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

extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate liner;
extern crate termion;
extern crate clap;
extern crate libc;

use std::io::prelude::*;
use liner::{Context, Completer, Prompt};
use termion::{color, style};
use clap::App;
use hexcalcparser::HexCalcParser;

mod hexcalcparser;

struct EmptyCompleter;

impl Completer for EmptyCompleter {
    fn completions(&mut self, _start: &str) -> Vec<String> {
        Vec::new()
    }
}

#[allow(unused_must_use)]
fn main() {
    App::new("hexcalc")
        .version("0.2.0")
        .about("Hex Calculator")
        .author("Kurt Kanzenbach <kurt@kmk-computers.de>")
        .get_matches();

    let parser = HexCalcParser::new();
    let istty = unsafe { libc::isatty(libc::STDIN_FILENO as i32) } != 0;

    if ! istty {
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            let line = line.unwrap();
            match parser.parse(&line) {
                Ok(res) => println!("{} : 0x{:x} : {:#b}", res, res, res),
                Err(e)  => println!("Failed to parse line '{}':\n{}", line, e),
            }
        }
        return;
    }

    let mut con = Context::new();

    loop {
        let res = con.read_line(Prompt::from("=> "),
                                Some(Box::new(|s| String::from(s))),
                                &mut EmptyCompleter);

        if res.is_err() {
            break;
        }

        let res = res.unwrap();
        if res.is_empty() {
            continue;
        }

        match parser.parse(&res) {
            Ok(res) => println!("{}{}{} : 0x{:x} : {:#b}{}", style::Bold,
                                color::Fg(color::Green),
                                res, res, res, style::Reset),
            Err(e)  => println!("Failed to parse line '{}':\n{}{}{}{}", res,
                                style::Bold, color::Fg(color::Red), e,
                                style::Reset),
        }
        con.history.push(res.into());
    }
}
