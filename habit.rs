#![allow(dead_code)]
#![feature(struct_variant)]

#![feature(globs)]
#![feature(macro_rules)]

// For regex usage
#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

extern crate serialize;
extern crate core;
extern crate time;
extern crate getopts;

use getopts::*;
use core::fmt::{Show};
use std::os;
use std::io;
use std::io::fs;
//use std::io::stdio::{print, println};

//use api::conn::get;
use api::id::Id;
//use api::task::*;
use api::user::*;
use api::request::*;

mod api;

fn print<T:Show>(tasks: Vec<T>) {
    for t in tasks.iter() {
        println!("{}", t);
    }
}

// TODO recursive?
// Also create dir for file
fn create_dir(dir: &Path) {
    if dir.is_file() {
        fail!("dir: {} is a file, aborting.", dir.display());
    }
    if !dir.is_dir() {
        println!("Creating dir: {}", dir.display());
        match fs::mkdir(dir, io::UserRWX) {
            Ok(_) => (),
            Err(e) => fail!("Failed to create dir: {}", e),
        };
    }
}

struct Env {
    configdir: Path,
    cachedir: Path,
    id: Id,
}

impl Env {
    pub fn new() -> Env {
        let homedir = match os::homedir() {
            Some(d) => d,
            None => fail!("Could not find your homedir!"),
        };

        // TODO lazy?
        let configdir = homedir.join(".habitrpg-viewer");
        create_dir(&configdir);

        let cachedir = configdir.join("cache");
        create_dir(&cachedir);

        let id = Id::from_file(&configdir.join("id.json"));

        Env {
            configdir: configdir,
            cachedir: cachedir,
            id: id,
        }
    }
}

fn help(opts: &[OptGroup]) {
    let usage = usage("Command line interface to fetch information from habitrpg", opts);
    println!("{}", usage);
}

fn main() {
    //println!("Your homedir is: {}", homedir.display());
    let args: Vec<String> = os::args().iter()
                                      .map(|x| x.to_string())
                                      .collect();

    let opts = [
        optflag("h", "help", "Print this"),
        optflag("", "todos", "Output unfinished todos"),
        optflag("", "dailys", "Output dailys"),
        optflag("", "habits", "Output habits"),
        optflag("v", "verbose", "Verbose output"),
        optflag("d", "debug", "Debug"),
        optflag("", "conky", "Add format specifiers for conky display. Harr."),
    ];

    let args = match getopts(args.tail(), opts) {
        Ok(m) => m,
        Err(e) => fail!("Failed to get args: {}", e),
    };

    if args.opt_present("h") {
        help(opts);
        return;
    }

    // TODO debug/verbose flags to control output!
    //let debug = args.opt_present("debug");
    let verbose = args.opt_present("verbose");
    let conky = args.opt_present("conky");

    let env = Env::new();

    if verbose {
        //println!("Config dir: {}", configdir.display());
        println!("Registering with");
        println!("  api_token: {}", env.id.api_token);
        println!("  user_id: {}", env.id.user_id);
    }

    let user = User::fetch(&env.cachedir, &env.id);

    if args.opt_present("todos") {
        for t in user.unfinished_todos().iter() {
            if conky {
                print!("${{voffset 8}}");
            }
            println!("{}", t);
        }
    }
    else if args.opt_present("dailys") {
        for t in user.dailys().iter() {
            if conky {
                print!("${{voffset 8}}");
            }
            println!("{}", t);
        }
    }
    else if args.opt_present("habits") {
        for t in user.habits().iter() {
            if conky {
                print!("${{voffset 8}}");
            }
            println!("{}", t);
        }
    }
    else {

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
}

