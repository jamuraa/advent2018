use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, prelude::*, BufReader},
    str::FromStr,
};

#[derive(Debug)]
struct Star(i64, i64, i64, i64);

impl Star {
    fn new(x: i64, y: i64, z: i64, t: i64) -> Star {
        Star(x, y, z, t)
    }

    fn dist_from(&self, other: &Star) -> i64 {
        (self.0 - other.0).abs()
            + (self.1 - other.1).abs()
            + (self.2 - other.2).abs()
            + (self.3 - other.3).abs()
    }
}

struct Constellation {
    stars: Vec<Star>,
}

impl Constellation {
    fn new() -> Constellation {
        Constellation { stars: Vec::new() }
    }

    fn attached(&self, star: &Star) -> bool {
        self.stars.iter().find(|x| x.dist_from(star) <= 3).is_some()
    }

    fn insert(&mut self, star: Star) {
        self.stars.push(star)
    }
}

fn main() -> io::Result<()> {
    let mut unowned_stars: Vec<Star> = Vec::new();

    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let mut lines = reader.lines();
    while let Some(Ok(line)) = lines.next() {
        let coords = numbers_in_string(&line);
        unowned_stars.push(Star::new(coords[0], coords[1], coords[2], coords[3]));
    }

    println!("Added {} stars to the sky", unowned_stars.len());

    let mut constellations: Vec<Constellation> = Vec::new();

    while !unowned_stars.is_empty() {
        // Start a constellation with the first star
        let mut new_constellation = Constellation::new();
        new_constellation.insert(unowned_stars.pop().unwrap());
        // Keep trying to add stars to this constellation until we can't anymore
        loop {
            let (attached, unowned): (Vec<Star>, Vec<Star>) = unowned_stars
                .into_iter()
                .partition(|s| new_constellation.attached(&s));
            unowned_stars = unowned;
            let mut count = 0;
            for s in attached {
                new_constellation.insert(s);
                count += 1;
            }
            if count == 0 {
                // We didn't add any stars, move on to the next constellation.
                break;
            }
        }
        constellations.push(new_constellation);
    }

    println!("There are {} constellations: ", constellations.len());

    for c in constellations {
        println!("{} stars: {:?}", c.stars.len(), c.stars);
    }

    Ok(())
}

fn numbers_in_string<T>(s: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    s.split(|x| !char::is_numeric(x) && x != '-')
        .filter(|x| !x.is_empty())
        .map(|num_str| num_str.parse().unwrap())
        .collect()
}
