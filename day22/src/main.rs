use std::{
    collections::{BinaryHeap, HashMap},
    fmt,
};

enum Terrain {
    Rocky,
    Wet,
    Narrow,
}

impl Terrain {
    fn from_erosion(level: &usize) -> Terrain {
        match level % 3 {
            0 => Terrain::Rocky,
            1 => Terrain::Wet,
            2 => Terrain::Narrow,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Terrain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Terrain::Rocky => '.',
                Terrain::Wet => '=',
                Terrain::Narrow => '|',
            }
        )
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
enum Tool {
    Torch,
    Climbing,
    Neither,
}

impl fmt::Display for Tool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tool::Torch => "Torch",
                Tool::Climbing => "Climbing",
                Tool::Neither => "Neither",
            }
        )
    }
}

struct Cave {
    /// Map from x, y to erosion level
    map: HashMap<(usize, usize), usize>,
    target: (usize, usize),
    depth: usize,
}

impl Cave {
    fn new(depth: &usize, target: &(usize, usize)) -> Cave {
        let mut map = HashMap::new();
        for x in 0..target.0 + 300 {
            for y in 0..target.1 + 300 {
                let geo_lvl = if x == target.0 && y == target.1 {
                    0
                } else if x == 0 {
                    y * 48271
                } else if y == 0 {
                    x * 16807
                } else {
                    map.get(&(x - 1, y)).unwrap() * map.get(&(x, y - 1)).unwrap()
                };

                let ero_level = (geo_lvl + depth) % 20183;
                map.insert((x, y), ero_level);
            }
        }
        Cave {
            map,
            depth: *depth,
            target: target.clone(),
        }
    }

    fn erosion(&mut self, at: &(usize, usize)) -> usize {
        match self.map.get(&at) {
            Some(lvl) => *lvl,
            None => {
                let new_geo_lvl = if at.0 == 0 {
                    at.1 * 48271
                } else if at.1 == 0 {
                    at.0 * 16807
                } else {
                    self.erosion(&(at.0 - 1, at.1)) * self.erosion(&(at.0, at.1 - 1))
                };
                let ero_lvl = (new_geo_lvl + self.depth) % 20183;
                self.map.insert(*at, ero_lvl);
                ero_lvl
            }
        }
    }

    fn terrain(&mut self, at: &(usize, usize)) -> Terrain {
        Terrain::from_erosion(&self.erosion(at))
    }

    /// Returns the neighbors that are valid, with their time.
    fn neighbors_time(
        &mut self,
        at: &(usize, usize, Tool),
    ) -> HashMap<(usize, usize, Tool), usize> {
        let mut n = HashMap::new();
        // Could switch tools
        match (self.terrain(&(at.0, at.1)), &at.2) {
            (Terrain::Rocky, Tool::Climbing) => n.insert((at.0, at.1, Tool::Torch), 7),
            (Terrain::Rocky, Tool::Torch) => n.insert((at.0, at.1, Tool::Climbing), 7),
            (Terrain::Wet, Tool::Climbing) => n.insert((at.0, at.1, Tool::Neither), 7),
            (Terrain::Wet, Tool::Neither) => n.insert((at.0, at.1, Tool::Climbing), 7),
            (Terrain::Narrow, Tool::Neither) => n.insert((at.0, at.1, Tool::Torch), 7),
            (Terrain::Narrow, Tool::Torch) => n.insert((at.0, at.1, Tool::Neither), 7),
            (t, o) => panic!("Holding an invalid tool: {} at {}", t, o),
        };
        // Could move around instead.
        let mut shifts = Vec::new();
        if at.0 > 0 {
            shifts.push((at.0 - 1, at.1));
        }
        if at.1 > 0 {
            shifts.push((at.0, at.1 - 1));
        }
        shifts.push((at.0 + 1, at.1));
        shifts.push((at.0, at.1 + 1));
        for shift in shifts {
            match (self.terrain(&shift), &at.2) {
                (Terrain::Rocky, Tool::Climbing) => n.insert((shift.0, shift.1, Tool::Climbing), 1),
                (Terrain::Rocky, Tool::Torch) => n.insert((shift.0, shift.1, Tool::Torch), 1),
                (Terrain::Wet, Tool::Climbing) => n.insert((shift.0, shift.1, Tool::Climbing), 1),
                (Terrain::Wet, Tool::Neither) => n.insert((shift.0, shift.1, Tool::Neither), 1),
                (Terrain::Narrow, Tool::Neither) => n.insert((shift.0, shift.1, Tool::Neither), 1),
                (Terrain::Narrow, Tool::Torch) => n.insert((shift.0, shift.1, Tool::Torch), 1),
                _ => None,
            };
        }
        n
    }

    fn dijkstras_to_target(
        &mut self,
        at: &(usize, usize, Tool),
        target: &(usize, usize, Tool),
    ) -> usize {
        let mut dist: HashMap<(usize, usize, Tool), usize> = HashMap::new();

        // We start at the current spot, at zero minutes
        let mut open = BinaryHeap::new();
        open.push((std::usize::MAX - 0, (at.0, at.1, at.2)));
        dist.insert((at.0, at.1, at.2), 0);

        while !open.is_empty() {
            let (_, u) = open.pop().unwrap();
            print!("Exploring options from \t{:?}, \t({} mins)\r", u, dist[&u]);

            if &u == target {
                // We're done.
                return dist[&u];
            }

            for (v, time) in self.neighbors_time(&u) {
                let cur_dist = dist.get(&v).unwrap_or(&std::usize::MAX);
                let new_dist = dist[&u] + time;
                if new_dist < *cur_dist {
                    open.push((std::usize::MAX - new_dist, v));
                    dist.insert(v, new_dist);
                }
            }
        }

        dist[target]
    }

    fn risk_level(&self) -> usize {
        self.map.values().map(|v| v % 3).sum()
    }
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.target.1 + 1 {
            for x in 0..self.target.0 + 1 {
                if x == 0 && y == 0 {
                    write!(f, "M")?;
                } else if x == self.target.0 && y == self.target.1 {
                    write!(f, "T")?;
                } else {
                    write!(
                        f,
                        "{}",
                        Terrain::from_erosion(self.map.get(&(x, y)).unwrap())
                    )?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    let mut cave = Cave::new(&3558, &(15, 740));
    //let cave = Cave::new(&510, &(10, 10));

    println!("{}", cave);
    println!("\n\nRisk level of {}", cave.risk_level());
    println!(
        "\nShortest time to target is {}",
        cave.dijkstras_to_target(
            &(0, 0, Tool::Torch),
            &(cave.target.0, cave.target.1, Tool::Torch)
        )
    );
}
