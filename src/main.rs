mod d00;
mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;

use std::env::args;
use std::io::{stdin, Read};

const CMDS: &[(&str, fn(&mut dyn Read))] = &[
    ("d00p1", d00::run_part1),
    ("d00p2", d00::run_part2),
    ("d01p1", d01::run_part1),
    ("d01p2", d01::run_part2),
    ("d02p1", d02::run_part1),
    ("d02p2", d02::run_part2),
    ("d03p1", d03::run_part1),
    ("d03p2", d03::run_part2),
    ("d04p1", d04::run_part1),
    ("d04p2", d04::run_part2),
    ("d05p1", d05::run_part1),
    ("d05p2", d05::run_part2),
    ("d06p1", d06::run_part1),
    ("d06p2", d06::run_part2),
    ("d07p1", d07::run_part1),
    ("d07p2", d07::run_part2),
    ("d08p1", d08::run_part1),
    ("d08p2", d08::run_part2),
    ("d09p1", d09::run_part1),
    ("d09p2", d09::run_part2),
    ("d10p1", d10::run_part1),
    ("d10p2", d10::run_part2),
];

fn find_cmd(name: &str) -> Option<fn(&mut dyn Read)> {
    for (fun_name, fun) in CMDS.iter() {
        if name == *fun_name {
            return Some(*fun);
        }
    }
    return None;
}

fn help(name: &str) {
    println!("usage: {} <subcommand>", name);
    println!("subcommands:");
    for (name, _) in CMDS.iter() {
        println!("    {}", name);
    }
}

fn main() {
    let (subcmd_name, prog_name) = {
        let mut args = args();
        let prog_name = args.next().unwrap();

        let subcmd = match args.next() {
            None => {
                help(&prog_name);
                return;
            }
            Some(cmd) => cmd,
        };
        (subcmd, prog_name)
    };

    let run_fn = match find_cmd(&subcmd_name) {
        Some(f) => f,
        None => {
            println!("Unexpected subcommand name \"{}\"", subcmd_name);
            help(&prog_name);
            return;
        }
    };

    run_fn(&mut stdin())
}
