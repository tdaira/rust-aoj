use std::io::stdin;
use std::collections::HashMap;

struct Point {
    id: i64,
    x: i64,
    y: i64,
}

fn calc(point_list: &mut Vec<Point>) -> i64 {
    let mut dist_list = Vec::new();
    let mut point_list_map: HashMap<i64, &Point> =
        point_list.iter().map(|point| (point.id, point) ).collect();
    let mut finished_list = Vec::new();
    // Remove single key.
    let point = point_list_map.remove(&point_list[0].id).unwrap();
    finished_list.push(point);
    while !point_list_map.is_empty() {
        let mut min: i64 = std::i64::MAX;
        let mut min_point_id = None;
        for finished in &finished_list {
            for unfinished in point_list_map.iter().map(|t| t.1) {
                let dist = get_dist(finished, unfinished);
                if min > dist {
                    min = dist;
                    min_point_id = Option::Some(unfinished.id);
                }
            }
        }
        dist_list.push(min);
        finished_list.push(point_list_map[&min_point_id.unwrap()]);
        point_list_map.remove(&min_point_id.unwrap());
    }
    dist_list.iter().sum()
}

fn get_dist(p1: &Point, p2: &Point) -> i64 {
    let x_diff = (p1.x - p2.x).abs();
    let y_diff = (p1.y - p2.y).abs();
    if x_diff > y_diff {
        x_diff
    } else {
        y_diff
    }
}

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let lines: i64 = s.trim().parse().unwrap();
    let point_list: &mut Vec<Point> = &mut Vec::new();
    for i in 0..lines {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        let vals: Vec<i64> = s.trim().split_whitespace()
            .map(|e| e.parse().unwrap()).collect();
        point_list.push(Point{ id: i, x: vals[0], y: vals[1]});
    }
    println!("{:?}", calc(point_list));
}
