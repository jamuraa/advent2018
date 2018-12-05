#![feature(test)]

extern crate test;

use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn pair(a: u8, b: u8) -> bool {
    if a < 97 {
        a + 32 == b
    } else {
        a - 32 == b
    }
}

fn caseless_match(a: u8, b: u8) -> bool {
    match (a < 97, b < 97) {
        (true, true) => a == b,
        (false, true) => a == b + 32,
        (true, false) => a + 32 == b,
        (false, false) => a == b,
    }
}

fn react(chars: &mut String, without: Option<u8>) {
    let mut i = 0;
    while i != chars.len() - 1 {
        let x = chars.as_bytes()[i];
        if without.is_some() && caseless_match(x, without.unwrap()) {
            chars.remove(i);
            if i != 0 {
                i -= 1;
            }
            continue;
        }
        let y = chars.as_bytes()[i + 1];
        if pair(x, y) {
            chars.replace_range(i..i + 2, "");
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
    let x = chars.as_bytes()[i];
    if without.is_some() && caseless_match(x, without.unwrap()) {
        chars.remove(i);
    }
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut lines = reader.lines();
    let chars: String = lines.next().unwrap().unwrap();
    let mut original = chars.clone();

    react(&mut original, None);
    println!("Reacting original: final length {}", original.len());

    let mut smallest_char = 'a';
    let mut smallest_len = chars.len();

    for c in 'a' as u8..'{' as u8 {
        let mut new = original.clone();
        react(&mut new, Some(c));
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
    use test::Bencher;

    #[test]
    fn test_react() {
        let mut a = "AcCb".to_string();
        let a2 = "Ab";
        react(&mut a, None);
        assert_eq!(a2, a);
        let mut c = "dabAcCaCBAcCcaDA".to_string();
        let c4 = "dabCBAcaDA";
        react(&mut c, None);
        assert_eq!(c4, c);
        react(&mut c, None);
        assert_eq!(c4, c);
    }

    #[test]
    fn test_without() {
        let a = "dabAcCaCBAcCcaDA".to_string();
        let mut b = a.clone();
        react(&mut b, Some('a' as u8));
        let a2 = "dbCBcD";
        assert_eq!(a2, b);
        let mut b = a.clone();
        react(&mut b, Some('b' as u8));
        let a3 = "daCAcaDA";
        assert_eq!(a3, b);
        let mut b = a.clone();
        react(&mut b, Some('c' as u8));
        let a4 = "daDA";
        assert_eq!(a4, b);
        let mut b = a.clone();
        react(&mut b, Some('d' as u8));
        let a5 = "abCBAc";
        assert_eq!(a5, b);
    }

    #[bench]
    fn bench_main(b: &mut Bencher) {
        b.iter(|| main());
    }

}
