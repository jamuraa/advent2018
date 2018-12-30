use std::{
    fmt,
    fs::File,
    io::{self, prelude::*, BufReader},
};

struct Forest {
    map: Vec<String>,
}

impl Forest {
    fn new(map: &[String]) -> Forest {
        let mut rows = Vec::new();
        for line in map {
            rows.push(line.clone());
        }
        Forest { map: rows }
    }

    fn at<'a>(&'a self, x: usize, y: usize) -> char {
        self.map[x].as_bytes()[y] as char
    }

    fn count_neighbors<'a>(&'a self, x: usize, y: usize, acre: &char) -> usize {
        let xi: i64 = x as i64;
        let yi: i64 = y as i64;
        let neighbors: [(i64, i64); 8] = [
            (xi - 1, yi - 1),
            (xi - 1, yi),
            (xi - 1, yi + 1),
            (xi, yi - 1),
            (xi, yi + 1),
            (xi + 1, yi - 1),
            (xi + 1, yi),
            (xi + 1, yi + 1),
        ];

        let dim = self.map.len() as i64;
        neighbors
            .iter()
            .filter(|(x, _)| x >= &0 && x < &dim)
            .filter(|(_, y)| y >= &0 && y < &dim)
            .filter(|(x, y)| &self.at(*x as usize, *y as usize) == acre)
            .count()
    }

    fn next(&self) -> Forest {
        let dim = self.map.len();
        let mut rows = Vec::new();
        for x in 0..dim {
            let mut row: String = String::new();
            for y in 0..dim {
                let next = match self.at(x, y) {
                    '.' => {
                        if self.count_neighbors(x, y, &'|') >= 3 {
                            '|'
                        } else {
                            '.'
                        }
                    }
                    '|' => {
                        if self.count_neighbors(x, y, &'#') >= 3 {
                            '#'
                        } else {
                            '|'
                        }
                    }
                    '#' => {
                        if self.count_neighbors(x, y, &'#') >= 1
                            && self.count_neighbors(x, y, &'|') >= 1
                        {
                            '#'
                        } else {
                            '.'
                        }
                    }
                    x => panic!("What is in your forest!?!? a {}!?", x),
                };
                row.push(next);
            }
            rows.push(row);
        }
        Forest { map: rows }
    }

    fn count(&self, acre: &char) -> u64 {
        self.map
            .iter()
            .map(|x| x.chars())
            .flatten()
            .filter(|x| x == acre)
            .count() as u64
    }

    fn value(&self) -> u64 {
        let yards = self.count(&'#');
        let trees = self.count(&'|');
        yards * trees
    }
}

impl fmt::Display for Forest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.map.iter() {
            write!(f, "{}\n", line)?;
        }
        write!(f, "Value: {}", self.value())?;
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let mut lines = reader.lines();
    let mut strs: Vec<String> = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        strs.push(line);
    }

    let mut forest = Forest::new(&strs);

    println!("Initial forest: \n{}", forest);

    for mins in 0..1000000000 {
        forest = forest.next();
        println!("{}. {}", mins, forest.value());
        if mins % 10000 == 0 {
            println!("After {} mins: \n{}", mins + 1, forest);
        }
    }

    println!("After:\n{}", forest);
    Ok(())
}
