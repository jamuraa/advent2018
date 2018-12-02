// Copyright 2018 Marie Janssen.  All rights reserved.
// Use of this source code is governed by a BSD-style license that can be found in the LICENSE file.

use {
    std::{
        io::{self, prelude::*, BufReader, ErrorKind},
        collections::HashSet,
        fs::File,
    }
};

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);


    // Read the input in.
    let mut current_frequency = 0;
    let mut pattern = Vec::new();
    let mut buffer = String::new();
    loop {
        buffer.clear();
        reader.read_line(&mut buffer)?;
        if buffer.is_empty() {
            println!("No lines left. {} elements in the repeating sequence. Frequency after one repeat is {}", pattern.len(), current_frequency);
            break;
        }
        let change = buffer.trim().parse::<i32>().map_err(|e| io::Error::new(ErrorKind::Interrupted, e))?;
        current_frequency = current_frequency + change;
        pattern.push(change);
    }

    let mut current_frequency = 0;
    let mut steps = 0;
    let mut reached = HashSet::new();
    reached.insert(current_frequency);

    for change in pattern.iter().cycle() {
        steps = steps + 1;
        current_frequency = current_frequency + change;
        if reached.contains(&current_frequency) {
            println!("The first frequency reached twice is {} after {} steps.", current_frequency, steps);
            return Ok(());
        }
        reached.insert(current_frequency);
    };
    Ok(())
}
