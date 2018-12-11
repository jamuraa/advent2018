use std::{
    collections::HashMap,
    fs::File,
    io::{self, prelude::*, BufReader},
};

struct GuardRecord {
    id: u32,
    minute_count: [u32; 60],
}

impl GuardRecord {
    /// Makes a new empty guard record with the given id
    fn new(id: u32) -> GuardRecord {
        GuardRecord {
            id,
            minute_count: [0; 60],
        }
    }

    /// Add a record of a guard's shift.
    fn add_shift(&mut self, fallwake: &[u32]) {
        for (sleep, wake) in fallwake.to_vec().chunks(2).map(|x| (x[0], x[1])) {
            for min in sleep as usize..wake as usize {
                self.minute_count[min] += 1;
            }
        }
    }

    fn sum_asleep(&self) -> u32 {
        self.minute_count.iter().sum()
    }

    fn sleepiest_minute(&self) -> usize {
        self.minute_count
            .iter()
            .enumerate()
            .max_by(|(_, x), (_, y)| x.cmp(&y))
            .unwrap()
            .0
    }
}

fn numbers_in_string(s: &str) -> Vec<u32> {
    s.split(|x| !char::is_numeric(x))
        .filter(|x| !x.is_empty())
        .map(|num_str| num_str.parse().unwrap())
        .collect()
}

fn main() -> io::Result<()> {
    // Note: input_sorted is generated with `sort input.txt > input_sorted.txt`
    let f = File::open("input_sorted.txt")?;
    let reader = BufReader::new(f);

    let mut guard_records: HashMap<u32, GuardRecord> = HashMap::new();

    // Read the input in.
    let mut lines = reader.lines().peekable();
    let mut shifts = 0;
    while let Some(Ok(shift_begins)) = lines.next() {
        let numbers = numbers_in_string(&shift_begins);
        let guard_id = numbers[5];
        let record = guard_records
            .entry(guard_id)
            .or_insert(GuardRecord::new(guard_id));
        let mut fallwake = Vec::new();
        loop {
            if let Some(Ok(line)) = lines.peek() {
                if line.find("Guard").is_some() {
                    break;
                }
            } else {
                break;
            };
            let line = lines.next().unwrap().unwrap();
            let numbers = numbers_in_string(&line);
            let minutes = numbers[4];
            fallwake.push(minutes);
        }
        record.add_shift(&fallwake);
        shifts += 1;
    }

    println!("{} guards with {} shifts\n", guard_records.len(), shifts);

    // Find the guard with the most minutes asleep
    let sleepiest = guard_records
        .values()
        .max_by(|x, y| x.sum_asleep().cmp(&y.sum_asleep()))
        .unwrap();

    println!(
        "Sleepiest Guard is #{} with {} mins asleep",
        sleepiest.id,
        sleepiest.sum_asleep()
    );

    let sleepy_minute = sleepiest.sleepiest_minute();
    println!(
        "Sleepiest minute for guard #{} is {} (magic {})\n",
        sleepiest.id,
        sleepy_minute,
        sleepiest.id * sleepy_minute as u32
    );

    let mut max_guard_id: u32 = 0;
    let mut max_guard_minute: u32 = 0;
    let mut max_guard_sleeps: u32 = 0;

    for guard in guard_records.values() {
        let sleepy_minute = guard.sleepiest_minute();
        let sleeps = guard.minute_count[sleepy_minute];
        println!(
            "Sleepiest minute for guard #{} is {} past (asleep {} times)",
            guard.id, sleepy_minute, sleeps
        );
        if sleeps > max_guard_sleeps {
            max_guard_id = guard.id;
            max_guard_minute = sleepy_minute as u32;
            max_guard_sleeps = sleeps;
        }
    }

    println!("Sleepiest minute for any guard:");
    println!(
        "#{} at {} past (asleep {} times) (magic {})",
        max_guard_id,
        max_guard_minute,
        max_guard_sleeps,
        max_guard_id * max_guard_minute
    );

    Ok(())
}
