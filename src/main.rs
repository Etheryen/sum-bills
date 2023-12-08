const ALLOWED_SYMBOLS: [char; 3] = ['+', '-', '.'];

fn parse_line(line: &str) -> f64 {
    let take_to = line
        .find(|c: char| !ALLOWED_SYMBOLS.contains(&c) && c.to_digit(10).is_none())
        .expect(&format!("Couldn't find a number on line: {}", line));

    line.chars()
        .take(take_to)
        .collect::<String>()
        .parse()
        .expect(&format!("Couldn't parse number on line: {}", line))
}

fn main() {
    let args_error_message = "Usage: `cargo run -- STRING FILE`
STRING: line after which to sum all bills
FILE: has a section after STRING with lines that begin with numbers";

    let args = std::env::args().collect::<Vec<_>>();
    let start_string = args.get(1).expect(args_error_message);
    let file_name = args.get(2).expect(args_error_message);

    let file = std::fs::read_to_string(&file_name).expect("Should be able to read file");

    let inputs = file
        .split('\n')
        .skip_while(|&line| line != start_string)
        .skip(1)
        .filter(|&line| !line.is_empty())
        .collect::<Vec<&str>>();

    let sum = inputs.into_iter().map(parse_line).sum::<f64>();
    println!("Sum is: {}", sum);
}
