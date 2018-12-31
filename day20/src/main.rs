use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fmt,
    fs::File,
    io::{self, prelude::*, BufReader},
};

struct Map {
    doors: HashSet<(usize, usize)>,
    rooms: HashSet<(usize, usize)>,
}

/// Build sets of the doors and rooms attached to an open space at x, y given the (partial) regex.
fn build_doors_rooms(
    regex: &str,
    cursors: &HashSet<(usize, usize)>,
) -> (
    HashSet<(usize, usize)>,
    HashSet<(usize, usize)>,
    HashSet<(usize, usize)>,
) {
    let mut chars = regex.chars();
    let mut running_cursors = cursors.clone();
    let mut doors_found = HashSet::new();
    // We're at least standing in a room
    let mut rooms_found: HashSet<(usize, usize)> = running_cursors.iter().cloned().collect();
    //println!("Building map: {}, {:?}", regex, running_cursors);
    while let Some(r) = chars.next() {
        match r {
            'W' => {
                for at in &running_cursors {
                    // There's as door to our left(s)
                    doors_found.insert((at.0, at.1 - 1));
                    // and an open room beyond
                    rooms_found.insert((at.0, at.1 - 2));
                }
                // travel to them.
                running_cursors = running_cursors
                    .into_iter()
                    .map(|(x, y)| (x, y - 2))
                    .collect();
            }
            'E' => {
                for at in &running_cursors {
                    // There's as door to our right(s)
                    doors_found.insert((at.0, at.1 + 1));
                    // and an open room beyond
                    rooms_found.insert((at.0, at.1 + 2));
                }
                // travel to them.
                running_cursors = running_cursors
                    .into_iter()
                    .map(|(x, y)| (x, y + 2))
                    .collect();
            }
            'N' => {
                for at in &running_cursors {
                    // There's as door to our top(s)
                    doors_found.insert((at.0 - 1, at.1));
                    // and an open room beyond
                    rooms_found.insert((at.0 - 2, at.1));
                }
                // travel to them.
                running_cursors = running_cursors
                    .into_iter()
                    .map(|(x, y)| (x - 2, y))
                    .collect();
            }
            'S' => {
                for at in &running_cursors {
                    // There's as door to our bottom(s)
                    doors_found.insert((at.0 + 1, at.1));
                    // and an open room beyond
                    rooms_found.insert((at.0 + 2, at.1));
                }
                // travel to them.
                running_cursors = running_cursors
                    .into_iter()
                    .map(|(x, y)| (x + 2, y))
                    .collect();
            }
            '(' => {
                // We've started a choose your adventure. Grab the whole regex.
                let mut cyoa = String::new();
                let mut nested = 0;
                let mut end_cursors = HashSet::new();
                while let Some(r) = chars.next() {
                    if nested == 0 {
                        if r == ')' {
                            // we've found our matching close bracket
                            let (new_doors, new_rooms, new_cursors) =
                                build_doors_rooms(&cyoa, &running_cursors);
                            // Add where we ended up (this is the first half)
                            for cursor in new_cursors {
                                end_cursors.insert(cursor);
                            }
                            for door in new_doors {
                                doors_found.insert(door);
                            }
                            for room in new_rooms {
                                rooms_found.insert(room);
                            }
                            break;
                        } else if r == '|' {
                            // we've found our branch, explore the first then start a new branch
                            let (new_doors, new_rooms, new_cursors) =
                                build_doors_rooms(&cyoa, &running_cursors);
                            // Add where we ended up (this is the first half)
                            for door in new_doors {
                                doors_found.insert(door);
                            }
                            for room in new_rooms {
                                rooms_found.insert(room);
                            }
                            end_cursors = new_cursors;
                            // Continue on to build the other half of the string
                            cyoa = String::new();
                            continue;
                        }
                    }
                    cyoa.push(r);
                    if r == '(' {
                        nested += 1;
                    } else if r == ')' {
                        nested -= 1;
                    }
                }
                running_cursors = end_cursors;
            }
            // We shouldn't be able to encounter anything else.
            what => panic!("Unexpected character while dungeoncrawling: {}", what),
        }
    }
    (doors_found, rooms_found, running_cursors)
}

impl Map {
    fn from_regex(regex: &String) -> Map {
        let mut start = HashSet::new();
        start.insert((5000, 5000));
        let (doors, rooms, _) = build_doors_rooms(&regex[1..regex.len() - 1], &start);
        Map { doors, rooms }
    }

    fn neighbors(&self, at: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut n = Vec::new();
        if self.doors.contains(&(at.0 - 1, at.1)) {
            n.push((at.0 - 2, at.1));
        }
        if self.doors.contains(&(at.0 + 1, at.1)) {
            n.push((at.0 + 2, at.1));
        }
        if self.doors.contains(&(at.0, at.1 - 1)) {
            n.push((at.0, at.1 - 2));
        }
        if self.doors.contains(&(at.0, at.1 + 1)) {
            n.push((at.0, at.1 + 2));
        }
        n
    }

    // Run Dijkstra's algorithm to find the shortest path to all the (reachable) points in the map.
    fn dijkstras_from(
        &self,
        at: &(usize, usize),
    ) -> (
        HashMap<(usize, usize), usize>,
        HashMap<(usize, usize), (usize, usize)>,
    ) {
        let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
        let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

        // No doors to the current spot.
        let mut open = BinaryHeap::new();
        open.push((std::usize::MAX - 0, (at.0, at.1)));
        dist.insert((at.0, at.1), 0);

        while !open.is_empty() {
            let (_, u) = open.pop().unwrap();

            for v in self.neighbors(&u) {
                let cur_dist = dist.get(&v).unwrap_or(&std::usize::MAX);
                let new_dist = dist[&u] + 1;
                if new_dist < *cur_dist {
                    open.push((std::usize::MAX - new_dist, v));
                    dist.insert(v, dist[&u] + 1);
                    prev.insert(v, u);
                }
            }
        }

        (dist, prev)
    }

    fn doors_to_furthest_room(&self) -> usize {
        let (dist, _) = self.dijkstras_from(&(5000, 5000));
        dist.iter().map(|(_, &dist)| dist).max().unwrap()
    }

    fn gte_n_doors_away(&self, n: &usize) -> usize {
        let (dist, _) = self.dijkstras_from(&(5000, 5000));
        dist.iter()
            .map(|(_, &dist)| dist)
            .filter(|dist| dist >= n)
            .count()
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let upper_left = self.rooms.iter().min().unwrap();
        let bottom_right = self.rooms.iter().max().unwrap();
        for x in upper_left.0 - 1..bottom_right.0 + 2 {
            for y in upper_left.1 - 1..bottom_right.1 + 2 {
                if self.rooms.contains(&(x, y)) {
                    write!(f, ".")?;
                } else if self.doors.contains(&(x, y)) {
                    if self.rooms.contains(&(x - 1, y)) {
                        write!(f, "-")?;
                    } else {
                        write!(f, "|")?;
                    }
                } else {
                    write!(f, "#")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut strings = Vec::new();
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let mut lines = reader.lines();
    while let Some(Ok(line)) = lines.next() {
        strings.push(line);
    }

    for regex in strings {
        let room_map = Map::from_regex(&regex);
        let furthest = room_map.doors_to_furthest_room();
        println!(
            "{}\nFurthest room requres passing {} doors.\n\n{}",
            regex, furthest, room_map
        );
        println!(
            "{} rooms are at least {} doors away",
            room_map.gte_n_doors_away(&1000),
            1000
        );
    }

    Ok(())
}
