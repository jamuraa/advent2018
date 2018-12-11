use std::{
    collections::HashMap,
    fs::File,
    io::{self, prelude::*, BufReader},
};

/// If there's one letter which is different between `one` and `two`, then
/// returns Some(index).  Otherwise None.
fn edit_distance_replace_one(one: &str, two: &str) -> Option<usize> {
    if one.len() != two.len() {
        return None;
    }
    let one_chars = one.chars();
    let two_chars = two.chars();
    let mut together = one_chars.zip(two_chars);
    let idx = together.position(|(x, y)| x != y)?;
    if one[idx + 1..] != two[idx + 1..] {
        None
    } else {
        Some(idx)
    }
}

/// Returns whether the box contains exactly two of the same letter, or exactly three of the same
/// letter.
fn letter_counts(box_id: &str) -> (bool, bool) {
    let mut letters = HashMap::<char, i32>::new();
    for c in box_id.chars() {
        let letter_count = letters.entry(c).or_insert(0);
        *letter_count += 1;
    }
    let two = letters.values().any(|count| *count == 2);
    let three = letters.values().any(|count| *count == 3);
    (two, three)
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    // Read the input in.
    let box_ids: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    println!("{} box IDs", box_ids.len());

    let mut twice_count = 0;
    let mut thrice_count = 0;
    for box_id in box_ids.iter() {
        let (twice, thrice) = letter_counts(&box_id);
        if twice {
            twice_count = twice_count + 1;
        }
        if thrice {
            thrice_count = thrice_count + 1;
        }
    }
    println!("Box IDs with two of any letter: {}", twice_count);
    println!("Box IDs with three of any letter: {}", thrice_count);
    println!("Rudimentary checksum: {}", twice_count * thrice_count);

    let mut iter = box_ids.iter().peekable();

    loop {
        if let None = iter.peek() {
            break;
        }
        let this_id = iter.next().unwrap();
        // iter now contains every id that we haven't checked this_id against
        let rest = iter.clone();
        for id in rest {
            if let Some(idx) = edit_distance_replace_one(this_id, id) {
                println!(
                    "{} and {} only differ in the {}th letter.",
                    this_id, id, idx
                );
                let (before, after) = this_id.split_at(idx);
                let (_, after) = after.split_at(1);
                println!("They share {}{}", before, after);
                return Ok(());
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counts() {
        assert_eq!((false, false), letter_counts("abcdef"));
        assert_eq!((true, true), letter_counts("bababc"));
        assert_eq!((true, false), letter_counts("abbcde"));
        assert_eq!((false, true), letter_counts("abcccd"));
        assert_eq!((true, false), letter_counts("aabcdd"));
        assert_eq!((true, false), letter_counts("abcdee"));
        assert_eq!((false, true), letter_counts("ababab"));
    }

    #[test]
    fn test_replace_one() {
        assert_eq!(None, edit_distance_replace_one("abcde", "axcye"));
        assert_eq!(Some(2), edit_distance_replace_one("fghij", "fguij"));
        assert_eq!(None, edit_distance_replace_one("fghij", "fghij"));
    }
}
