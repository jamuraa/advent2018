use {
    std::{
    collections::{HashMap, HashSet},
    fs::File,
    iter,
    io::{self, prelude::*, BufReader},
}
};

type Place = (i32,i32);

#[derive(PartialEq, Debug)]
struct Claim {
    id: i32,
    start: (i32, i32),
    width: i32,
    height: i32
}

// A Claim Map maps places on the fabric to claims that want to use that piece
type ClaimMap = HashMap<Place, HashSet<i32>>;

impl Claim {
    // Uuuuugh this is fragile and bleh.
    // TODO: Find a library to help out next time.
    fn parse(s: &str) -> Claim {
        let mut sides = s.split('@');
        let idish = sides.next().unwrap().trim_start_matches('#').trim();
        let spec = sides.next().unwrap();
        let id = idish.parse::<i32>().unwrap();
        let mut spec_iter = spec.split(':');
        let place_text = spec_iter.next().unwrap();
        let size_text = spec_iter.next().unwrap();
        let mut place_iter = place_text.split(',');
        let x = place_iter.next().unwrap().trim().parse::<i32>().unwrap();
        let y = place_iter.next().unwrap().trim().parse::<i32>().unwrap();
        let mut size_iter = size_text.split('x');
        let width = size_iter.next().unwrap().trim().parse::<i32>().unwrap();
        let height = size_iter.next().unwrap().trim().parse::<i32>().unwrap();
        Claim { id, start: (x, y), width, height }
    }

    fn places<'a>(&'a self) -> impl Iterator<Item = Place> + 'a {
        let xs = self.start.0..self.start.0+self.width;
        let ys = self.start.1..self.start.1+self.height;
        let width: usize = self.width as usize;
        let ys_repeated = ys.map(move |y| iter::repeat(y).take(width)).flatten();
        xs.cycle().zip(ys_repeated).map(Into::into)
    }

    fn fill(&self, claims: &mut ClaimMap) {
        for place in self.places() {
            let entry = claims.entry(place).or_insert(HashSet::new());
            entry.insert(self.id);
        }
    }
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    // Read the input in.
    let claims: Vec<Claim> = reader.lines().map(|l| Claim::parse(&l.unwrap())).collect();
    println!("{} claims", claims.len());

    let mut map = ClaimMap::new();

    for claim in &claims {
        claim.fill(&mut map);
    }

    let mut overallocated = 0;
    let mut clear_claims : HashSet<i32> = claims.iter().map(|x| x.id).collect();
    for (key, val) in map.iter() {
        if val.len() > 1 {
            println!("{:?} is overallocated: {} claims", key, val.len());
            for id in val {
                clear_claims.remove(&id);
            }
            overallocated += 1;
        }
    }
    println!("A total of {} spaces are overallocated", overallocated);
    println!("Claims {:?} are not overlapping with anything", clear_claims);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_claims() {
        let claim = Claim { id: 1, start: (1, 3), width: 4, height: 4 };
        assert_eq!(claim, Claim::parse("#1 @ 1,3: 4x4"));
        let claim = Claim { id: 2, start: (3, 1), width: 4, height: 4 };
        assert_eq!(claim, Claim::parse("#2 @ 3,1: 4x4"));
        let claim = Claim { id: 3, start: (5, 5), width: 2, height: 2 };
        assert_eq!(claim, Claim::parse("#3 @ 5,5: 2x2"));
        let claim = Claim { id: 123, start: (3, 2), width: 5, height: 4 };
        assert_eq!(claim, Claim::parse("#123 @ 3,2: 5x4"));
    }

    #[test]
    fn fill_places() {
        let mut map = ClaimMap::new();
        Claim::parse("#1 @ 1,3: 4x4").fill(&mut map);
        Claim::parse("#2 @ 3,1: 4x4").fill(&mut map);
        Claim::parse("#3 @ 5,5: 2x2").fill(&mut map);

        assert_eq!(2, map.get(&(3, 3)).unwrap().len());
        assert_eq!(2, map.get(&(3, 4)).unwrap().len());
        assert_eq!(2, map.get(&(4, 3)).unwrap().len());
        assert_eq!(2, map.get(&(4, 4)).unwrap().len());
    }
}
