use std::io::Write;

pub(crate) fn read_line() -> String {
    let mut buffer = String::new();
    std::io::stdout().flush().expect("Failed to flush stdout");
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read input from stdin");
    buffer
}
