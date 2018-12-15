use std::{
    collections::HashMap,
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn numbers_in_string(s: &str) -> Vec<u32> {
    s.split(|x| !char::is_numeric(x))
        .filter(|x| !x.is_empty())
        .map(|num_str| num_str.parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn print_marbles(marbles: &Vec<usize>, current: usize) {
    return;
    for (idx, marble) in marbles.iter().enumerate() {
        if idx == current {
            print!("(");
        } else {
            print!(" ");
        }
        print!("{}", marble);
        if idx == current {
            print!(")");
        } else {
            print!(" ");
        }
    }
    println!("");
}

#[allow(dead_code)]
fn print_marbles_ring(marbles: &MarbleRing, current: usize) {
    let mut now = marbles.1;
    let mut remaining = marbles.len();
    while remaining > 0 {
        if now == current {
            print!("({})", now);
        } else {
            print!(" {} ", now);
        }
        now = marbles.next(&now, 1);
        remaining -= 1;
    }
    println!("");
}

struct Marble {
    number: usize,
    // The next and previous marble in the ring.
    next: usize,
    prev: usize,
}

impl Marble {
    fn new(number: usize, next: usize, prev: usize) -> Marble {
        Marble { number, next, prev }
    }
}

struct MarbleRing(HashMap<usize, Marble>, usize);

impl MarbleRing {
    fn new() -> MarbleRing {
        let mut ring = HashMap::new();
        ring.insert(0, Marble::new(0, 0, 0));
        MarbleRing(ring, 0)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn next(&self, spot: &usize, steps: usize) -> usize {
        let mut spot = self.0.get(spot).unwrap();
        let mut left = steps;
        while left > 0 {
            spot = self.0.get(&spot.next).unwrap();
            left -= 1;
        }
        spot.number
    }

    fn prev(&self, spot: &usize, steps: usize) -> usize {
        let mut spot = self.0.get(spot).unwrap();
        let mut left = steps;
        while left > 0 {
            spot = self.0.get(&spot.prev).unwrap();
            left -= 1;
        }
        spot.number
    }

    // Insert marble `new` clockwise of `exist`
    fn insert(&mut self, exist: usize, new: usize) {
        let next_num: usize;
        {
            let exist = self.0.get_mut(&exist).unwrap();
            next_num = exist.next;
            exist.next = new;
        }
        {
            let new_marble = Marble::new(new, next_num, exist);
            self.0.insert(new, new_marble);
        }
        {
            let next = self.0.get_mut(&next_num).unwrap();
            next.prev = new;
        }
    }

    // Remove the marble numbered `gone`.
    // Returns the number of the next marble in the ring.
    fn remove(&mut self, gone: usize) -> usize {
        let prev_num: usize;
        let next_num: usize;
        let removing;
        {
            removing = self.0.remove(&gone).unwrap();
            prev_num = removing.prev;
            next_num = removing.next;
        }
        {
            let prev = self.0.get_mut(&removing.prev).unwrap();
            prev.next = next_num;
        }
        {
            let next = self.0.get_mut(&removing.next).unwrap();
            next.prev = prev_num;
        }
        if self.1 == removing.number {
            self.1 = next_num;
        }

        next_num
    }
}

fn play_game_ring(players: usize, last_points: usize) -> u32 {
    let mut marbles = MarbleRing::new();

    let mut scores = Vec::new();
    scores.resize(players, 0);

    let mut current_player = 1;
    let mut next_marble = 1;
    let mut current_marble = 0;
    while next_marble <= last_points {
        if next_marble % 23 == 0 {
            scores[current_player] += next_marble;
            let to_remove = marbles.prev(&current_marble, 7);
            scores[current_player] += to_remove;
            current_marble = marbles.remove(to_remove);
        } else {
            let add_spot = marbles.next(&current_marble, 1);
            marbles.insert(add_spot, next_marble);
            current_marble = next_marble;
        }
        next_marble += 1;
        current_player = (current_player + 1) % players;
        //if next_marble % 100 == 0 {
        //    print!("Placed {} marbles\r", next_marble);
        //}
        //print_marbles_ring(&marbles, current_marble);
    }

    let mut high_score = 0;

    for (idx, score) in scores.iter().enumerate() {
        println!("Score for player {} is {}", idx, score);
        if score > &high_score {
            high_score = *score;
        }
    }
    println!("High Score is {}", high_score);
    high_score as u32
}

fn play_game(players: usize, last_points: usize) -> u32 {
    let mut marbles: Vec<usize> = vec![0];
    let mut current_marble_idx = 0;
    let mut next_marble;
    let mut current_player = 0;

    let mut scores = Vec::new();
    scores.resize(players, 0);

    if last_points < 23 {
        return 0;
    }

    print_marbles(&marbles, current_marble_idx);

    marbles = vec![0, 1];
    current_marble_idx = 1;
    print_marbles(&marbles, current_marble_idx);

    marbles = vec![0, 2, 1];
    current_marble_idx = 1;
    next_marble = 3;
    print_marbles(&marbles, current_marble_idx);

    marbles.reserve(last_points);

    while next_marble <= last_points {
        if next_marble % 23 == 0 {
            // Don't place it, it's scored!
            scores[current_player] += next_marble;
            let remove_idx = (current_marble_idx + marbles.len() - 7) % marbles.len();
            let removed = marbles.remove(remove_idx);
            scores[current_player] += removed;
            current_marble_idx = remove_idx;
        } else {
            let next_marble_idx = (current_marble_idx + 2) % marbles.len();
            marbles.insert(next_marble_idx, next_marble);
            current_marble_idx = next_marble_idx;
        }
        if next_marble % 100 == 0 {
            print!("Placed {} marbles\r", next_marble);
        }
        next_marble += 1;
        current_player = (current_player + 1) % players;
        print_marbles(&marbles, current_marble_idx);
    }

    let mut high_score = 0;

    for (idx, score) in scores.iter().enumerate() {
        println!("Score for player {} is {}", idx, score);
        if score > &high_score {
            high_score = *score;
        }
    }
    println!("High Score is {}", high_score);
    high_score as u32
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut lines = reader.lines();
    let nums = numbers_in_string(&lines.next().unwrap().unwrap());

    let players = nums[0];
    let last_points = nums[1];

    //play_game_ring(30, 5807);

    //play_game_ring(players as usize, last_points as usize);
    play_game_ring(players as usize, (last_points * 100) as usize);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        assert_eq!(32, play_game(9, 25));
        assert_eq!(8317, play_game(10, 1618));
        assert_eq!(146373, play_game(13, 7999));
        assert_eq!(2764, play_game(17, 1104));
        assert_eq!(54718, play_game(21, 6111));
        assert_eq!(37305, play_game(30, 5807));
    }

    #[test]
    fn test_game_ring() {
        assert_eq!(32, play_game_ring(9, 25));
        assert_eq!(8317, play_game_ring(10, 1618));
        assert_eq!(146373, play_game_ring(13, 7999));
        assert_eq!(2764, play_game_ring(17, 1104));
        assert_eq!(54718, play_game_ring(21, 6111));
        assert_eq!(37305, play_game_ring(30, 5807));
    }
}
