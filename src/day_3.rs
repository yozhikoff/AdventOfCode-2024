pub mod solution {
    use regex::Regex;
    pub fn solve(resp: String) {
        let re = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))").unwrap();
        let mut sum = 0;

        let mut mul_enabled = true;

        for capture in re.captures_iter(&resp) {
            if capture.get(1).is_some() {
                if mul_enabled {
                    sum += capture[1].parse::<u64>().unwrap() * capture[2].parse::<u64>().unwrap()
                }
            } else if capture.get(3).is_some() {
                mul_enabled = true;
            } else {
                mul_enabled = false;
            }
        }
        println!("{}", sum);
    }
}
