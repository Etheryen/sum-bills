use color_eyre::{
    eyre::{Context, ContextCompat},
    Result,
};

const ALLOWED_SYMBOLS: [char; 3] = ['+', '-', '.'];

pub fn parse_line(line: &str) -> Result<f64> {
    let take_to = line
        .find(|c: char| !is_char_allowed(&c))
        .wrap_err(parse_error_message(line))?;

    line.chars()
        .take(take_to)
        .collect::<String>()
        .parse::<f64>()
        .wrap_err(parse_error_message(line))
}

pub fn get_currency(lines: Vec<&str>) -> Option<String> {
    let currency = lines
        .first()?
        .chars()
        .skip_while(is_char_allowed)
        .take_while(|c| !c.is_whitespace())
        .collect::<String>();

    Some(currency)
}

fn parse_error_message(line: &str) -> String {
    format!("Couldn't find a number on line: {}", line)
}

fn is_char_allowed(c: &char) -> bool {
    ALLOWED_SYMBOLS.contains(c) || c.is_ascii_digit()
}
