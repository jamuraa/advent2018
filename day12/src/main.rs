use std::collections::VecDeque;
use std::fmt;

struct Pots(VecDeque<i32>);

impl Pots {
    fn new(garden: String) -> Pots {
        Pots(
            garden
                .into_bytes()
                .iter()
                .enumerate()
                .filter(|(_i, x)| **x as char == '#')
                .map(|(i, _)| i as i32)
                .collect(),
        )
    }

    fn min_pot(&self) -> i32 {
        *self.0.front().unwrap()
    }

    fn max_pot(&self) -> i32 {
        *self.0.back().unwrap()
    }

    fn pot_at(&self, at: i32) -> bool {
        let mut it = self.0.iter();
        while let Some(idx) = it.next() {
            if idx == &at {
                return true;
            }
            if idx > &at {
                return false;
            }
        }
        return false;
    }

    fn next_generation(&self) -> Pots {
        let mut new_pots = VecDeque::new();
        let mut from_pot = self.min_pot() - 2;
        let mut pot_scan = VecDeque::new();
        pot_scan.push_back('.');
        pot_scan.push_back('.');
        pot_scan.push_back('.');
        pot_scan.push_back('.');
        pot_scan.push_back('#');
        while from_pot < self.max_pot() + 2 {
            let ng_pot = match pot_scan
                .iter()
                .fold(String::new(), |mut s, x| {
                    s.push(*x);
                    s
                })
                .as_str()
            {
                "....." => '.',
                "#...." => '.',
                "..###" => '.',
                "##..#" => '#',
                ".###." => '#',
                "...##" => '.',
                "#.#.." => '.',
                "..##." => '.',
                "##.#." => '#',
                "..#.." => '.',
                ".#..." => '#',
                "##.##" => '.',
                "....#" => '.',
                ".#.#." => '.',
                "#..#." => '#',
                "#.###" => '.',
                ".##.#" => '#',
                ".####" => '.',
                ".#..#" => '.',
                "####." => '#',
                "#...#" => '#',
                ".#.##" => '#',
                "#..##" => '.',
                "..#.#" => '#',
                "#.##." => '.',
                "###.." => '.',
                "#####" => '#',
                "###.#" => '#',
                "...#." => '#',
                "#.#.#" => '#',
                ".##.." => '.',
                "##..." => '#',
                //"...##" => '#',
                //"..#.." => '#',
                //".#..." => '#',
                //".#.#." => '#',
                //".#.##" => '#',
                //".##.." => '#',
                //".####" => '#',
                //"#.#.#" => '#',
                //"#.###" => '#',
                //"##.#." => '#',
                //"##.##" => '#',
                //"###.." => '#',
                //"###.#" => '#',
                //"####." => '#',
                _ => '.',
            };
            if ng_pot == '#' {
                new_pots.push_back(from_pot)
            }
            from_pot += 1;
            pot_scan.pop_front();
            if self.pot_at(from_pot + 2) {
                pot_scan.push_back('#');
            } else {
                pot_scan.push_back('.');
            }
        }
        Pots(new_pots)
    }
}

impl fmt::Display for Pots {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for idx in self.min_pot()..(self.max_pot() + 1) {
            if self.pot_at(idx) {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        write!(f, "{}", s)
    }
}

fn sum_after_generations(garden: String, gens: usize) -> i32 {
    let mut state = Pots::new(garden);

    println!("{}: {}", 0, state);
    for gen in 0..gens {
        state = state.next_generation();
        println!("{}: {}", gen, state);
        println!("Pots at: {:?}", state.0);
        let sum: i32 = state.0.iter().sum::<i32>();
        println!("Sum of {} locations: {}", state.0.len(), sum);
    }

    let sum: i32 = state.0.iter().sum::<i32>();
    println!("Sum of locations: {}", sum);
    sum
}

fn main() {
    let start = String::from("#.......##.###.#.#..##..##..#.#.###..###..##.#.#..##....#####..##.#.....########....#....##.#..##...");
    //let start = String::from("#..#.#..##......###...###");

    //sum_after_generations(start, 20);
    sum_after_generations(start, 1000);
}
