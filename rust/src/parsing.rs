use color_eyre::{
    eyre::{Context, ContextCompat},
    Result,
};

const ALLOWED_SYMBOLS: [char; 3] = ['+', '-', '.'];

pub fn parse_line(line: &str) -> Result<f64> {
    let take_to = line
        .find(|c: char| !ALLOWED_SYMBOLS.contains(&c) && c.to_digit(10).is_none())
        .wrap_err_with(|| format!("Couldn't find a number on line: {}", line))?;

    line.chars()
        .take(take_to)
        .collect::<String>()
        .parse::<f64>()
        .wrap_err_with(|| format!("Couldn't parse number on line: {}", line))
}
