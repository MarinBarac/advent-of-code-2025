use std::fs;

fn main() {
    let file_path = "./input.txt";

    println!("In file {file_path}");
    let commands: [(isize, isize); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];
    let allowed_adjacents = 4;

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let contents: Vec<&str> = contents.split("\n").collect();
    let contents: Vec<Vec<&str>> = contents
        .into_iter()
        .map(|row| row.split("").collect())
        .collect();

    let result = contents.clone();
    let mut finds: Vec<(usize, usize)> = vec![];

    loop {
        let mut found = false;

        for i in 0..contents.len() {
            let row = contents.get(i).expect("Row out of bound");

            // println!("{:?}", row);

            for j in 0..row.len() {
                if finds.contains(&(i, j)) {
                    continue;
                }

                let mut adjacent_rolls = 0;

                let current_char = contents.get(i as usize).expect("Current row out of bound");
                let current_char = current_char
                    .get(j as usize)
                    .expect("Current column out of bound");

                // println!("Current char {:?}", current_char);

                if !current_char.contains("@") {
                    continue;
                }

                for command in commands {
                    let x = (i as isize) + (command.0);
                    let y = (j as isize) + (command.1);

                    if x < 0 || x > ((contents.len() - 1) as isize) {
                        continue;
                    }

                    if y < 0 {
                        continue;
                    }

                    if finds.contains(&((x as usize), (y as usize))) {
                        continue;
                    }

                    let char = contents.get(x as usize).expect("Adjacent row out of bound");

                    if y > (char.len() - 1) as isize {
                        continue;
                    }

                    let char = char.get(y as usize).expect("Adjacent column out of bound");

                    if char.contains("@") {
                        adjacent_rolls += 1;
                    }
                }

                if adjacent_rolls < allowed_adjacents {
                    finds.push((i, j));

                    found = true;
                }
            }
        }

        if !found {
            break;
        }
    }

    println!("FInds: {}", finds.len());
}
