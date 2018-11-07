//! This module contains the configuration of the application.
//!
//! All options are passed individually to each function and are not bundled together.
//!
//! # Examples
//!
//! ```no_run
//! # use machine_usable_words_generator::Options;
//! let options = Options::parse();
//! println!("Output root directory: {}", options.output_dir.0);
//! ```


use std::path::{PathBuf, Path};
use clap::{AppSettings, Arg};
use std::fs;


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    /// The root directory for the output.
    pub output_dir: (String, PathBuf),
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let matches = app_from_crate!("\n")
            .setting(AppSettings::ColoredHelp)
            .arg(Arg::from_usage("[OUT_DIR] 'Directory to output the wordsets into. Default: current directory'")
                .validator(Options::output_directory_validator))
            .get_matches();

        Options {
            output_dir: matches.value_of("OUT_DIR")
                .map(|o| {
                    ({
                         let mut out = o.to_string();
                         if !['/', '\\'].contains(&out.chars().last().unwrap()) {
                             out.push('/');
                         }
                         out
                     },
                     if let Some(fname) = Path::new(&o).file_name() {
                         let mut p = PathBuf::from(&o);
                         if !p.is_absolute() {
                             p = PathBuf::from(format!("./{}", o));
                         }
                         p.parent().unwrap().canonicalize().unwrap_or_else(|_| Path::new(".").canonicalize().unwrap()).join(fname)
                     } else {
                         Path::new(&o).canonicalize().unwrap_or_else(|_| PathBuf::from(o))
                     })
                })
                .unwrap_or_else(|| ("./".to_string(), PathBuf::from("."))),
        }
    }

    fn output_directory_validator(s: String) -> Result<(), String> {
        let mut p = PathBuf::from(&s);
        if !p.is_absolute() {
            p = PathBuf::from(format!("./{}", s));
        }
        if p.parent().is_some() {
            p.pop();
            fs::canonicalize(&p).map_err(|_| format!("Output directory's parent directory \"{}\" nonexistant", p.display())).and_then(|f| if !f.is_file() {
                Ok(())
            } else {
                Err(format!("Output directory's parent directory \"{}\" actually a file", p.display()))
            })
        } else {
            Ok(())
        }
    }
}
