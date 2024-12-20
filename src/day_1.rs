pub mod solution {
    use std::collections::HashMap;

    pub fn solve(resp: String) {
        let mut first_list = Vec::new();
        let mut second_list = Vec::new();
    
        for part in resp.trim().split('\n') {
            let mut numbers = part.split("   ");
            first_list.push(numbers.next().unwrap().parse::<i32>().unwrap());
            second_list.push(numbers.next().unwrap().parse::<i32>().unwrap());
    
        }
    
        first_list.sort();
        second_list.sort();
    
        let mut distance = 0;
        let mut counts = HashMap::new();
    
        let it = first_list.iter().zip(second_list.iter());
    
        for (_i, (x, y)) in it.enumerate() {
            distance += (x - y).abs();
            *counts.entry(y).or_insert(0) += 1;
        }
        println!("Distance is {}", distance);
    
        let mut similarity = 0;
    
        for x in first_list {
            similarity += x * counts.get(&x).cloned().unwrap_or(0);
        }
    
        println!("Distance is {}", similarity);
    }
}
