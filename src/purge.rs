use std::time::SystemTime;
use std::{fs::File, fs::FileTimes};

pub fn time(file: &mut File) {
    let clean_time = SystemTime::UNIX_EPOCH;
    let times = FileTimes::new()
        .set_accessed(clean_time)
        .set_modified(clean_time);
    // TODO: check for OS specific creation time purge

    file.set_times(times).expect("Error updating file times");
}
