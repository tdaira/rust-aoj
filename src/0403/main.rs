use std::io::stdin;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::collections::binary_heap::BinaryHeap;
use std::cmp::Ordering;

struct Point {
    id: i64,
    x: i64,
    y: i64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.id == other.id
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.x.hash(state);
        self.y.hash(state);
    }
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
        self.point1 == other.point1
            && self.point2 == other.point2
            && self.distance == other.distance
    }
}

impl<'a> Eq for PointDistance<'a> {}

fn calc(point_list: &Vec<Point>) -> i64 {
    let mut dist_list = Vec::new();
    let mut point_map: HashMap<&Point, ()> =
        point_list.iter().map(|point| (point, ()) ).collect();
    let mut finished_map: HashMap<&Point, ()> = HashMap::new();
    let mut binary_heap = BinaryHeap::new();
    // Remove single key.
    let mut last_finished = &point_list[0];
    point_map.remove(last_finished).unwrap();
    finished_map.insert(last_finished, ());
    while !point_map.is_empty() {
        for unfinished in &point_map {
            let dist = get_dist(last_finished, unfinished.0);
            binary_heap.push(PointDistance{
                point1: last_finished,
                point2: unfinished.0,
                distance: dist});
        }
        let mut min_point = None;
        loop {
            let min = binary_heap.pop().unwrap();
            if !finished_map.contains_key(min.point2) {
                min_point = Some(min);
                break;
            }
        }
        let min_content = min_point.unwrap();
        last_finished = min_content.point2;
        dist_list.push(min_content.distance);
        finished_map.insert(min_content.point2, ());
        point_map.remove(min_content.point2);
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
    println!("{:?}", calc(&*point_list));
}
