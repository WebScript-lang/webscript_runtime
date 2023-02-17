use std::{path::PathBuf, str::FromStr};

use anyhow::{anyhow, bail, Context, Result};
use url::Url;

pub struct Loader {}

impl Loader {
    pub fn new() -> Self {
        Self {}
    }

    pub fn load_module(&self, url: &str) -> Result<String> {
        // Check if the url is a path.
        let path = PathBuf::from_str(url);
        if path.is_ok() {
            return self.load_path(path.unwrap());
        }

        // Check if the url is a valid url.
        let url = match Url::parse(url) {
            Ok(url) => url,
            Err(err) => bail!(anyhow!(err).context(format!("Invalid url: {}", url))),
        };

        // Load the module from the url.
        match url.scheme() {
            "file" => {
                let path = url
                    .to_file_path()
                    .expect(format!("Invalid path: {}", url).as_str());

                self.load_path(path)
            }
            _ => {
                bail!("Unsupported scheme: {}", url.scheme());
            }
        }
    }

    fn load_path(&self, path: PathBuf) -> Result<String> {
        match std::fs::read_to_string(path.clone()) {
            Ok(code) => Ok(code),
            Err(err) => {
                let path = if path.is_absolute() {
                    path
                } else {
                    std::env::current_dir()
                        .with_context(|| format!("Failed to open a file \"{}\"", path.display()))?
                        .join(&path)
                };

                Err(err).context(format!("Failed to open a file \"{}\"", path.display()))
            }
        }
    }
}
