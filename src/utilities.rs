use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufReader};
use std::path::Path;

use ggez::{filesystem, Context, GameResult};

pub fn load_messages(file_name: impl AsRef<Path>) -> Option<impl Iterator<Item = String>> {
    Some(
        BufReader::new(get_resource(file_name)?)
            .lines()
            .filter_map(|message| message.ok()),
    )
}

pub fn load_scores(file_name: &str, context: &mut Context) -> HashMap<String, u128> {
    if filesystem::exists(context, file_name) {
        todo!()
    } else {
        HashMap::new()
    }
}

pub fn save_scores(
    context: &mut Context,
    file_name: &str,
    scores: &HashMap<String, u128>,
) -> eyre::Result<()> {
    dbg!("about to open file for writing");
    let mut scores_file = filesystem::create(context, file_name)?;
    dbg!("about to write to file");
    match scores_file.write(b"some bytes") {
        Ok(_) => println!("wrote some bytes to the file"),
        Err(error) => eprintln!("{}", error),
    };
    dbg!("did it write?");
    Ok(())
}

/// opens a file from the resources folder
/// takes an `AsRef<Path>` for convenience. with this, a `&str` can be used
fn get_resource(path: impl AsRef<Path>) -> Option<File> {
    let mut path_buf = std::env::current_exe().ok()?.parent()?.to_path_buf();
    path_buf.push("resources");
    path_buf.push(path);
    File::open(path_buf).ok()
}

fn get_read_only_scores(file_name: &str) -> () {}
