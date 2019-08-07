use std::io::stdin;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::cell::RefCell;

struct Point {
    id: i32,
    x: i64,
    y: i64,
}

struct Edge {
    point1: i32,
    point2: i32,
    distance: i64,
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.distance.cmp(&other.distance).reverse())
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        self.point1 == other.point1
            && self.point2 == other.point2
            && self.distance == other.distance
    }
}

impl Eq for Edge {}

struct AutoDeleteBTree {
    v: RefCell<Vec<Edge>>,
    finished: RefCell<HashMap<i32, ()>>,
}

impl AutoDeleteBTree {
    pub fn new() -> AutoDeleteBTree {
        AutoDeleteBTree{
            v: RefCell::new(Vec::new()),
            finished: RefCell::new(HashMap::new())
        }
    }
    pub fn add_finished(&mut self, id: i32) {
        let mut mut_map = self.finished.borrow_mut();
        mut_map.insert(id, ());
    }

    pub fn add(&mut self, edge: Edge) {
        let mut mut_vec = self.v.borrow_mut();
        mut_vec.push(edge);
        let mut current_index = mut_vec.len() - 1;
        let mut parent_index = self.parent_id(current_index);
        while current_index != 0 {
            if mut_vec.get(current_index).unwrap().distance
                < mut_vec.get(parent_index).unwrap().distance {
                mut_vec.swap(current_index, parent_index);
            }
            current_index = parent_index;
            parent_index = self.parent_id(current_index)
        }
        if self.finished.borrow_mut().contains_key(&mut_vec.last().unwrap().point2) {
            mut_vec.pop();
        }
    }

    pub fn pop_while_valid(&mut self) {
        while !self.pop() {}
    }

    pub fn pop(&mut self) -> bool {
        let mut mut_vec = self.v.borrow_mut();
        let ref_map = self.finished.borrow();
        if !ref_map.contains_key(&mut_vec.first().unwrap().point2) {
            return true;
        }
        let last_index = mut_vec.len() - 1;
        mut_vec.swap(0, last_index);
        mut_vec.pop();
        let mut current_index = 0;
        loop {
            let left_child_index = self.left_child_id(current_index);
            let right_child_index = self.right_child_id(current_index);
            let current = mut_vec.get(current_index).unwrap().distance;
            let mut left_distance = std::i64::MAX;
            let mut right_distance = std::i64::MAX;
            match mut_vec.get(left_child_index) {
                Some(edge) => {
                    left_distance = edge.distance;
                },
                None => {
                    left_distance = std::i64::MAX;
                },
            }
            match mut_vec.get(right_child_index) {
                Some(edge) => {
                    right_distance = edge.distance;
                },
                None => {
                    right_distance = std::i64::MAX;
                },
            }
            if left_distance < current && left_distance <= right_distance {
                mut_vec.swap(current_index, left_child_index);
                current_index = left_child_index;
                continue
            }
            if right_distance < current && right_distance <= left_distance {
                mut_vec.swap(current_index, right_child_index);
                current_index = right_child_index;
                continue
            }
            break;
        }
        false
    }

    fn parent_id(&self, child_id: usize) -> usize {
        ((child_id as i32 - 1) / 2) as usize
    }

    fn left_child_id(&self, parent_id: usize) -> usize {
        (parent_id as i32 * 2 + 1) as usize
    }

    fn right_child_id(&self, parent_id: usize) -> usize {
        (parent_id as i32 * 2 + 2) as usize
    }

    pub fn min_id(&self) -> i32 {
        self.v.borrow()[0].point2
    }

    pub fn min_dist(&self) -> i64 {
        self.v.borrow()[0].distance
    }
}

fn calc(point_list: &Vec<Point>) -> i64 {
    let mut tree = AutoDeleteBTree::new();
    let mut unfinished: HashMap<i32, &Point> = point_list.iter()
        .map(|point| (point.id, point)).collect();
    let mut last_finished = unfinished.remove(&0).unwrap();
    let mut distance = 0;
    while unfinished.len() > 0 {
        tree.add_finished(last_finished.id);
        for point in &unfinished {
            tree.add(Edge{
                point1: last_finished.id,
                point2: point.1.id,
                distance: get_dist(last_finished, point.1)});
        }
        tree.pop_while_valid();
        distance += tree.min_dist();
        last_finished = unfinished.remove(&tree.min_id()).unwrap();
    }
    distance
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
