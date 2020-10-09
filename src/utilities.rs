use std::io::{prelude::*, BufReader, Lines};
use std::{fs::File, path::Path};

pub fn load_messages(file_name: impl AsRef<Path>) -> Option<impl Iterator<Item = String>> {
    Some(
        BufReader::new(get_resource(file_name)?)
            .lines()
            .filter_map(|message| message.ok()),
    )
}

/// opens a file from the resources folder
/// takes an `AsRef<Path>` for convenience. with this, a `&str` can be used
fn get_resource(path: impl AsRef<Path>) -> Option<File> {
    let mut path_buf = std::env::current_exe().ok()?.parent()?.to_path_buf();
    path_buf.push("resources");
    path_buf.push(path);
    File::open(path_buf).ok()
}
