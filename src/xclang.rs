#![feature(globs, phase)]

extern crate rclang;
extern crate getopts;

use getopts::*;
use std::os;
use std::io::fs;

use rclang::compilation_database::CompilationDatabase;
use rclang::translation_unit::TranslationUnit;
use rclang::types::*;

fn opts() -> ~[OptGroup] {
    ~[optflag("s", "syntax-check", "perform syntax check on the file"),
      optopt("c", "code-completion", "return completion options for the location(line:column)", "LOCATION"),
      optopt("p", "prefix", "prefix will be used for filtering completion results", "PREFIX"),
      optopt("o", "original", "path to the original file, used with commands on temp buffers", "PATH"),
      optflag("h", "help", "print this help menu")]
}

fn print_usage(program: &str, opts: &[OptGroup]) {
    let brief = format!("Usage: {} [options] file_path", program);
    println!("{}", getopts::usage(brief, opts));
}

fn c_db_for(file_path: &Path) -> CompilationDatabase {
    let mut path = file_path.clone();
    while path.pop() {
        let files = match fs::readdir(&path) { Ok(f) => f,  Err(e) => fail!(e) };
        let result = files.iter().find(|&f| {
            match f.filename_str() {
                Some(name) => (name == "compile_commands.json"),
                None => false
            }
        });

        if result.is_some() { break; }
    };

    if path.components().len() == 0 {
        fail!("Unable to find directory with compile_commands.json");
    }

    match CompilationDatabase::from_directory(&path) {
        Ok(c_db) => c_db,
        Err(_e) => fail!("Unable to create compilation database")
    }
}

fn syntax_check(file_path: &Path, original_file_path: &Path) {
    let c_db = c_db_for(original_file_path);

    let tu = match c_db.compilation_command_for(original_file_path) {
        Some(c_data) => TranslationUnit::new(&c_data, file_path),
        None => fail!("Unable to find compilation command in the database")
    };

    for diagnostic in tu.diagnostics().iter() {
        println!("{}", diagnostic.formatted);
    }
}

fn main() {
    let args = os::args();
    let program = args[0].clone();
    let matches = match getopts(args.tail(), opts()) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_err_msg()) }
    };

    if matches.opt_present("h") || matches.free.is_empty() {
        print_usage(program, opts());
        return;
    }

    let input = os::getcwd().join(matches.free.get(0).clone());

    if matches.opt_present("s") {
        let original_file = match matches.opt_str("o") {
            Some(f) => os::getcwd().join(f),
            None => input.clone()
        };
        syntax_check(&input, &original_file);
        return;
    }
}
