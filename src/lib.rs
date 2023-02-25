pub mod cli;

use crate::cli::Environment;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::{Path, PathBuf};
use tera::Tera;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize, Debug)]
pub struct StitchConfig {
    /// Denotes the order that the files should be stitched in
    pub order: Vec<PathBuf>,
    /// The output file to write the stitched file to
    pub output_file: Option<PathBuf>,
}

#[derive(Serialize)]
pub struct StitchFile {
    pub path: PathBuf,
    pub raw_contents: String,
}

impl StitchFile {
    pub fn new(path: PathBuf, raw_contents: String) -> Self {
        Self { path, raw_contents }
    }

    /// Renders the file using the given environment
    fn render(&mut self, environment: Environment) -> Result<()> {
        let mut ctx = tera::Context::new();
        if environment == Environment::Overleaf {
            ctx.insert("environment", "overleaf");
        } else {
            ctx.insert("environment", "texit");
        }
        let _out = Tera::one_off(self.raw_contents.as_str(), &ctx, false);
        Ok(())
    }
}

pub fn get_config_from_fs(path: &Path) -> Result<StitchConfig> {
    let config = std::fs::read_to_string(path)?;
    let config: StitchConfig = serde_yaml::from_str(&config)?;
    Ok(config)
}

pub fn read_files(paths: &[PathBuf]) -> Result<Vec<StitchFile>> {
    let mut strings = Vec::new();
    for path in paths {
        let file_contents = std::fs::read_to_string(path)?;
        strings.push(StitchFile::new(path.clone(), file_contents));
    }
    Ok(strings)
}

/// Stitches the files in the given order
pub fn stitch_and_render(files: &[StitchFile], environment: Environment) -> Result<String> {
    let mut ctx = tera::Context::new();
    ctx.insert("preamble_stitch_version", VERSION);
    ctx.insert("files", files);
    ctx.insert::<String, &str>("environment", &environment.into());
    let template = String::from(r#"
% generated by preamble-stitch
% version: {{ preamble_stitch_version }}
% environment: {{ environment }}
% genrated at: {% now() %}
{% for file in files %}
% -PS- file: {{ file.path }} -PS-
{{ file.raw_contents }}
% -PS- end of file: {{ file.path }} -PS-
{% endfor %}
    "#);
    let out = Tera::one_off(template.as_str(), &ctx, false)?;
    Ok(out)
}

/// Writes a file to the filesystem at the given path
pub fn write_file(input: String, output_path: &Path) -> Result<()> {
    let mut file = std::fs::File::create(output_path)?;
    file.write_all(input.as_bytes())?;
    Ok(())
}
