use std::io::stdin;
use std::collections::binary_heap::BinaryHeap;
use std::cmp::Ordering;
use std::process::exit;
use std::collections::HashMap;

struct Point {
    id: i32,
    x: i64,
    y: i64,
}

struct PointDistance<'a> {
    point1: &'a Point,
    point2: &'a Point,
    distance: i64,
}

impl<'a> PartialOrd for PointDistance<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.distance.cmp(&other.distance).reverse())
    }
}

impl<'a> Ord for PointDistance<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

impl<'a> PartialEq for PointDistance<'a> {
    fn eq(&self, other: &PointDistance) -> bool {
        self.point1.id == other.point1.id
            && self.point2.id == other.point2.id
            && self.distance == other.distance
    }
}

impl<'a> Eq for PointDistance<'a> {}

fn calc(point_list: &Vec<Point>) -> i64 {
    let mut dist_list = Vec::new();
    let mut point_map: HashMap<i32, &Point> =
        point_list.iter().map(|point| (point.id, point) ).collect();
    let mut finished_map: HashMap<i32, &Point> = HashMap::new();
    let mut point2_map:HashMap<i32, PointDistance> = HashMap::new();
    // Remove single key.
    let mut last_finished = &point_list[0];
    point_map.remove(&last_finished.id).unwrap();
    finished_map.insert(last_finished.id, last_finished);
    while !point_map.is_empty() {
        point2_map.remove(&last_finished.id);
        for unfinished in &point_map {
            let dist = get_dist(last_finished, unfinished.1);
            let before = point2_map.remove(&unfinished.1.id);
            match before {
                Some(pd) => {
                    if pd.distance > dist {
                        point2_map.insert(
                            unfinished.1.id,
                            PointDistance {
                                point1: last_finished,
                                point2: unfinished.1,
                                distance: dist
                            });
                    } else {
                        point2_map.insert(
                            pd.point2.id,
                            pd
                        );
                    }
                },
                None => {
                    point2_map.insert(
                        unfinished.1.id,
                        PointDistance {
                            point1: last_finished,
                            point2: unfinished.1,
                            distance: dist
                        });
                },
            }
        }
        let mut binary_heap = BinaryHeap::new();
        for dist in &point2_map {
            binary_heap.push(dist.1);
        }
        let min_distance = binary_heap.pop().unwrap();
        last_finished = min_distance.point2;
        dist_list.push(min_distance.distance);
        finished_map.insert(min_distance.point2.id, min_distance.point2);
        point_map.remove(&min_distance.point2.id);
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
    let lines: i32 = s.trim().parse().unwrap();
    let point_list: &mut Vec<Point> = &mut Vec::new();
    for i in 0..lines {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        let vals: Vec<i64> = s.trim().split_whitespace()
            .map(|e| e.parse().unwrap()).collect();
        point_list.push(Point{ id: i, x: vals[0], y: vals[1]});
    }
    println!("{:?}", calc(&*point_list));
}
