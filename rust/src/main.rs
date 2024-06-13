use crate::parsing::parse_line;
use color_eyre::eyre::{eyre, ContextCompat, Result};
use itertools::Itertools;
use parsing::get_currency;

mod parsing;

const ARGS_ERROR_MESSAGE: &str = "Usage: `cargo run -- HEADER FILE`
HEADER: line after which to sum all bills
FILE: has a section after HEADER with lines that begin with numbers";

fn main() -> Result<()> {
    let args = std::env::args().collect_vec();
    let header = args.get(1).wrap_err(ARGS_ERROR_MESSAGE)?;
    let file_path = args.get(2).wrap_err(ARGS_ERROR_MESSAGE)?;

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

    let sum = lines
        .iter()
        .map(|&line| parse_line(line))
        .sum::<Result<f64>>()?;
    let currency = get_currency(lines).unwrap_or(String::from(""));

    println!("Sum is {}{}", sum, currency);

    Ok(())
}
