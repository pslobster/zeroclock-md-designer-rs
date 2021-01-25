#![warn(rust_2018_idioms)]

use std::fs;

use anyhow::Result;
use clap::{crate_authors, crate_description, crate_name, crate_version, App as ClapApp, Arg};

use md_designer::app::App;
use md_designer::rule::Rule;

fn main() -> Result<()> {
    // setup clap
    let clap = ClapApp::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("path")
                .required(true)
                .help("input file path (.md)"),
        )
        .arg(
            Arg::with_name("conf_path")
                .required(true)
                .help("config file path (.yml)"),
        )
        .get_matches();

    // check user input
    let input_text = if let Some(path) = clap.value_of("path") {
        fs::read_to_string(path)?
    } else {
        "".to_string()
    };

    let cfg_text = if let Some(path) = clap.value_of("conf_path") {
        fs::read_to_string(path)?
    } else {
        "".to_string()
    };

    let rule = Rule::marshal(&cfg_text)?;

    let app = App::new(&input_text, rule)?;

    app.export_excel()?;

    Ok(())
}
