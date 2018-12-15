use std::{
    collections::VecDeque,
    fs::File,
    io::{self, prelude::*, BufReader},
};

#[derive(PartialEq)]
enum CartTurnState {
    Left,
    GayilyForward,
    Right
}

impl CartTurnState {
    fn next(&self) -> Self {
        match self {
            CartTurnState::Left => CartTurnState::GayilyForward,
            CartTurnState::GayilyForward => CartTurnState::Right,
            CartTurnState::Right => CartTurnState::Left,
        }
    }
}

struct Cart {
    loc: (usize, usize),
    direction: char,
    next_turn: CartTurnState,
    last_tick: i32,
}

impl Cart {
    fn new(x: usize, y: usize, direction: char) -> Cart {
        Cart { loc: (x, y), direction, next_turn: CartTurnState::Left,
        last_tick: 0 }
    }

    fn cart_char(c: &char) -> bool {
        match c { 
            '^' | '>' | '<' | 'v' => true,
            _ => false
        }
    }

    fn to_track(c: &char) -> char {
        match c {
            '^' | 'v' => '|',
            '<' | '>' => '-',
            r => *r,
        }
    }

    // Makes the cart travel along the map one step
    // Returns the new location of the cart.
    fn go(&mut self, map: &Vec<Vec<char>>) -> (usize, usize) {
        // Move!
        self.loc = match self.direction {
            '>' => (self.loc.0 + 1, self.loc.1),
            '<' => (self.loc.0 - 1, self.loc.1),
            'v' => (self.loc.0, self.loc.1 + 1),
            '^' => (self.loc.0, self.loc.1 - 1),
            _ => unreachable!(),
        };
        // Change direction now
        self.direction = match (map[self.loc.1][self.loc.0], self.direction) {
            ('-', _) | ('|', _) => self.direction,
            ('\\', '<') => '^',
            ('\\', 'v') => '>',
            ('\\', '>') => 'v',
            ('\\', '^') => '<',
            ('/', '<') => 'v',
            ('/', '^') => '>',
            ('/', '>') => '^',
            ('/', 'v') => '<',
            ('+', c) => {
                let dir = match (&self.next_turn, c) {
                    (CartTurnState::Left, '^') => '<',
                    (CartTurnState::Left, '<') => 'v',
                    (CartTurnState::Left, 'v') => '>',
                    (CartTurnState::Left, '>') => '^',
                    (CartTurnState::GayilyForward, c) => c,
                    (CartTurnState::Right, '^') => '>',
                    (CartTurnState::Right, '<') => '^',
                    (CartTurnState::Right, 'v') => '<',
                    (CartTurnState::Right, '>') => 'v',
                    _ => unreachable!(),
                };
                self.next_turn = self.next_turn.next();
                dir
            },
            _ => unreachable!(),
        };
        self.last_tick += 1;
        self.loc.clone()
    }
}

fn print_map(map: &Vec<Vec<char>>, carts: &VecDeque<Cart>) {
    let rows = map.len();
    let cols = map[0].len();
    for y in 0..rows {
        for x in 0..cols {
            let c = match carts.iter().find(|c| c.loc == (x, y)) {
                Some(c) => c.direction,
                None => map[y][x],
            };
            print!("{}", c);
        }
        println!("");
    }
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut lines = reader.lines();

    let mut map: Vec<Vec<char>> = Vec::new();

    let mut columns: u16 = 0;

    let mut carts: VecDeque<Cart> = VecDeque::new();

    let mut rows = 0;
    while let Some(Ok(line)) = lines.next() {
        let mut this_carts: VecDeque<Cart> = line.match_indices(|c| Cart::cart_char(&c) ).map(|(ind, ch)| Cart::new(ind, rows, ch.chars().next().unwrap())).collect();
        carts.append(&mut this_carts);
        let line = line.chars().map(|c| Cart::to_track(&c)).collect();
        map.push(line);
        rows += 1;
    }

    let cols = map[0].len();

    let mut tick: i32 = -1;
    loop {
        // If we are at the next tick, sort them in order.
        if carts.front().unwrap().last_tick != tick {
            tick += 1;
            if carts.len() == 1 {
                println!("Only one cart left at tick {} at {:?}", tick, carts.front().unwrap().loc);
                break;
            }
            //print_map(&map, &carts);
            let (front, back) = carts.as_mut_slices();
            front.sort_unstable_by(|a, b| a.loc.cmp(&b.loc));
            back.sort_unstable_by(|a, b| a.loc.cmp(&b.loc));
        }
        let mut moving_cart = carts.pop_front().unwrap();
        let new_loc = moving_cart.go(&map);
        // Did we collide
        let mut idx_remove = None;
        if let Some((idx, c)) = carts.iter().enumerate().find(|(_, c)| c.loc == new_loc) {
            println!("Collision at tick {} at {:?}", moving_cart.last_tick, moving_cart.loc);
            idx_remove = Some(idx);
        }
        if let Some(idx) = idx_remove.take() {
            carts.remove(idx);
        } else {
            carts.push_back(moving_cart);
        }
    }
    Ok(())
}
