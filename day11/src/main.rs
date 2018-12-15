use std::collections::HashMap;

fn power_lvl(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x + 10;
    let mut power = rack_id * y;
    power += serial;
    power = power * rack_id;
    let hund_digit = power / 100 % 10;
    hund_digit - 5
}

fn power_grid(
    top_left_x: i32,
    top_left_y: i32,
    size: i32,
    serial: i32,
    memo: &mut HashMap<(i32, i32, i32), i32>,
) -> i32 {
    if let Some(size) = memo.get(&(top_left_x, top_left_y, size)) {
        return *size;
    }
    let result = match size {
        1 => power_lvl(top_left_x, top_left_y, serial),
        2 => {
            power_lvl(top_left_x, top_left_y, serial)
                + power_lvl(top_left_x + 1, top_left_y, serial)
                + power_lvl(top_left_x, top_left_y + 1, serial)
                + power_lvl(top_left_x + 1, top_left_y + 1, serial)
        }
        _ => {
            let tl = memo.get(&(top_left_x, top_left_y, size - 1)).unwrap();
            let br = memo
                .get(&(top_left_x + 1, top_left_y + 1, size - 1))
                .unwrap();
            let tr = power_lvl(top_left_x + size - 1, top_left_y, serial);
            let bl = power_lvl(top_left_x, top_left_y + size - 1, serial);
            let center = memo
                .get(&(top_left_x + 1, top_left_y + 1, size - 2))
                .unwrap();
            tl + br + tr + bl - center
        }
    };
    memo.insert((top_left_x, top_left_y, size), result);
    result
}

fn main() {
    let serial = 4172;

    let mut max_grid = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_size = 0;

    let mut memo = HashMap::new();

    for size in 1..300 {
        for topleft_x in 1..(302 - size) {
            for topleft_y in 1..(302 - size) {
                let grid = power_grid(topleft_x, topleft_y, size, serial, &mut memo);
                if grid > max_grid {
                    max_x = topleft_x;
                    max_y = topleft_y;
                    max_size = size;
                    max_grid = grid;
                    println!(
                        "Grid at {},{} size {} has {} power",
                        topleft_x, topleft_y, size, grid
                    );
                }
            }
        }
    }

    println!(
        "Grid at {},{} size {} has {} power",
        max_x, max_y, max_size, max_grid
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_level() {
        assert_eq!(4, power_lvl(3, 5, 8));
        assert_eq!(-5, power_lvl(122, 79, 57));
        assert_eq!(0, power_lvl(217, 196, 39));
        assert_eq!(4, power_lvl(101, 153, 71));
    }
}
