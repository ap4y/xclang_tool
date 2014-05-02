#![feature(phase)]
extern crate regex;
#[phase(syntax)] extern crate regex_macros;
extern crate collections;
#[phase(syntax, link)] extern crate log;

use collections::HashMap;

pub struct CommandData {
    pub directory: ~str,
    pub command:   ~str,
    pub file:      ~str
}

pub struct XCodeBuildParser {
    pch_map: HashMap<~str, ~str>
}

impl XCodeBuildParser {
    fn process_compiled_header(&mut self, command_line: &str) {
        let re = regex!(r"-c (?P<input>.*) -o (?P<output>.*)");
        let captures_opt = re.captures(command_line);
        if captures_opt.is_none() { return; }

        let captures = captures_opt.unwrap();
        self.pch_map.insert(captures.name("output").to_owned(), captures.name("input").to_owned());
    }

    fn parse_command_line(&self, command_line: &str) -> (~str, ~str) {
        let file_re = regex!(r"\B-c (?P<file>.*?)\s");
        let file_captures = file_re.captures(command_line).unwrap();
        let file = file_captures.name("file");

        let pch_re = regex!(r"-include (?P<pch>.*pch)");
        let (compiled_pch, original_pch) = match pch_re.captures(command_line) {
            Some(captures) => {
                let compiled = captures.name("pch").to_owned();
                let original = self.pch_map.get(&compiled);
                (original.to_owned(), compiled)
            },
            None => (~"", ~"")
        };
        let command = command_line.replace(compiled_pch, original_pch);

        (file.to_owned(), command.trim().to_owned())
    }

    pub fn parse_output(&mut self, xcodebuild_output: &str) -> Vec<CommandData> {
        let mut line_iterator = xcodebuild_output.lines();
        let mut result: Vec<CommandData> = Vec::new();

        loop {
            let command_line = line_iterator.skip_while(|&line| {
                !line.starts_with("CompileC") && !line.starts_with("ProcessPCH")
            }).next();
            if command_line.is_none() { break; }
            line_iterator.next();

            if command_line.unwrap().starts_with("ProcessPCH") {
                line_iterator.next(); // cd
                line_iterator.next(); // export LANG
                line_iterator.next(); // export PATH
                self.process_compiled_header(line_iterator.next().unwrap());
                continue;
            }

            let directory = line_iterator.next().unwrap().replace("    cd ", "");
            line_iterator.next(); // export LANG
            line_iterator.next(); // export PATH
            let (file, command) = self.parse_command_line(line_iterator.next().unwrap());
            result.push(CommandData{ directory: directory, command: command, file: file });
        }

        result
    }

    pub fn new() -> XCodeBuildParser {
        XCodeBuildParser { pch_map: HashMap::new() }
    }
}

#[cfg(test)]
mod test {

    use XCodeBuildParser;

    fn parser_with_pch() -> XCodeBuildParser {
        let output = r##"ProcessPCH /Users/arthurevstifeev/Library/Developer/Xcode/DerivedData/TestApplication-ggnvtdtbeunuqqgbdwrstauepclk/Build/Intermediates/PrecompiledHeaders/TestApplication-Prefix-gnsrlwixdykpdkeyczvocpwcwymh/TestApplication-Prefix.pch.pch TestApplication/TestApplication-Prefix.pch normal i386 objective-c com.apple.compilers.llvm.clang.1_0.compiler
    cd /Users/arthurevstifeev/github/xclang_tool/tests/TestApplication
    export LANG=en_US.US-ASCII
    export PATH="/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneSimulator.platform/Developer/usr/bin:/Applications/Xcode.app/Contents/Developer/usr/bin:/usr/local/bin:/usr/local/bin:/usr/local/sbin:/usr/bin:/bin:/usr/sbin:/sbin:/Users/arthurevstifeev/.gem/ruby/2.0.0/bin"
    /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/clang -x objective-c-header -arch i386 -c /foo/TestApplication-Prefix.pch -o /bar/TestApplication-Prefix.pch.pch"##;

        let mut parser = XCodeBuildParser::new();
        parser.parse_output(output);
        parser
    }

    #[test]
    fn compile_pch() {
        let parser = parser_with_pch();
        assert!(parser.pch_map.len() == 1);
        assert!(parser.pch_map.get(&~"/bar/TestApplication-Prefix.pch.pch") == &~"/foo/TestApplication-Prefix.pch")
    }

    #[test]
    fn compile_c() {
        let output = r##"CompileC /Users/arthurevstifeev/Library/Developer/Xcode/DerivedData/TestApplication-ggnvtdtbeunuqqgbdwrstauepclk/Build/Intermediates/TestApplication.build/Debug-iphonesimulator/TestApplication.build/Objects-normal/i386/TestClass.o TestApplication/TestClass.m normal i386 objective-c com.apple.compilers.llvm.clang.1_0.compiler
    cd /Users/arthurevstifeev/github/xclang_tool/tests/TestApplication
    export LANG=en_US.US-ASCII
    export PATH="/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneSimulator.platform/Developer/usr/bin:/Applications/Xcode.app/Contents/Developer/usr/bin:/usr/local/bin:/usr/local/bin:/usr/local/sbin:/usr/bin:/bin:/usr/sbin:/sbin:/Users/arthurevstifeev/.gem/ruby/2.0.0/bin"
    /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/clang -x objective-c -arch i386 -include /bar/TestApplication-Prefix.pch.pch -c /foo/TestClass.m -o /baz/TestClass.o"##;

        let mut parser = parser_with_pch();
        let result = parser.parse_output(output);
        assert!(result.len() == 1);
        let command_data = result.get(0);
        assert!(command_data.command == ~"/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/clang -x objective-c -arch i386 -include /bar/TestApplication-Prefix.pch.pch -c /foo/TestClass.m -o /baz/TestClass.o")
        assert!(command_data.file == ~"/foo/TestClass.m")
        assert!(command_data.directory == ~"/Users/arthurevstifeev/github/xclang_tool/tests/TestApplication")
    }
}
