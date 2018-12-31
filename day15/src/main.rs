use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, BinaryHeap},
    fmt,
    fs::File,
    io::{self, prelude::*, BufReader},
};

#[derive(Clone)]
struct Unit {
    x: usize,
    y: usize,
    team: char,
    hitpoints: i64,
    power: i64,
}

const INITIAL_HITPOINTS: i64 = 200;

impl Unit {
    fn new(x: usize, y: usize, team: char, power: i64) -> Unit {
        Unit {
            x,
            y,
            team,
            power,
            hitpoints: INITIAL_HITPOINTS,
        }
    }

    fn reading_order_cmp(&self, other: &Unit) -> Ordering {
        (self.x, self.y).cmp(&(other.x, other.y))
    }

    fn is_enemy(&self, other: &Unit) -> bool {
        other.hitpoints > 0 && self.team != other.team
    }

    fn in_range(&self, other: &Unit) -> bool {
        if other.hitpoints <= 0 {
            false
        } else {
            neighbors_of(self.x, self.y)
                .iter()
                .find(|&&s| s == (other.x, other.y))
                .is_some()
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}({}) at ({}, {})",
            self.team, self.hitpoints, self.x, self.y
        )?;
        Ok(())
    }
}

struct Map {
    rows: usize,
    cols: usize,
    walls: HashSet<(usize, usize)>,
    units: Vec<Unit>,
}

fn neighbors_of(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut n = Vec::new();
    if x > 0 {
        n.push((x - 1, y));
    }
    if y > 0 {
        n.push((x, y - 1));
    }
    n.push((x, y + 1));
    n.push((x + 1, y));
    n
}

impl Map {
    fn new(initial: &[String], elves_power: i64, goblins_power: i64) -> Map {
        let mut units = Vec::new();
        let mut walls = HashSet::new();
        for (x, row) in initial.iter().enumerate() {
            // Find all the elves and goblins in this row, and store the layout.
            for (y, c) in row.chars().enumerate() {
                match c {
                    'E' => units.push(Unit::new(x, y, 'E', elves_power)),
                    'G' => units.push(Unit::new(x, y, 'G', goblins_power)),
                    '#' => {
                        walls.insert((x, y));
                    }
                    _ => (),
                };
            }
        }
        Map {
            units,
            walls,
            rows: initial.len(),
            cols: initial[0].len(),
        }
    }

