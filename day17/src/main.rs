use std::{
    cmp::{max, min},
    collections::{HashSet, VecDeque},
    fmt,
    fs::File,
    io::{self, prelude::*, BufReader},
    str::FromStr,
    u16,
};

enum FlowSituation {
    /// There is a corner at y, x (wall to the left or right and a wall to the bottom
    Corner(u16, u16),
    /// There is a cliff, and the water would fall starting at position (y, x)
    Cliff(u16, u16),
}

struct ClayScan {
    locations: HashSet<(u16, u16)>,
    /// locations of the clay, with the y value first.
    water: HashSet<(u16, u16)>,
    /// locations that water has settled
    reached: HashSet<(u16, u16)>,
    /// locations that water has touched
    drops: HashSet<(u16, u16)>,
    /// locations we've dropped water from
    range_x: (u16, u16),
    range_y: (u16, u16),
}

impl ClayScan {
    fn new() -> ClayScan {
        ClayScan {
            locations: HashSet::new(),
            water: HashSet::new(),
            reached: HashSet::new(),
            drops: HashSet::new(),
            range_x: (u16::MAX, 0),
            range_y: (0, 0),
        }
    }

    fn insert_horiz(&mut self, y: u16, x_start: u16, x_end: u16) {
        for x in x_start..x_end + 1 {
            self.locations.insert((y, x));
        }
        self.range_y = (min(self.range_y.0, y), max(self.range_y.1, y));
        self.range_x = (min(self.range_x.0, x_start), max(self.range_x.1, x_end));
    }

    fn insert_vert(&mut self, x: u16, y_start: u16, y_end: u16) {
        for y in y_start..y_end + 1 {
            self.locations.insert((y, x));
        }
        self.range_y = (min(self.range_y.0, y_start), max(self.range_y.1, y_end));
        self.range_x = (min(self.range_x.0, x), max(self.range_x.1, x));
    }

    // Returns true if there is either water or clay at spot y, x.
    fn spot_full(&self, y: u16, x: u16) -> bool {
        self.locations.contains(&(y, x)) || self.water.contains(&(y, x))
    }

    fn situation_left_of(&mut self, y: u16, x: u16) -> FlowSituation {
        let mut scan_x = x;
        loop {
            // We weached this spot with water no matter what.
            self.reached.insert((y, scan_x));
            // If we don't have something below us, we're cliffed
            if !self.spot_full(y + 1, scan_x) {
                return FlowSituation::Cliff(y, scan_x);
            }
            // If we have something to our left, we are cornered
            if self.spot_full(y, scan_x - 1) {
                return FlowSituation::Corner(y, scan_x);
            }
            // Otherwise let's keep scanning.
            scan_x -= 1;
        }
    }

    fn situation_right_of(&mut self, y: u16, x: u16) -> FlowSituation {
        let mut scan_x = x;
        loop {
            // We reached this spot with water.
            self.reached.insert((y, scan_x));
            // If we don't have something below us, we're cliffed
            if !self.spot_full(y + 1, scan_x) {
                return FlowSituation::Cliff(y, scan_x);
            }
            // If we have something to our right, we are cornered
            if self.spot_full(y, scan_x + 1) {
                return FlowSituation::Corner(y, scan_x);
            }
            // Otherwise let's keep scanning.
            scan_x += 1;
        }
    }

