#!/usr/local/bin/rustscript

#[macro_use] extern crate shells;

use std::process;

let (code, stdout, _stderr) = sh!("ping -c 3 127.0.0.1");
println!("Ping output: {}", stdout);
println!("hello");
process::exit(code);
