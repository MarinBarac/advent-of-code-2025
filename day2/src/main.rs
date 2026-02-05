use itertools::Itertools;
use std::fs;

fn main() {
    let file_path = "./input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut total = 0;

    // first part
    // contents.trim().split(",").for_each(|sequence| {
    //     match sequence.split("-").collect::<Vec<&str>>().as_slice() {
    //         [start, end] => {
    //             let start_num: u64 = start
    //                 .parse()
    //                 .expect("Start of interval must be valid number");

    //             let end_num: u64 = end.parse().expect("End of interval must be valid number");

    //             for n in start_num..=end_num {
    //                 let n_string = n.to_string();

    //                 let first_string = n_string
    //                     .get(..n_string.len() / 2)
    //                     .expect("First substring out of bound");
    //                 let second_string = n_string
    //                     .get(n_string.len() / 2..)
    //                     .expect("Second substring out of bound");

    //                 if first_string.contains(second_string) {
    //                     total += n;
    //                 }
    //             }
    //         }
    //         _ => {
    //             println!("Invalid interval pattern");
    //         }
    //     }
    // });

    //second part

    contents.trim().split(",").for_each(|sequence| {
        match sequence.split("-").collect::<Vec<&str>>().as_slice() {
            [start, end] => {
                let start_num: u64 = start
                    .parse()
                    .expect("Start of interval must be valid number");

                let end_num: u64 = end.parse().expect("End of interval must be valid number");

                for n in start_num..=end_num {
                    let n_string = n.to_string();
                    let mut found_match = false;

                    for chunk_size in 1..=n_string.len() / 2 {
                        if (n_string.len() % chunk_size) != 0 {
                            continue;
                        }

                        let substring = n_string.get(..chunk_size).expect("Substring out of bound");
                        let mut found_mismatch = false;

                        for chunk in n_string.chars().chunks(chunk_size).into_iter() {
                            let string_to_compare = String::from_iter(chunk);

                            if string_to_compare != substring {
                                found_mismatch = true;
                                break;
                            }
                        }

                        if !found_mismatch {
                            found_match = true;
                            break;
                        }
                    }

                    if found_match {
                        total += n;
                    }
                }
            }
            _ => {
                println!("Invalid interval pattern");
            }
        }
    });

    println!("Total {total}");
}
