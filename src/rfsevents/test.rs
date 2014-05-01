extern crate std;
extern crate rfsevents;

use rfsevents::{FSEventStream, FSEventStreamEventFlags, FSEventStreamEventId};
use std::os::getcwd;

fn callback(path: Path, flags: FSEventStreamEventFlags, id: FSEventStreamEventId) {
    println!("{}, {}, {}", path.as_str().unwrap(), flags as uint, id);
}

fn main() {

    let path = Path::new(getcwd());
    let stream = FSEventStream::new(&path, 3.0, callback);

    stream.schedule_stream();
}
