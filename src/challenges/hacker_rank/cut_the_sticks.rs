use std::i32::MAX;
use std::io::stdin;

pub fn main() {
    let mut size_of_arr = String::new();

    stdin().read_line(&mut size_of_arr).unwrap();

    let mut arr_pieces = String::new();

    stdin().read_line(&mut arr_pieces).unwrap();

    let arr_pieces: Vec<&str> = arr_pieces.split(" ").collect();
    let mut arr_pieces: Vec<i32> = convert_arr_str_to_i32(arr_pieces);

    while arr_pieces.len() > 0 {
        let (cuts, temp) = cut_sticks(&mut arr_pieces);
        println!("{}", cuts);
        arr_pieces = temp;
    }
}

fn convert_arr_str_to_i32(arr: Vec<&str>) -> Vec<i32> {
    let mut temp: Vec<i32> = Vec::new();

    for unit in arr {
        let n: i32 = unit.trim().parse().unwrap();
        temp.push(n);
    }

    temp
}

fn cut_sticks(arr: &mut Vec<i32>) -> (i32, Vec<i32>) {
    let min_value = get_min_value(arr);
    let mut cuts = 0;
    let mut temp: Vec<i32> = Vec::new();

    for index in 0..arr.len() {
        let result = arr[index] - min_value;

        if !(result <= 0) {
            temp.push(result);
        }

        cuts += 1;
    }

    (cuts, temp)
}

fn get_min_value(arr: &Vec<i32>) -> i32 {
    let mut min = MAX;

    for number in arr {
        if *number < min {
            min = *number;
        }
    }

    min
}
