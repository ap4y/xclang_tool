#![crate_id = "xclang#0.1"]
#![desc = "CLI for common editor actions for Objective-C"]
#![license = "MIT"]

#![feature(globs)]

#![feature(phase)]
extern crate regex;
#[phase(syntax)] extern crate regex_macros;

extern crate collections;
extern crate serialize;
#[phase(syntax, link)] extern crate log;
extern crate getopts;

extern crate rclang;
extern crate rfsevents;

use getopts::*;
use std::os;

mod helpers;
mod xcodebuild;
mod xcodebuild_parser;

fn opts() -> ~[OptGroup] {
    ~[optopt("l", "location", "location(line:column) for completion", "LOCATION"),
      optopt("p", "prefix", "prefix for filtering completion results", "PREFIX"),
      optopt("o", "original", "path to the original file, used with commands on temp buffers", "PATH"),
      optopt("w", "workspace", "Workspace name(without extension), used with compilation-database", "WORKSPACE"),
      optopt("s", "scheme", "Scheme name(defaults to workspace), used with compilation-database", "SCHEME"),
      optopt("t", "sdk-target", "SDK(iphonesimulator7.0) to use with compilation-database", "TARGET"),
      optflag("c", "continuous", "Automatically refresh compilation database when new files added")]
}

fn print_usage(program: &str, opts: &[OptGroup]) {
    let commands_help = r##"
Available commands:
    help:                 print this help menu
    syntax-check:         perform syntax check on the file
    code-completion:      return completion options for the location(line:column)
    goto-definition:      return definition location for the specific location(line:column)
    compilation-database: performs project compilation and processes result into compilation database"##;

    let brief = format!("Usage: {} [command] [options] file_path\n{}", program, commands_help);
    println!("{}", getopts::usage(brief, opts));
}

fn parse_arguments(args: &[~str]) -> (~str, ~str, Matches, Path) {
    let program = args[0].clone();

    let option_matches = match getopts(args, opts()) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_err_msg()) }
    };

    if option_matches.free.len() < 3 {
        print_usage(program, opts());
        if option_matches.free.len() < 2 { fail!("Command can't be empty") }
        fail!("File can't be empty")
    };

    let command = option_matches.free.get(1).clone();
    let input = os::getcwd().join(option_matches.free.get(2).clone());

    return (program, command, option_matches, input);
}

pub fn main() {
    let (program, command, opt_matches, input) = parse_arguments(os::args());

    let original = match opt_matches.opt_str("o") {
        Some(f) => os::getcwd().join(f),
        None => input.clone()
    };

    if command == ~"help" {
        print_usage(program, opts());
        return;
    }

    if command == ~"code-completion" {
        let loc = match opt_matches.opt_str("l") {
            Some(l) => l, None => fail!("Missing completion location")
        };

        let prefix = match opt_matches.opt_str("p") { Some(p) => p, None => ~"" };
        return match helpers::code_completion(&original, &input, loc, prefix) {
            Ok(completion) => print!("{}", completion),
            Err(e) => fail!("{}", e)
        };
    }

    if command == ~"goto-definition" {
        let loc = match opt_matches.opt_str("l") {
            Some(l) => l, None => fail!("Missing completion location")
        };

        return match helpers::goto_definition(&original, &input, loc) {
            Ok(location) => print!("{}", location),
            Err(e) => fail!("{}", e)
        };
    }

    if command == ~"syntax-check" {
        return match helpers::syntax_check(&original, &input) {
            Ok(diagnostic) => print!("{}", diagnostic),
            Err(e) => fail!("{}", e)
        };
    }

    if command == ~"compilation-database" {
        let workspace = match opt_matches.opt_str("w") {
            Some(w) => w,
            None => fail!("Workspace can't be empty")
        };
        let scheme = match opt_matches.opt_str("s") { Some(s) => s, None => workspace.clone() };
        let sdk = match opt_matches.opt_str("t") { Some(t) => t, None => ~"iphonesimulator7.1" };

        let watcher = xcodebuild::XCodeBuildWatcher::new(os::getcwd(), workspace, scheme, sdk);
        let result = if opt_matches.opt_present("c") { watcher.watch() } else { watcher.run() };
        return match result { Ok(_) => (), Err(e) => fail!("{}", e) }
    }
}

#[cfg(test)]
mod test {

    use std::os;
    use super::{parse_arguments};

    #[test]
    fn test_parse_arguments() {
        let arguments = [~"xclang", ~"syntax-check", ~"-l", ~"10:10", ~"-o", ~"bar.m", ~"foo.m"];
        let (program, command, opt_matches, input) = parse_arguments(arguments);

        assert!(program == ~"xclang");
        assert!(command == ~"syntax-check");
        assert!(input == os::getcwd().join("foo.m"));
        assert!(opt_matches.opt_present("l"));
        assert!(opt_matches.opt_present("o"));
    }
}
