#![crate_id = "rclang:0.1"]
#![desc = "Partial rust binding for clang"]
#![license = "MIT"]
#![crate_type = "lib"]

#![feature(globs, phase)]

#[phase(syntax, link)] extern crate log;
extern crate std;
extern crate collections;
use std::libc;

mod ffi;
pub mod cx_string;
pub mod cdb;
pub mod translation_unit;
pub mod source_location;
pub mod code_completion;
pub mod diagnostic;

#[allow(non_camel_case_types)]
pub mod types;

#[cfg(test)]
mod tests {
    use std::os;

    use cdb::*;
    use translation_unit::*;

    fn c_db_dir() -> Path {
        os::getcwd().join("./tests/TestApplication")
    }
    fn file_path() -> Path {
        c_db_dir().join("TestApplication/AppDelegate.m")
    }

    fn compilation_database() -> CompilationDatabase {
        match CompilationDatabase::from_directory(&c_db_dir()) {
            Ok(database) => database,
            Err(_e) => fail!("Unable to load database")
        }
    }

    fn compilation_data() -> CompilationCommand {
        let database = compilation_database();
        let result = database.compilation_command_for(&file_path());
        match result {
            Some(c_data) => c_data,
            None => fail!("Failed requesting compilation data")
        }
    }

    fn translation_unit() -> TranslationUnit {
        match compilation_database().translation_unit_for(&file_path()) {
            Some(tu) => tu,
            None     => fail!("Unable to return translation unit")
        }
    }

    #[test]
    fn create_cdb_from_directory() {
        compilation_database();
    }

    #[test]
    fn compile_command() {
        let c_data = compilation_data();
        assert!(c_data.args.len() == 91);
    }

    #[test]
    fn code_completion() {
        let completions = translation_unit().complete_code_at(file_name(), 20, 20);
        assert!(completions.len() == 47);
    }

    #[test]
    fn diagnostic() {
        let diagnostics = translation_unit().diagnostics();
        for diagnostic in diagnostics.iter() {
            debug!("diagnostic: {}", diagnostic.formatted);
        }
        assert!(diagnostics.len() == 1);
    }

    #[test]
    fn go_to_definition() {
        let source_location = translation_unit().go_to_definition(&compilation_database(), file_name(), 20, 20);
        debug!("go_to_definition.file: {}", source_location.file);
        debug!("go_to_definition.line: {}", source_location.line);
        debug!("go_to_definition.column: {}", source_location.column);

        assert!(source_location.line == 66);
        assert!(source_location.column == 9);
    }
}
