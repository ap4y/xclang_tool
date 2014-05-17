use std::io::fs;

use rclang::compilation_database::CompilationDatabase;
use rclang::translation_unit::TranslationUnit;
use rclang::types::*;

pub fn syntax_check(original_file: &Path, input_file: &Path) -> Result<StrBuf, ~str> {
    let tu = try!(tu_for(original_file, input_file));

    let mut output = StrBuf::new();
    for diagnostic in tu.diagnostics().iter() {
        output.push_str(diagnostic.formatted);
    }

    return Ok(output);
}

pub fn code_completion(original_file: &Path, input_file: &Path, location: &str, prefix: &str) -> Result<StrBuf, ~str> {
    let tu = try!(tu_for(original_file, input_file));

    let loc_split: ~[&str] = location.split_str(":").collect();
    if loc_split.len() != 2 { return Err(~"Location should be in format line:column") }

    let line = from_str(loc_split[0]).unwrap();
    let column: uint = from_str(loc_split[1]).unwrap();

    let completions = tu.complete_code_at(input_file, line, column - prefix.len() + 1);
    let mut c_iterator = completions.iter().filter(|&c| {
        c.availability == CXAvailability_Available
    });

    let mut output = StrBuf::new();
    for completion in c_iterator {
        let result = completion.to_yas();
        if result.contains(prefix) { output.push_str(result) }
    };

    return Ok(output);
}

fn c_db_for(file_path: &Path) -> Result<CompilationDatabase, ~str> {
    let mut path = file_path.clone();
    while path.pop() {
        let files = match fs::readdir(&path) { Ok(f) => f, Err(e) => fail!(e) };
        let result = files.iter().find(|&f| {
            match f.filename_str() {
                Some(name) => (name == "compile_commands.json"),
                None => false
            }
        });

        if result.is_some() { break; }
    };

    if path.components().len() == 0 {
        return Err(~"Unable to find directory with compile_commands.json");
    }

    match CompilationDatabase::from_directory(&path) {
        Ok(c_db) => Ok(c_db),
        Err(_e) => Err(~"Unable to create compilation database")
    }
}

fn tu_for(original_file: &Path, input_file: &Path) -> Result<TranslationUnit, ~str> {
    let c_db = try!(c_db_for(original_file));

    match c_db.compilation_command_for(original_file) {
        Some(c_data) => Ok(TranslationUnit::new(&c_data, input_file)),
        None => Err(~"Unable to find compilation command in the database")
    }
}

#[cfg(test)]
mod test {

    use std::os;
    use super::{syntax_check, code_completion};

    #[test]
    fn test_syntax_check() {
        let input = os::getcwd().join("tests/TestApplication/TestApplication/AppDelegate.m");
        let diagnostic = syntax_check(&input, &input);
        assert!(diagnostic.is_ok());
        assert!(diagnostic.unwrap() == StrBuf::from_str("/Users/arthurevstifeev/github/xclang_tool/tests/TestApplication/TestApplication/AppDelegate.m:17:15: warning: unused variable 'testString' [-Wunused-variable]"));
    }

    #[test]
    fn test_code_completion() {
        let input = os::getcwd().join("tests/TestApplication/TestApplication/AppDelegate.m");
        let completion = code_completion(&input, &input, "16:18", "pre");
        assert!(completion.is_ok());
        assert!(completion.unwrap() == StrBuf::from_str("prepareToTest\tvoid"));
    }
}