    fn unit_at<'a>(&'a self, x: &usize, y: &usize) -> Option<&Unit> {
        self.units
            .iter()
            .find(|u| u.hitpoints > 0 && &u.x == x && &u.y == y)
    }

    fn is_occupied(&self, x: &usize, y: &usize) -> bool {
        self.walls.contains(&(*x, *y)) || self.unit_at(x, y).is_some()
    }

    // Run Dijkstra's algorithm to find the shortest path to all the (reachable) points in the map.
    fn dijkstras_from(
        &self,
        x: usize,
        y: usize,
    ) -> (
        HashMap<(usize, usize), usize>,
        HashMap<(usize, usize), (usize, usize)>,
    ) {
        let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
        let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

        // We don't care if the current spot is occupied or not
        let mut open = BinaryHeap::new();
        open.push((std::usize::MAX - 0, (x, y)));
        dist.insert((x, y), 0);

        while !open.is_empty() {
            let (_, u) = open.pop().unwrap();

            for v in neighbors_of(u.0, u.1) {
                if !self.is_occupied(&v.0, &v.1) {
                    let cur_dist = dist.get(&v).unwrap_or(&std::usize::MAX);
                    let new_dist = dist[&u] + 1;
                    if new_dist < *cur_dist {
                        open.push((std::usize::MAX - new_dist, v));
                        dist.insert(v, new_dist);
                        prev.insert(v, u);
                    }
                }
            }
        }

        (dist, prev)
    }

    fn round(&mut self) -> Option<char> {
        // Sort the units in reading order
        self.units.sort_unstable_by(|a, b| a.reading_order_cmp(&b));
        for idx in 0..self.units.len() {
            // If the active unit is dead it will be removed after everyone else takes a turn but
            // it doesn't get a turn.
            if self.units[idx].hitpoints <= 0 {
                continue;
            }
            let mut active = self.units[idx].clone();

            // If a target is in range
            let targets: Vec<usize> = self
                .units
                .iter()
                .enumerate()
                .filter(|(_, x)| active.is_enemy(x))
                .map(|(idx, _)| idx)
                .collect();
            if targets.is_empty() {
                // Combat is over, the active unit's team won.
                return Some(active.team);
            }
            let targets_in_range: Vec<_> = targets
                .iter()
                .filter(|&&x| active.in_range(&self.units[x]))
                .collect();
            if targets_in_range.is_empty() {
                // No targets in range, movement
                let (distance, prev) = self.dijkstras_from(active.x, active.y);
                let mut reachable_squares_in_range: Vec<_> = targets
                    .iter()
                    .map(|&o| neighbors_of(self.units[o].x, self.units[o].y))
                    .flatten()
                    .filter(|(x, y)| !self.is_occupied(x, y))
                    .filter(|u| prev.get(u).is_some())
                    .collect();

                if reachable_squares_in_range.is_empty() {
                    // The unit ends it's turn.
                    //println!("{} can't move to any enemy, skipping turn", active);
                    continue;
                }
                // Sort the squares first by reading order (reversed)
                reachable_squares_in_range.sort_unstable_by(|a, b| b.cmp(a));
                // then a stable sort by distance (reversed)
                reachable_squares_in_range.sort_by(|a, b| distance[b].cmp(&distance[a]));
                // the last element is the chosen spot
                let chosen = reachable_squares_in_range.pop().unwrap();
                let (distance, _) = self.dijkstras_from(chosen.0, chosen.1);
                let mut next_steps: Vec<_> = neighbors_of(active.x, active.y)
                    .iter()
                    .filter(|x| distance.get(x).is_some())
                    .cloned()
                    .collect();
                next_steps.sort_by(|a, b| distance.get(a).unwrap().cmp(&distance.get(b).unwrap()));
                let next_step = next_steps[0];
                //print!("{} choosing to head towards ({}, {}) and stepping to ({}, {}) ",
                //    active, chosen.0, chosen.1, next_step.0, next_step.1);
                active.x = next_step.0;
                active.y = next_step.1;
            }
            self.units.get_mut(idx).unwrap().x = active.x;
            self.units.get_mut(idx).unwrap().y = active.y;
            // After movement, we may have a target in range now. re-target.
            let mut targets_in_range: Vec<_> = targets
                .iter()
                .filter(|&&x| active.in_range(&self.units[x]))
                .collect();
            if !targets_in_range.is_empty() {
                // Already in reading order (since units was in reading order), so stably sorting
                // by hitpoints means the target to damage will be first.
                targets_in_range
                    .sort_by(|&&a, &&b| self.units[a].reading_order_cmp(&self.units[b]));
                targets_in_range
                    .sort_by(|&&a, &&b| self.units[a].hitpoints.cmp(&self.units[b].hitpoints));
                self.units.get_mut(*targets_in_range[0]).unwrap().hitpoints -= active.power;
            //println!("{} attacks {}", active, self.units[*targets_in_range[0]])
            } else {
                //println!("");
            }
        }
        // Remove dead units from the board
        self.units.retain(|x| x.hitpoints > 0);
        // No one won so far..
        None
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in 0..self.rows {
            let mut units_on_line = " ".to_string();
            for y in 0..self.cols {
                if self.walls.contains(&(x, y)) {
                    write!(f, "#")?;
                } else if let Some(u) = self.unit_at(&x, &y) {
                    write!(f, "{}", u.team)?;
                    units_on_line = format!("{} {}", units_on_line, u);
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "{}\n", units_on_line)?;
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut strings: Vec<_> = Vec::new();

    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let mut lines = reader.lines();
    while let Some(Ok(line)) = lines.next() {
        strings.push(line);
    }

    let mut elves_power = 3;
    let goblins_power = 3;
    'power: loop {
        let mut map = Map::new(&strings, elves_power, goblins_power);
        let initial_elves = map.units.iter().filter(|x| x.team == 'E').count();
        let mut rounds = 0;
        loop {
            println!("Round {} Map:\n{}", rounds, map);
            rounds += 1;
            let winner = map.round();
            if map.units.iter().filter(|x| x.team == 'E').count() != initial_elves {
                elves_power += 1;
                continue 'power;
            }
            if winner.is_some() {
                break;
            }
        }

        map.units.retain(|x| x.hitpoints > 0);
        let num_units = map.units.len();
        println!("There are {} {} units left", num_units, map.units[0].team);

        for unit in &map.units {
            println!("{}", unit);
        }

        let sum_hp = map.units.iter().fold(0, |a, x| a + x.hitpoints);
        println!("Total hitpoints: {}", sum_hp);

        println!("Battle outcome: {}", sum_hp * (rounds - 1));
        break;
    }
    Ok(())
}
