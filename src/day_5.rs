pub mod solution {

    use std::collections::HashSet;

    pub fn parse(resp: &String) -> (Vec<HashSet<usize>>, Vec<HashSet<usize>>, Vec<Vec<usize>>) {
        let mut rules = vec![HashSet::new(); 100];
        let mut rules_reversed = vec![HashSet::new(); 100];
        let mut updates = Vec::new();

        let (rules_string, updates_string) = resp.split_once("\n\n").unwrap();

        for rule in rules_string.split("\n") {
            let parts: Vec<usize> = rule
                .split('|')
                .map(|s| s.trim().parse::<usize>().unwrap())
                .collect();
            rules[parts[0]].insert(parts[1]);
            rules_reversed[parts[1]].insert(parts[0]);
        }

        for rule in updates_string.trim().split("\n") {
            let parts: Vec<usize> = rule
                .trim()
                .split(',')
                .map(|s| s.trim().parse::<usize>().unwrap())
                .collect();
            updates.push(parts);
        }
        (rules, rules_reversed, updates)
    }

    pub fn get_middle_number(updates: &Vec<usize>) -> usize {
        updates[updates.len() / 2]
    }

    pub fn solve_part_one(
        rules: &Vec<HashSet<usize>>,
        updates: &Vec<Vec<usize>>,
    ) -> Vec<Vec<usize>> {
        let mut middle_sum = 0;
        let mut incorrect_updates = Vec::new();

        for update in updates {
            let mut prefix = HashSet::new();
            let mut had_intersection = false;

            for number in update.iter() {
                if !rules[*number]
                    .intersection(&prefix)
                    .collect::<Vec<_>>()
                    .is_empty()
                {
                    had_intersection = true;
                    incorrect_updates.push(update.clone());
                    break;
                }
                prefix.insert(*number);
            }

            if !had_intersection {
                middle_sum += get_middle_number(&update);
            }
        }
        println!("Part one: {}", middle_sum);
        incorrect_updates
    }

    pub fn solve_part_two(
        rules: &Vec<HashSet<usize>>,
        rules_reversed: &Vec<HashSet<usize>>,
        updates: &Vec<Vec<usize>>,
    ) {
        let mut middle_sum = 0;

        for update in updates {
            let mut sorted_update = Vec::new();
            let mut S: HashSet<usize> = HashSet::new();
            let current_update_set: HashSet<usize> = update.iter().cloned().collect();
            for page in &current_update_set {
                if rules_reversed[*page]
                    .intersection(&current_update_set)
                    .collect::<Vec<_>>()
                    .is_empty()
                {
                    S.insert(*page);
                }
            }

            let mut rules_cur = rules.clone();
            let mut rules_reversed_cur = rules_reversed.clone();

            while !S.is_empty() {
                let page_out = *S.iter().next().unwrap();
                S.remove(&page_out);
                sorted_update.push(page_out);
                for page_in in rules_cur[page_out].clone().intersection(&current_update_set){
                    rules_reversed_cur[*page_in].remove(&page_out);
                    rules_cur[page_out].remove(&page_in);
                    if rules_reversed_cur[*page_in]
                        .intersection(&current_update_set)
                        .collect::<Vec<_>>()
                        .is_empty()
                    {
                        S.insert(*page_in);
                    }
                }
            }
            // println!("{:?}\n{:?}\n\n", update, sorted_update);
            middle_sum += get_middle_number(&sorted_update);
        }
        println!("Part two: {}", middle_sum);
    }

    pub fn solve(resp: String) {
        //         let resp = String::from("47|53
        // 97|13
        // 97|61
        // 97|47
        // 75|29
        // 61|13
        // 75|53
        // 29|13
        // 97|29
        // 53|29
        // 61|53
        // 97|53
        // 61|29
        // 47|13
        // 75|47
        // 97|75
        // 47|61
        // 75|61
        // 47|29
        // 75|13
        // 53|13

        // 75,47,61,53,29
        // 97,61,53,29,13
        // 75,29,13
        // 75,97,47,61,53
        // 61,13,29
        // 97,13,75,29,47");

        let (rules, rules_reversed, updates) = parse(&resp);
        let incorrect_updates = solve_part_one(&rules, &updates);
        solve_part_two(&rules, &rules_reversed, &incorrect_updates);
    }
}
