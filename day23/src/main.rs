use std::{
    cmp::{max, min},
    fs::File,
    io::{self, prelude::*, BufReader},
    str::FromStr,
};

#[derive(Clone, Debug)]
struct NanoBot {
    pos: (i64, i64, i64),
    radius: i64,
}

fn pair_manhattan(x: &(i64, i64, i64), y: &(i64, i64, i64)) -> i64 {
    (x.0 - y.0).abs() + (x.1 - y.1).abs() + (x.2 - y.2).abs()
}

impl NanoBot {
    fn new(x: i64, y: i64, z: i64, radius: i64) -> NanoBot {
        NanoBot {
            pos: (x, y, z),
            radius,
        }
    }

    fn dist(&self, other: &(i64, i64, i64)) -> i64 {
        pair_manhattan(&self.pos, other)
    }

    fn in_radius(&self, other: &NanoBot) -> bool {
        self.in_range(&other.pos)
    }

    fn in_range(&self, other: &(i64, i64, i64)) -> bool {
        self.dist(other) <= self.radius
    }

    fn scaled(&self, scale: i64) -> NanoBot {
        NanoBot {
            pos: (self.pos.0 / scale, self.pos.1 / scale, self.pos.2 / scale),
            radius: self.radius / scale,
        }
    }
}

fn main() -> io::Result<()> {
    let mut bots: Vec<NanoBot> = Vec::new();

    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let mut lines = reader.lines();
    while let Some(Ok(line)) = lines.next() {
        let nums: Vec<i64> = numbers_in_string(&line);
        bots.push(NanoBot::new(nums[0], nums[1], nums[2], nums[3]));
    }

    let range = 55;
    let mut scale = 1000000;
    let mut center = (0, 0, 0);
    while scale != 0 {
        let bots_scaled: Vec<NanoBot> = bots.iter().map(|x| x.scaled(scale)).collect();
        println!(
            "Scale: {} Range: ({}..{}) ({}..{}) ({}..{})",
            scale,
            center.0 - range,
            center.0 + range,
            center.1 - range,
            center.1 + range,
            center.2 - range,
            center.2 + range
        );

        let mut max_rad = 0;
        let mut max_rad_idx = 0;

        for (idx, bot) in bots_scaled.iter().enumerate() {
            if bot.radius > max_rad {
                max_rad_idx = idx;
                max_rad = bot.radius;
            }
        }

        println!(
            "Max radius bot is {} : {:?}",
            max_rad_idx, bots_scaled[max_rad_idx]
        );

        let in_radius: Vec<&NanoBot> = bots_scaled
            .iter()
            .filter(|x| bots_scaled[max_rad_idx].in_radius(x))
            .collect();
        println!("There are {} bots in radius of that bot", in_radius.len());

        let mut best_point = (0, 0, 0);
        let mut best_point_bots = 0;
        let mut min_dist_from_origin = std::i64::MAX;
        for x in center.0 - range..center.0 + range {
            for y in center.1 - range..center.1 + range {
                for z in center.2 - range..center.2 + range {
                    let point = (x, y, z);
                    let dist = pair_manhattan(&point, &(0, 0, 0));
                    let in_radius: Vec<&NanoBot> =
                        bots_scaled.iter().filter(|x| x.in_range(&point)).collect();
                    let bots = in_radius.len();
                    if (bots > best_point_bots)
                        || ((bots == best_point_bots) && (dist < min_dist_from_origin))
                    {
                        best_point = point;
                        best_point_bots = bots;
                        min_dist_from_origin = dist;
                        println!(
                            "{} bots in range of {:?} which is {} from the origin",
                            best_point_bots, best_point, min_dist_from_origin
                        );
                    }
                }
            }
        }
        println!(
            "Scale: {} {:?} is {} away from origin",
            scale,
            best_point,
            pair_manhattan(&best_point, &(0, 0, 0))
        );
        // Make scale bigger
        scale /= 10;
        center = (
            best_point.0 * 10 + 5,
            best_point.1 * 10 + 5,
            best_point.2 * 10 + 5,
        );
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan() {
        assert_eq!(0, pair_manhattan(&(0, 0, 0), &(0, 0, 0)));
        assert_eq!(1, pair_manhattan(&(1, 0, 0), &(0, 0, 0)));
        assert_eq!(4, pair_manhattan(&(4, 0, 0), &(0, 0, 0)));
        assert_eq!(2, pair_manhattan(&(0, 2, 0), &(0, 0, 0)));
        assert_eq!(5, pair_manhattan(&(0, 5, 0), &(0, 0, 0)));
        assert_eq!(3, pair_manhattan(&(0, 0, 3), &(0, 0, 0)));
        assert_eq!(3, pair_manhattan(&(1, 1, 1), &(0, 0, 0)));
        assert_eq!(4, pair_manhattan(&(1, 1, 2), &(0, 0, 0)));
        assert_eq!(5, pair_manhattan(&(1, 3, 1), &(0, 0, 0)));
    }

}