    /// Fills the clay if water is falling starting at x, y
    fn fill(&mut self, y: u16, x: u16) {
        let mut drops_left = HashSet::new();
        let mut dead_drops = HashSet::new();
        drops_left.insert((y, x));
        'drops: while !drops_left.is_empty() {
            let drop_to_take = drops_left.iter().next().unwrap().clone();
            let drop_start = drops_left.take(&drop_to_take).unwrap();
            self.drops.insert(drop_start);
            //print!("{:?} ->", drop_start);
            let mut drop_loc = drop_start.clone();
            //println!("Water source location at {:?}", drop_loc);
            // If we are already underwater, we can ignore this.
            if self.water.contains(&drop_loc) {
                //println!("Underwater drop location at {:?}, bubbling up...", drop_loc);
                drop_loc = (drop_loc.0 - 1, drop_loc.1);
                drops_left.insert(drop_loc);
                //print_clayscan_range(self, &self.range_x, &(max(0, drop_loc.0 as i32 - 20) as u16, drop_loc.0 + 5));
                continue 'drops;
            }
            // Travel down until you hit something
            while !self.spot_full(drop_loc.0 + 1, drop_loc.1) {
                drop_loc = (drop_loc.0 + 1, drop_loc.1);
                // If we are still falling off the map, we can't fill anything else.
                if drop_loc.0 > self.range_y.1 {
                    println!(
                        "Water flowed off the map at {:?}, {} drops left",
                        drop_loc,
                        drops_left.len()
                    );
                    dead_drops.insert(drop_start);
                    continue 'drops;
                }
                // Otherwise we've "reached" this tile.
                self.reached.insert(drop_loc.clone());
            }
            // What's going on here?
            match (
                self.situation_left_of(drop_loc.0, drop_loc.1),
                self.situation_right_of(drop_loc.0, drop_loc.1),
            ) {
                (FlowSituation::Corner(y1, x1), FlowSituation::Corner(y2, x2)) => {
                    // We are gonna fill this whole level from x1 to x2, then fill again,
                    // with the next drop from where we started.
                    //println!("Filling with water ({}, {}..{})", y1, x1, x2);
                    for x in x1..x2 + 1 {
                        self.water.insert((y1, x));
                    }
                    drops_left.insert(drop_start);
                    //print_clayscan_range(self, &self.range_x, &(drop_start.0, y1 + 5));
                }
                (FlowSituation::Corner(y1, x1), FlowSituation::Cliff(y2, x2)) => {
                    // We fall off the right side. Add a drop location there, and we're done.
                    if !dead_drops.contains(&(y2, x2)) {
                        drops_left.insert((y2, x2));
                    }
                    //println!("Flowed off to the right ({}, {})", y2, x2);
                    //print_clayscan_range(self, &self.range_x, &(drop_start.0, y2 + 5));
                }
                (FlowSituation::Cliff(y1, x1), FlowSituation::Corner(y2, x2)) => {
                    // Same as last time, but this time it's the left side.
                    if !dead_drops.contains(&(y1, x1)) {
                        drops_left.insert((y1, x1));
                    }
                    //println!("Flowed off to the left ({}, {})", y1, x1);
                    //print_clayscan_range(self, &self.range_x, &(drop_start.0, y1 + 5));
                }
                (FlowSituation::Cliff(y1, x1), FlowSituation::Cliff(y2, x2)) => {
                    // We are overflowing both sides, so we drop from both sides at once.
                    if dead_drops.contains(&(y1, x1)) && dead_drops.contains(&(y2, x2)) {
                        // The drop that made this is also a dead drop,
                        dead_drops.insert(drop_start);
                    } else {
                        if !dead_drops.contains(&(y1, x1)) {
                            drops_left.insert((y1, x1));
                        }
                        if !dead_drops.contains(&(y2, x2)) {
                            drops_left.insert((y2, x2));
                        }
                    }
                    //println!("Flowed off both sides ({}, {}) and ({}, {})", y1, x1, y2, x2);
                    //print_clayscan_range(self, &self.range_x, &(drop_start.0, y1 + 5));
                }
            }
        }
    }
}

fn print_clayscan_range(scan: &ClayScan, range_x: &(u16, u16), range_y: &(u16, u16)) {
    for y in range_y.0..range_y.1 + 1 {
        for x in range_x.0..range_x.1 + 1 {
            if scan.drops.contains(&(y, x)) {
                print!("*");
            } else if scan.locations.contains(&(y, x)) {
                print!("#");
            } else if scan.water.contains(&(y, x)) {
                print!("~");
            } else if scan.reached.contains(&(y, x)) {
                print!("|");
            } else {
                print!(".");
            }
        }
        print!("\r\n");
    }
}

impl fmt::Display for ClayScan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in self.range_y.0..self.range_y.1 + 1 {
            for x in self.range_x.0..self.range_x.1 + 1 {
                if self.drops.contains(&(y, x)) {
                    print!("*");
                } else if self.locations.contains(&(y, x)) {
                    write!(f, "#")?;
                } else if self.water.contains(&(y, x)) {
                    write!(f, "~")?;
                } else if self.reached.contains(&(y, x)) {
                    write!(f, "|")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\r\n")?;
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut scan = ClayScan::new();

    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let mut lines = reader.lines();
    while let Some(Ok(line)) = lines.next() {
        let nums = numbers_in_string(&line);
        if line.starts_with('x') {
            scan.insert_vert(nums[0], nums[1], nums[2]);
        } else {
            scan.insert_horiz(nums[0], nums[1], nums[2]);
        }
    }

    println!(
        "Found {} clay locations in the ranges: x {:?} y {:?}",
        scan.locations.len(),
        scan.range_x,
        scan.range_y
    );

    println!("Filling starting at y = 0, x = 500..");
    scan.fill(0, 500);

    println!("{}", scan);

    println!("The water reaches {} spots", scan.reached.len());
    println!("The water settles in {} spots", scan.water.len());

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
