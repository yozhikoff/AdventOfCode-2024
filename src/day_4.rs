pub mod solution {

    pub fn get_horizontal_indices(width: usize, height: usize) -> Vec<Vec<(usize, usize)>> {
        let mut indices = Vec::new();
        for i in 0..height {
            let mut seq = Vec::new();
            for j in 0..width {
                seq.push((i, j));
            }
            indices.push(seq);
        }
        indices
    }

    pub fn get_vertical_indices(width: usize, height: usize) -> Vec<Vec<(usize, usize)>> {
        let mut indices = Vec::new();
        for j in 0..width {
            let mut seq = Vec::new();
            for i in 0..height {
                seq.push((i, j));
            }
            indices.push(seq);
        }
        indices
    }

    pub fn get_diag_indices_1(width: usize, height: usize) -> Vec<Vec<(usize, usize)>> {
        let mut indices = Vec::new();
        for mut i in 0..height - 1 {
            let mut seq = Vec::new();
            let mut j: usize = 0;
            while i < height && j < width {
                seq.push((i, j));
                if i == 0 {
                    break;
                }
                i -= 1;
                j += 1;
            }
            indices.push(seq);
        }

        for mut j in 0..width {
            let mut seq = Vec::new();
            let mut i = height - 1;
            while i < height && j < width {
                seq.push((i, j));
                if i == 0 {
                    break;
                }
                i -= 1;
                j += 1;
            }
            indices.push(seq);
        }
        indices
    }

    pub fn get_diag_indices_2(width: usize, height: usize) -> Vec<Vec<(usize, usize)>> {
        let mut indices = Vec::new();
        for mut i in 1..height {
            let mut seq = Vec::new();
            let mut j = 0;
            while i < height && j < width {
                seq.push((i, j));
                i += 1;
                j += 1;
            }
            indices.push(seq);
        }

        for mut j in 0..width {
            let mut seq = Vec::new();
            let mut i = 0;
            while i < height && j < width {
                seq.push((i, j));
                i += 1;
                j += 1;
            }
            indices.push(seq);
        }
        indices
    }

    pub fn process_seq(seq: Vec<(usize, usize)>, data: &Vec<Vec<char>>) -> usize {
        let mut n = 0;

        let mut forward = 0; // X M A S
        let mut reverse = 0; // S A M X

        for (i, j) in seq {
            match data[i][j] {
                'X' => {
                    forward = 1;
                    if reverse == 3 {
                        n += 1;
                    }
                    reverse = 0;
                }
                'M' => {
                    forward = if forward == 1 { 2 } else { 0 };
                    reverse = if reverse == 2 { 3 } else { 0 };
                }
                'A' => {
                    forward = if forward == 2 { 3 } else { 0 };
                    reverse = if reverse == 1 { 2 } else { 0 };
                }
                'S' => {
                    if forward == 3 {
                        n += 1;
                    }
                    forward = 0;
                    reverse = 1;
                }
                _ => {}
            }
        }
        n
    }

    pub fn solve_part_2(data: &Vec<Vec<char>>) -> usize {
        if data.len() < 3 || data[0].len() < 3 {
            return 0;
        }
        let xmas = [['M', 'A', 'S'], ['S', 'A', 'M']];
        let mut n = 0;
        for i in 1..data.len() - 1 {
            for j in 1..data[0].len() - 1 {
                if xmas.contains(&[data[i - 1][j - 1], data[i][j], data[i + 1][j + 1]])
                    && xmas.contains(&[data[i - 1][j + 1], data[i][j], data[i + 1][j - 1]])
                {
                    n += 1;
                }
            }
        }
        n
    }

    pub fn solve(resp: String) {
        //         let resp = String::from(
        // "
        // MMMSXXMASM
        // MSAMXMSMSA
        // AMXSXMAAMM
        // MSAMASMSMX
        // XMASAMXAMM
        // XXAMMXXAMA
        // SMSMSASXSS
        // SAXAMASAAA
        // MAMMMXMMMM
        // MXMXAXMASX

        // ",
        // );

        let mut n = 0;

        let data: Vec<_> = resp
            .trim()
            .split('\n')
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect();
        for f in [
            get_horizontal_indices,
            get_vertical_indices,
            get_diag_indices_1,
            get_diag_indices_2,
        ] {
            for seq in f(data[0].len(), data.len()) {
                n += process_seq(seq, &data);
            }
        }
        println!("Part 1: {}", n);
        println!("Part 2: {}", solve_part_2(&data));
    }
}
