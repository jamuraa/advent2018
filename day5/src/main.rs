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

fn react(chars: &mut String) {
    let mut i = 0;
    while i != chars.len() - 1 {
        let x = chars.as_bytes()[i];
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
}

fn without_polymer(chars: &String, rem: &char) -> String {
    let mut ret = chars.clone();
    ret.retain(|x| !x.eq_ignore_ascii_case(rem));
    ret
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut lines = reader.lines();
    let chars: String = lines.next().unwrap().unwrap();
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
        let mut a = "AcCb".to_string();
        let a2 = "Ab";
        react(&mut a);
        assert_eq!(a2, a);
        let mut c = "dabAcCaCBAcCcaDA".to_string();
        let c4 = "dabCBAcaDA";
        react(&mut c);
        assert_eq!(c4, c);
        react(&mut c);
        assert_eq!(c4, c);
    }

    #[test]
    fn test_without() {
        let a = "dabAcCaCBAcCcaDA".to_string();
        let a2 = "dbcCCBcCcD";
        assert_eq!(a2, without_polymer(&a, &'a'));
        let a3 = "daAcCaCAcCcaDA";
        assert_eq!(a3, without_polymer(&a, &'b'));
        let a4 = "dabAaBAaDA";
        assert_eq!(a4, without_polymer(&a, &'c'));
        let a5 = "abAcCaCBAcCcaA";
        assert_eq!(a5, without_polymer(&a, &'d'));
    }

}
