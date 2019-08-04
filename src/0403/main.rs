use std::io::stdin;

struct Point {
    x: i64,
    y: i64,
}

fn calc(point_list: &mut Vec<Point>) -> i64 {
    let mut dist_list = Vec::new();
    let mut point_list_tmp: Vec<&Point> = point_list.iter().collect();
    for finish_point in 1..point_list_tmp.len() {
        let mut min: i64 = std::i64::MAX;
        let mut unfinished_index  = 0;
        for i in 0..finish_point {
            let finished = point_list_tmp[i];
            for j in finish_point..point_list_tmp.len() {
                let unfinished = point_list_tmp[j];
                let dist = get_dist(finished, unfinished);
                if min > dist {
                    min = dist;
                    unfinished_index = j;
                }
            }
        }
        dist_list.push(min);
        point_list_tmp.swap(finish_point, unfinished_index);
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
    for _ in 0..lines {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        let vals: Vec<i64> = s.trim().split_whitespace()
            .map(|e| e.parse().unwrap()).collect();
        point_list.push(Point{ x: vals[0], y: vals[1]});
    }
    println!("{:?}", calc(point_list));
}
