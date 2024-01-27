use crate::{config::get_config, parsing::parse_line};
use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;

mod config;
mod parsing;

fn main() -> Result<()> {
    let config = get_config()?;
    let header = config.get_string("header")?;
    let file_path = config.get_string("file")?;

    let file = std::fs::read_to_string(file_path)?;

    let lines = file
        .lines()
        .skip_while(|&line| line != header)
        .skip(1)
        .filter(|&line| !line.is_empty())
        .collect_vec();

    if lines.is_empty() {
        return Err(eyre!("No bills found"));
    }

    let sum = lines.into_iter().map(parse_line).sum::<Result<f64>>()?;

    println!("Sum is: {}", sum);

    Ok(())
}
