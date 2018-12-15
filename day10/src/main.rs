use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn numbers_in_string(s: &str) -> Vec<i32> {
    s.split(|x| !char::is_numeric(x) && x != '-')
        .filter(|x| !x.is_empty())
        .map(|num_str| num_str.parse().unwrap())
        .collect()
}

struct Star {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Star {
    fn new(x: i32, y: i32, velx: i32, vely: i32) -> Star {
        Star {
            position: (x, y),
            velocity: (velx, vely),
        }
    }

    fn next(&self) -> Star {
        let new = Star::new(
            self.position.0 + self.velocity.0,
            self.position.1 + self.velocity.1,
            self.velocity.0,
            self.velocity.1,
        );
        //println!("Star: Old({}, {}) + ({}, {}) => ({}, {})",
        //          self.position.0, self.position.1,
        //          self.velocity.0, self.velocity.1,
        //          new.position.0, new.position.1);
        new
    }
}

fn min_x(field: &Vec<Star>) -> i32 {
    field
        .iter()
        .min_by(|x, y| x.position.0.cmp(&y.position.0))
        .unwrap()
        .position
        .0
}

fn max_x(field: &Vec<Star>) -> i32 {
    field
        .iter()
        .max_by(|x, y| x.position.0.cmp(&y.position.0))
        .unwrap()
        .position
        .0
}

fn min_y(field: &Vec<Star>) -> i32 {
    field
        .iter()
        .min_by(|x, y| x.position.1.cmp(&y.position.1))
        .unwrap()
        .position
        .1
}

fn max_y(field: &Vec<Star>) -> i32 {
    field
        .iter()
        .max_by(|x, y| x.position.1.cmp(&y.position.1))
        .unwrap()
        .position
        .1
}

fn print_stars(field: &Vec<Star>) {
    let max_x = max_x(field);
    let min_x = min_x(field);
    let max_y = max_y(field);
    let min_y = min_y(field);

    let rowspan = max_x - min_x + 1;
    let skylen = (max_y - min_y + 1) * rowspan;
    let mut sky = Vec::new();
    sky.resize(skylen as usize, '.');

    for s in field {
        let idx = (s.position.0 - min_x) + (s.position.1 - min_y) * rowspan;
        sky[idx as usize] = '#';
    }

    let mut idx = 0;
    while idx < skylen {
        let s: String = sky[idx as usize..(idx + rowspan) as usize].iter().collect();
        println!("{}", s);
        idx += rowspan;
    }
    println!("");
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut lines = reader.lines();

    let mut stars: Vec<Star> = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        let nums = numbers_in_string(&line);
        stars.push(Star::new(nums[0], nums[1], nums[2], nums[3]));
    }

    let mut range_x = max_x(&stars) - min_x(&stars);
    let mut range_y = max_y(&stars) - min_y(&stars);

    let mut secs = 0;
    while secs < 200000 {
        if range_x < 20 || range_y < 20 {
            println!("After {} secs:", secs);
            print_stars(&stars);
        }
        stars = stars.iter().map(|x| x.next()).collect();
        range_x = max_x(&stars) - min_x(&stars);
        range_y = max_y(&stars) - min_y(&stars);
        secs += 1;
    }
    Ok(())
}
