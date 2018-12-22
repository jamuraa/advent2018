fn print_recipes(scores: &Vec<u8>, first: &usize, second: &usize) {
    for (idx, i) in scores.iter().enumerate() {
        match idx {
            _ if &idx == first => print!("({})", i),
            _ if &idx == second => print!("[{}]", i),
            _ => print!(" {} ", i),
        }
    }
    println!();
}

fn next_ten_recipe_scores(recipe_count: usize) -> Vec<u8> {
    let mut recipe_scores: Vec<u8> = vec![3, 7];
    let mut first_elf_recipe_idx = 0;
    let mut second_elf_recipe_idx = 1;

    while recipe_scores.len() < (recipe_count + 10) {
        let first_recipe = recipe_scores[first_elf_recipe_idx];
        let second_recipe = recipe_scores[second_elf_recipe_idx];
        let new_recipes = first_recipe + second_recipe;
        if new_recipes >= 10 {
            recipe_scores.push(1);
        }
        recipe_scores.push(new_recipes % 10);
        first_elf_recipe_idx += first_recipe as usize + 1;
        first_elf_recipe_idx = first_elf_recipe_idx % recipe_scores.len();
        second_elf_recipe_idx += second_recipe as usize + 1;
        second_elf_recipe_idx = second_elf_recipe_idx % recipe_scores.len();
    }
    recipe_scores[recipe_count..recipe_count + 10].to_vec()
}

fn last_matching(seq: &[u8], last: &[u8]) -> usize {
    let mut matching = last.len();
    while matching > 0 {
        let idx = if matching > seq.len() {
            0
        } else {
            seq.len() - matching
        };
        let a = &seq[idx..];
        let b = &last[0..matching];
        if a == b {
            return matching;
        }
        matching -= 1;
    }
    matching
}

fn find_recipes_before_sequence(recipe_seq: &Vec<u8>) -> usize {
    let mut recipe_scores: Vec<u8> = vec![3, 7];
    let mut first_elf_recipe_idx = 0;
    let mut second_elf_recipe_idx = 1;

    let mut num_recipes: usize = 2;
    let mut num_recipes_matching = last_matching(&recipe_scores, &recipe_seq);
    loop {
        let first_recipe = recipe_scores[first_elf_recipe_idx];
        let second_recipe = recipe_scores[second_elf_recipe_idx];
        let new_recipes = first_recipe + second_recipe;
        if new_recipes >= 10 {
            num_recipes += 1;
            recipe_scores.push(1);
            if recipe_seq[num_recipes_matching] == 1 {
                num_recipes_matching += 1;
                if num_recipes_matching == recipe_seq.len() {
                    return num_recipes - num_recipes_matching;
                }
            } else if num_recipes_matching > 0 {
                num_recipes_matching = last_matching(&recipe_scores, &recipe_seq);
            }
        }
        let added = new_recipes % 10;
        num_recipes += 1;
        recipe_scores.push(added);
        if recipe_seq[num_recipes_matching] == added {
            num_recipes_matching += 1;
            if num_recipes_matching == recipe_seq.len() {
                return num_recipes - num_recipes_matching;
            }
        } else if num_recipes_matching > 0 {
            num_recipes_matching = last_matching(&recipe_scores, &recipe_seq);
        }

        first_elf_recipe_idx += first_recipe as usize + 1;
        first_elf_recipe_idx = first_elf_recipe_idx % recipe_scores.len();
        second_elf_recipe_idx += second_recipe as usize + 1;
        second_elf_recipe_idx = second_elf_recipe_idx % recipe_scores.len();
    }
}

fn main() {
    let input = 846021;
    let input_seq = vec![8, 4, 6, 0, 2, 1];
    println!(
        "The next ten recipe scores after recipe {} are {:?}",
        input,
        next_ten_recipe_scores(input)
    );
    println!(
        "There are {} recipes before the sequence {:?}",
        find_recipes_before_sequence(&input_seq),
        &input_seq
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_ten() {
        assert_eq!(
            vec![5, 1, 5, 8, 9, 1, 6, 7, 7, 9],
            next_ten_recipe_scores(9)
        );
        assert_eq!(
            vec![0, 1, 2, 4, 5, 1, 5, 8, 9, 1],
            next_ten_recipe_scores(5)
        );
        assert_eq!(
            vec![9, 2, 5, 1, 0, 7, 1, 0, 8, 5],
            next_ten_recipe_scores(18)
        );
        assert_eq!(
            vec![5, 9, 4, 1, 4, 2, 9, 8, 8, 2],
            next_ten_recipe_scores(2018)
        );
    }

    #[test]
    fn test_find_pattern() {
        assert_eq!(9, find_recipes_before_sequence(vec![5, 1, 5, 8, 9]));
        assert_eq!(5, find_recipes_before_sequence(vec![0, 1, 2, 4, 5]));
        assert_eq!(18, find_recipes_before_sequence(vec![9, 2, 5, 1, 0]));
        assert_eq!(2018, find_recipes_before_sequence(vec![5, 9, 4, 1, 4]));
    }

}
