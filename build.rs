use anyhow::{Result, Context};
use fs_extra::dir::CopyOptions;
use std::{
    env,
    path::Path,
    fs,
};

///this gets appended to the target folder
const OUTPUT_FOLDER: &str = "resources";
///The directory to copy assets from
const ASSETS_DIR: &str = "./assets";
///if true, existing assets in the target dir are overwritten, otherwise they are skipped
const OVERWRITE: bool = true;

fn main() -> Result<()> {
    //This is provided by cargo, but we need to move up in order to get to the correct folder
    let out_dir = env::var("OUT_DIR")?;
    let mut out_path = Path::new(&out_dir);

    //we call parent thrice here to get to the target/debug or target/release folder
    for _ in 0..3 {
        out_path = out_path.parent().context("failed to get parent directory")?;
    }
    
    //append output directory for assets
    let out_path_buf = out_path.join(OUTPUT_FOLDER);
    out_path = out_path_buf.as_path();

    //make some copy options
    let mut copy_options = CopyOptions::new();
    copy_options.skip_exist = !OVERWRITE;
    copy_options.overwrite = OVERWRITE;

    //create resources folder
    fs::create_dir_all(out_path).context("failed to create output directory")?;
    //this loop is required so the files dont end up in resources/assets :/
    for file in fs::read_dir(ASSETS_DIR)? {
        //use fs_extra crate to recursively copy directory
        fs_extra::copy_items(&[file?.path()], out_path, &copy_options).context("copy failed")?;
    }

    Ok(())
}
