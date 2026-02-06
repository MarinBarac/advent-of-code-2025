use std::{fs, ops::Add};

fn main() {
    let file_path = "./input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut total: u64 = 0;

    let size = 12;

    contents.split("\n").for_each(|bank| {
        let mut pairs: Vec<(u64, i32)> = vec![];

        for i in 0..size {
            let mut starting_index = 0;
            let mut num = 0;
            let mut index = starting_index;

            if i != 0 {
                let tuple = pairs.get(i - 1).expect("Previous pair out of bound");
                starting_index = tuple.1 + 1;
            }

            for j in (starting_index as usize)..bank.len() - size + 1 + i {
                let current_num = bank.get(j..j + 1).expect("Current num out of bound");
                let current_num: u64 = current_num
                    .parse()
                    .expect("Current num is not valid number");

                if current_num > num {
                    num = current_num;
                    index = j as i32;
                }
            }

            pairs.push((num, index));
        }

        let num: u64 = pairs
            .into_iter()
            .map(|x| x.0.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse()
            .expect("Final number is not u64");

        println!("In row {bank} number is {num}");

        total += num;
    });

    println!("Total: {total}");
}
