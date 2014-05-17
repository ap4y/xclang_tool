use std::io::{fs, Process, Writer};
use std::str;

use serialize::{json, Encodable};

use xcodebuild_parser::{XCodeBuildParser, CommandData};
use rfsevents::*;

pub struct XCodeBuildWatcher {
    folder:    Path,
    workspace: ~str,
    scheme:    ~str,
    sdk:       ~str
}

impl XCodeBuildWatcher {
    pub fn new(folder: Path, workspace: ~str, scheme: ~str, sdk: ~str) -> XCodeBuildWatcher {
        XCodeBuildWatcher {
            folder:    folder,
            workspace: workspace,
            scheme:    scheme,
            sdk:       sdk
        }
    }

    pub fn watch(&self) -> Result<(), ~str> {
        try!(self.run());

        println!("Listening for changes in folder: {}", self.folder.as_str().unwrap());
        let callback = |path: Path, flags: FSEventStreamEventFlags, _id: FSEventStreamEventId| {
            if (flags as uint & kFSEventStreamEventFlagItemCreated as uint) == 0 { return }
            println!("Found new file: {}", path.as_str().unwrap());
            match self.run() { Ok(_) => (), Err(e) => println!("{}\x07", e) };
        };
        let stream = FSEventStream::new(&self.folder, 3.0, callback);
        stream.schedule_stream();

        return Ok(());
    }

    pub fn run(&self) -> Result<(), ~str> {
        info!("Building: {}.xcworkspace", self.workspace);
        let output_str = try!(self.build());
        debug!("{}", output_str);

        let mut parser = XCodeBuildParser::new();
        let compile_commands = parser.parse_output(output_str);

        info!("Writing compilation_db");
        try!(self.write_c_db_json(compile_commands));

        return Ok(());
    }

    fn build(&self) -> Result<~str, ~str> {
        let args = [~"-workspace", (self.workspace + ".xcworkspace"),
                    ~"-scheme", self.scheme.to_owned(),
                    ~"-sdk", self.sdk.to_owned(),
                    ~"clean", ~"build"];

        let process_output = match Process::output("xcodebuild", args) {
            Ok(output) => output,
            Err(e) => return Err(format!("Failed to execute process: {}", e)),
        };

        if !process_output.status.success() {
            let stderr = str::from_utf8_lossy(process_output.error.as_slice());
            return Err(format!("{}", stderr));
        }

        match str::from_utf8(process_output.output.as_slice()) {
            Some(output_str) => Ok(output_str.to_owned()),
            None => Ok(~"")
        }
    }

    fn write_c_db_json(&self, compile_commands: Vec<CommandData>) -> Result<(), ~str> {
        let encoding_result = match fs::File::create(&self.folder.join("compile_commands.json")) {
            Ok(f) => {
                let mut f2 = f;
                let encoder = &mut json::PrettyEncoder::new(&mut f2 as &mut Writer);
                compile_commands.encode(encoder)
            },
            Err(e) => return Err(format!("Unable to create compile_commands.json: {}", e))
        };

        match encoding_result {
            Ok(_) => return Ok(()),
            Err(e) => return Err(format!("Unable to encode database into json: {}", e))
        };
    }
}
