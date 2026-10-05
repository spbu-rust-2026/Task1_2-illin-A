use std::io;
fn main() {
    let stdin = io::stdin();
    let mut total_sum: u128 = 0;
    for line in stdin.lines() {
        let current_line = line.ok().unwrap_or_default();
        let token = current_line.trim();
        if token.is_empty() {
            continue;
        }
        if token == "-1" {
            println!("{}", total_sum);
            return;
        }
        if let Ok(num) = token.parse::<u128>() {
            if num > 0 {
                total_sum += num;
                continue;
            }
        }
        println!("NaN");
        return;
    }
    println!("{}", total_sum);
}
