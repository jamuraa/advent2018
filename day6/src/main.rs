use std::{
    cmp::max,
    collections::HashMap,
    fs::File,
    io::{self, prelude::*, BufReader},
};

struct Point {
    id: char,
    x: i32,
    y: i32,
}

fn manhattan(a: &Point, b: &Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn numbers_in_string(s: &str) -> Vec<i32> {
    s.split(|x| !char::is_numeric(x))
        .filter(|x| !x.is_empty())
        .map(|num_str| num_str.parse().unwrap())
        .collect()
}

fn id_to_char(id: i32) -> char {
    if id < 26 {
        (id + 97) as u8 as char
    } else {
        (id - 26 + 65) as u8 as char
    }
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut lines = reader.lines();
    let mut points: Vec<Point> = Vec::new();
    let mut count = 0;
    let mut largest_side = 0;

    while let Some(Ok(pt_txt)) = lines.next() {
        let nums = numbers_in_string(&pt_txt);
        largest_side = max(largest_side, nums[0]);
        largest_side = max(largest_side, nums[1]);
        points.push(Point {
            id: id_to_char(count),
            x: nums[0],
            y: nums[1],
        });
        count += 1;
    }

    // For each point id, the list of points that are closest to that point.
    let mut closest_points: HashMap<char, Vec<Point>> = HashMap::new();

    let board_min = 0;
    let board_max = largest_side + 1;

    let safe_sum_dist = 10000;
    let mut safe_points = 0;

    for board_y in board_min..board_max + 1 {
        for board_x in board_min..board_max + 1 {
            let board_pt = Point {
                id: '.',
                x: board_x,
                y: board_y,
            };
            let mut closest_id = None;
            let mut closest_dist = 2 * (board_max - board_min);
            let mut sum_dist = 0;
            for point in &points {
                let dist = manhattan(&board_pt, point);
                sum_dist += dist;
                if dist < closest_dist {
                    closest_id = Some(point.id);
                    closest_dist = dist;
                } else if closest_id.is_some() && dist == closest_dist {
                    closest_id = None;
                    closest_dist = dist;
                }
            }
            if sum_dist < safe_sum_dist {
                safe_points += 1;
            }
            if let Some(id) = closest_id {
                let entry = closest_points.entry(id).or_insert(Vec::new());
                entry.push(board_pt);
            }
        }
    }

    let mut largest_area_not_inf_id = None;
    let mut largest_area_not_inf_points = 0;
    'outer: for (id, points) in closest_points {
        for point in &points {
            // If it touches an edge then it is infinite and doesn't count.
            if point.x == board_min
                || point.y == board_min
                || point.x == board_max
                || point.y == board_max
            {
                continue 'outer;
            }
        }
        if points.len() > largest_area_not_inf_points {
            largest_area_not_inf_id = Some(id);
            largest_area_not_inf_points = points.len();
        }
    }

    println!(
        "The largest space not infinite is id {} with {} spaces",
        largest_area_not_inf_id.unwrap(),
        largest_area_not_inf_points
    );
    println!(
        "{} points less than {} sum manhattan from all points",
        safe_points, safe_sum_dist
    );
    Ok(())
}
