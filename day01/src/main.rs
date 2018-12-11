// Copyright 2018 Marie Janssen.  All rights reserved.
// Use of this source code is governed by a BSD-style license that can be found in the LICENSE file.

use std::{
    collections::HashSet,
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    // Read the input in.
    let pattern: Vec<_> = reader
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();
    println!("{} elements in the repeating sequence.", pattern.len());
    println!( "Frequency after one repeat is {}", pattern.iter().sum::<i32>());

    let sums = pattern.iter().cycle().scan(0, |a, &x| {
        *a += x;
        Some(*a)
    });

    let mut reached = HashSet::new();
    reached.insert(0);

    for (steps, current_frequency) in sums.enumerate() {
        if reached.contains(&current_frequency) {
            println!(
                "The first frequency reached twice is {} after {} steps.",
                current_frequency, steps
            );
            return Ok(());
        }
        reached.insert(current_frequency);
    }
    Ok(())
}
