pub mod solution {

    pub fn parse_string_to_vectors(resp: String) -> Vec<Vec<i32>> {
        let mut out_vector = Vec::new();
        for row in resp.trim().split('\n') {
            let new_vec: Vec<i32> = row.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
            out_vector.push(new_vec);
        }
        out_vector
    }

    pub fn check_path_part_one(numbers: Vec<i32>) -> bool {
        // println!("{:?}", numbers);
        let mut current_number = numbers[0];
        let mut n_increasing = 0;
        let mut n_decreasing = 0;
        let mut has_invalid_steps = false;
        for new_number in numbers.iter().skip(1) {
            let difference = new_number - current_number;
            if difference.abs() < 1 || difference.abs() > 3 {
                has_invalid_steps = true;
                break;
            }
            if difference > 0 {
                n_increasing += 1;
            } else {
                n_decreasing += 1;
            }
            current_number = *new_number;
        }
        !has_invalid_steps && (n_increasing == 0 || n_decreasing == 0)
    }

    pub fn check_mostly_increasing(numbers: Vec<i32>) -> bool {
        let mut n_increasing = 0;
        let mut current_number = numbers[0];
        for new_number in numbers.iter().take(10).skip(1) {
            if current_number < *new_number {
                n_increasing += 1;
                current_number = *new_number;
            }
        }
        n_increasing > 1
    }

    pub fn check_path_part_two(numbers: Vec<i32>) -> bool {
        let mut current_number = numbers[0];
        let is_mostly_increasing = check_mostly_increasing(numbers.clone());

        for idx in 1..numbers.len() {
            let new_number = numbers[idx];
            let difference = new_number - current_number;
            if difference.abs() < 1
                || difference.abs() > 3
                || (difference > 0) != is_mostly_increasing
            {
                return check_path_part_one(
                    numbers
                        .iter()
                        .enumerate()
                        .filter(|&(i, _)| i != idx)
                        .map(|(_, v)| *v)
                        .collect(),
                ) || check_path_part_one(
                    numbers
                        .iter()
                        .enumerate()
                        .filter(|&(i, _)| i != idx - 1)
                        .map(|(_, v)| *v)
                        .collect(),
                );
            }
            current_number = new_number;
        }
        true
    }

    pub fn solve_first(resp: String) {
        let mut safe_counter = 0;
        let data = parse_string_to_vectors(resp);

        for numbers in data {
            if check_path_part_one(numbers) {
                safe_counter += 1;
            }
        }

        println!("Safe paths count: {}", safe_counter);
    }

    pub fn solve_second(resp: String) {
        let mut safe_counter = 0;
        let data = parse_string_to_vectors(resp);

        for numbers in data {
            let ans = check_path_part_two(numbers.clone());
            // println!("{:?} {}\n", numbers, ans);
            if ans {
                safe_counter += 1;
            }
        }

        println!("Safe paths count: {}", safe_counter);
    }

    pub fn solve(resp: String) {
        solve_first(resp.clone());
        solve_second(resp.clone());
    }
}