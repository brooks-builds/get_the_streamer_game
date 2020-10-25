use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

use ggez::{filesystem, Context};

pub fn load_messages(file_name: impl AsRef<Path>) -> Option<impl Iterator<Item = String>> {
    Some(
        BufReader::new(get_resource(file_name)?)
            .lines()
            .filter_map(|message| message.ok()),
    )
}

pub fn load_scores(file_name: &str, context: &mut Context) -> HashMap<String, u128> {
    if filesystem::exists(context, file_name) {
        match filesystem::open(context, file_name) {
            Ok(high_scores_file) => deserialize_high_scores(high_scores_file),
            Err(_) => HashMap::new(),
        }
    } else {
        HashMap::new()
    }
}

pub fn save_scores(
    context: &mut Context,
    file_name: &str,
    scores: &HashMap<String, u128>,
) -> eyre::Result<()> {
    let mut scores_file = filesystem::create(context, file_name)?;
    let mut serialized_scores = String::new();
    serialize_hashmap(scores, &mut serialized_scores);
    if let Err(error) = scores_file.write(serialized_scores.as_bytes()) {
        eprintln!("{}", error);
    };
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

fn serialize_hashmap(hashmap: &HashMap<String, u128>, output: &mut String) {
    for (username, points) in hashmap {
        let points = format!("{}\r\n", points);
        output.push_str(username);
        output.push(':');
        output.push_str(&points);
    }
}

fn deserialize_high_scores(file: filesystem::File) -> HashMap<String, u128> {
    let mut scores = HashMap::new();
    for high_score_line in BufReader::new(file).lines() {
        if let Ok(raw_high_score) = high_score_line {
            let high_score: Vec<&str> = raw_high_score.trim().split(':').collect();
            let score: u128 = high_score[1].parse().unwrap_or_else(|_| 0);
            let username = high_score[0].to_owned();
            scores.insert(username, score);
        }
    }
    scores
}
