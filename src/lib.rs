use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct StitchConfig {
    /// Denotes the order that the files should be stitched in
    pub order: Vec<PathBuf>,
    /// The output file to write the stitched file to
    pub output_file: Option<PathBuf>,
}

pub struct StitchFile {
    path: PathBuf,
    contents: String,
}

pub fn get_config_from_fs(path: &Path) -> Result<StitchConfig> {
    let config = std::fs::read_to_string(path)?;
    let config: StitchConfig = serde_yaml::from_str(&config)?;
    Ok(config)
}

pub fn read_files(paths: &[PathBuf]) -> Result<Vec<StitchFile>> {
    let mut strings = Vec::new();
    for path in paths {
        let string = std::fs::read_to_string(path)?;
        strings.push(StitchFile {
            path: path.clone(),
            contents: string,
        });
    }
    Ok(strings)
}

/// Stitches the files in the given order
pub fn stitch_files(files: &[StitchFile]) -> String {
    let mut output = String::new();
    for file in files {
        output.push_str(&format!("% -- file: {} -- \n", file.path.display()));
        output.push_str(&file.contents);
        output.push_str(&format!("% -- end file: {} --\n", file.path.display()));
    }
    output
}

/// Writes a file to the filesystem at the given path
pub fn write_file(input: String, output_path: &Path) -> Result<()> {
    let mut file = std::fs::File::create(output_path)?;
    file.write_all(input.as_bytes())?;
    Ok(())
}
