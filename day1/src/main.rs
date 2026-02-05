use std::fs;

fn main() {
    let file_path = "./input.txt";

    println!("In file {file_path}");

    let max = 99;
    let min = 0;
    let mut password = 0;
    let mut current: i32 = 50;
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    contents.split("\n").for_each(|x| {
        let num: i32 = x
            .get(1..)
            .expect("Should be string that contains direction and number")
            .parse()
            .expect("Direction should be only 1 character, then number comes");

        let multiplier = num / 100;
        println!("Multiplier: {multiplier}");
        password += multiplier;

        let num = num - (multiplier * 100);
        let current_before = current;

        println!("Num after mod: {num}");

        if x.starts_with("L") {
            current = current - num;
        } else if x.starts_with("R") {
            current = current + num;
        };

        println!("Current before: {current}");

        if current > max {
            current = current - max - 1;

            if current != 0 && current_before != 0 {
                println!("Current is different from 0: {current}");
                password += 1;
            }
        } else if current < min {
            current = current + max + 1;

            if current != 0 && current_before != 0 {
                println!("Current is different from 0: {current}");
                password += 1;
            }
        }
        println!("Current after: {current}");

        if current == 0 && current_before != 0 {
            password += 1;
        }

        println!("Password {password}");
    });

    println!("Final password is: {password}")
}
