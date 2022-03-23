use std::io::stdin;

pub fn main() {
    let mut return_date = String::new();
    let mut due_date = String::new();

    stdin().read_line(&mut return_date).unwrap();
    stdin().read_line(&mut due_date).unwrap();

    let return_date: Vec<&str> = return_date.split(" ").collect();
    let due_date: Vec<&str> = due_date.split(" ").collect();

    let (d1, m1, y1) = (
        parse_to_u16(return_date[0]),
        parse_to_u16(return_date[1]),
        parse_to_u16(return_date[2]),
    );
    let (d2, m2, y2) = (
        parse_to_u16(due_date[0]),
        parse_to_u16(due_date[1]),
        parse_to_u16(due_date[2]),
    );

    println!(
        "{}",
        if y1 < y2 || (y1 == y2 && ((m1 == m2 && d1 <= d2) || m1 < m2)) {
            String::from("0")
        } else if m1 == m2 && y1 == y2 {
            format!("{}", 15 * (d1 - d2))
        } else if y1 > y2 {
            String::from("10000")
        } else {
            let months1 = y1 * 12 + m1;
            let months2 = y2 * 12 + m2;
            let result: u16 = 500 * (months1 - months2);

            result.to_string()
        }
    );
}

fn parse_to_u16(text: &str) -> u16 {
    text.trim().parse::<u16>().unwrap()
}
