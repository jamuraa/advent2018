use std::{
    collections::HashMap,
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn react(chars: &mut Vec<char>) {
    let mut i = 0;
    while i != chars.len() - 1 {
        let x = chars[i];
        let y = chars[i + 1];
        if x.eq_ignore_ascii_case(&y) && (x.is_lowercase() != y.is_lowercase()) {
            chars.remove(i);
            chars.remove(i);
            if chars.len() == 0 {
                return;
            }
            if i != 0 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }
}

fn without_polymer(chars: &Vec<char>, rem: &char) -> Vec<char> {
    chars
        .iter()
        .filter(|x| !x.eq_ignore_ascii_case(rem))
        .map(|x| x.clone())
        .collect()
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut lines = reader.lines();
    let chars: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
    let mut original = chars.clone();

    react(&mut original);
    println!("Reacting original: final length {}", original.len());

    let mut smallest_char = 'a';
    let mut smallest_len = chars.len();

    for c in 'a' as u8..'{' as u8 {
        let mut new = without_polymer(&chars, &(c as char));
        react(&mut new);
        println!("Reacted w/o {}: length {}", c as char, new.len());
        if new.len() < smallest_len {
            smallest_char = c as char;
            smallest_len = new.len();
        }
    }

    println!(
        "Strongest Reaction is w/o {}: length {}",
        smallest_char, smallest_len
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_react() {
        let mut a = "AcCb".chars().collect();
        let a2: Vec<char> = "Ab".chars().collect();
        react(&mut a);
        assert_eq!(a2, a);
        let mut c = "dabAcCaCBAcCcaDA".chars().collect();
        let c4: Vec<char> = "dabCBAcaDA".chars().collect();
        react(&mut c);
        assert_eq!(c4, c);
        react(&mut c);
        assert_eq!(c4, c);
    }

    #[test]
    fn test_without() {
        let a: Vec<char> = "dabAcCaCBAcCcaDA".chars().collect();
        let a2: Vec<char> = "dbcCCBcCcD".chars().collect();
        assert_eq!(a2, without_polymer(&a, &'a'));
        let a3: Vec<char> = "daAcCaCAcCcaDA".chars().collect();
        assert_eq!(a3, without_polymer(&a, &'b'));
        let a4: Vec<char> = "dabAaBAaDA".chars().collect();
        assert_eq!(a4, without_polymer(&a, &'c'));
        let a5: Vec<char> = "abAcCaCBAcCcaA".chars().collect();
        assert_eq!(a5, without_polymer(&a, &'d'));
    }

}
