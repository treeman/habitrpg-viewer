#![feature(slicing_syntax)]

extern crate habitrpg;
extern crate getopts;
extern crate time;

use std::{ io, os };
use getopts::{
    optflag,
    getopts,
    usage
};

use env::Env;

mod env;
mod cache;

fn main() {
    let args = os::args();

    let opts = [
        optflag("h", "help", "Display this helpful message"),
        optflag("", "todos", "Output unfinished todos"),
        optflag("", "dailys", "Output dailys"),
        optflag("", "habits", "Output habits"),
        optflag("", "conky", "Add format specifiers for conky display. Harr."),
    ];

    let progname = args[0].clone();
    let usage = usage("View info from habitrpg.", &opts);

    let args = match getopts(args.tail(), &opts) {
        Ok(x) => x,
        Err(e) => panic!("{}", e),
    };

    if args.opt_present("h") {
        help(progname[], usage[]);
        return;
    }

    let env = Env::new();

    if args.opt_present("todos") {
        todos(&env);
    } else if args.opt_present("dailys") {
        dailys(&env);
    } else if args.opt_present("habits") {
        habits(&env);
    } else {
        user_info(&env);
    }
}

fn help(progname: &str, usage: &str) {
    println!("Usage: {} [OPTION]", progname);
    io::println(usage);
}

fn todos(env: &Env) {
    let user = cache::get_user(env);

    for t in user.unfinished_todos().iter() {
        println!("{}", t);
    }
}

fn dailys(env: &Env) {
    let user = cache::get_user(env);

    for t in user.dailys().iter() {
        println!("{}", t);
    }
}

fn habits(env: &Env) {
    let user = cache::get_user(env);

    for t in user.habits().iter() {
        println!("{}", t);
    }
}

fn user_info(env: &Env) {
    let user = cache::get_user(env);

    user.print_char();
    user.print_char_stats();
    println!("Tasks");
    user.print_task_stats();

    println!("\nDailys\n-------");
    for t in user.dailys().iter() {
        println!("{}", t);
    }
    println!("\nTodos\n-----");
    for t in user.unfinished_todos().iter() {
        println!("{}", t);
    }
}

