use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

use std::{i32::MAX, io::stdin, ops, time};

struct Input {
    pos: Vec<(i32, i32)>,
}

// op 1~9
fn convert_op(op: i32) -> (i32, i32) {
    let dx = (op - 1) % 3 - 1;
    let dy = (op - 1) / 3 - 1;
    (dx, dy)
}

use rand::prelude::*;
use std::f64;

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn distance(p1: Point, p2: Point) -> f64 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}

fn total_distance(points: &[Point], tour: &[usize]) -> f64 {
    let mut dist = 0.0;
    for i in 0..tour.len() {
        let j = (i + 1) % tour.len();
        dist += distance(points[tour[i]], points[tour[j]]);
    }
    dist
}

fn two_opt_swap(tour: &mut Vec<usize>, i: usize, k: usize) {
    tour[i..=k].reverse();
}

fn simulated_annealing(
    points: &[Point],
    initial_temp: f64,
    cooling_rate: f64,
    max_iter: usize,
) -> Vec<usize> {
    let mut rng = thread_rng();
    let n = points.len();
    let mut tour: Vec<usize> = (0..n).collect();
    tour.shuffle(&mut rng);

    let mut best_tour = tour.clone();
    let mut best_distance = total_distance(points, &tour);
    let mut temp = initial_temp;

    for _ in 0..max_iter {
        let i = rng.gen_range(0..n - 1);
        let k = rng.gen_range(i + 1..n);

        let mut new_tour = tour.clone();
        two_opt_swap(&mut new_tour, i, k);

        let new_distance = total_distance(points, &new_tour);
        if new_distance < best_distance
            || (rng.gen::<f64>() < ((best_distance - new_distance) / temp).exp())
        {
            eprintln!("{} {}", best_distance, new_distance);
            tour = new_tour;
            best_distance = new_distance;
            best_tour = tour.clone();
        }

        temp *= 1.0 - cooling_rate;
    }

    best_tour
}

fn eval(goal: (i32, i32), pos: (i32, i32), vel: (i32, i32)) -> i32 {
    let dx = goal.0 - pos.0;
    let dy = goal.1 - pos.1;
    dx * dx + dy * dy + (vel.0 * vel.0 + vel.1 * vel.1).pow(2)
}

fn main() {
    let mut input = Input { pos: vec![] };
    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        let buffer = buffer.trim();
        if buffer.is_empty() {
            break;
        }
        let mut iter = buffer.split_whitespace();
        let x = iter.next().unwrap().parse().unwrap();
        let y = iter.next().unwrap().parse().unwrap();
        input.pos.push((x, y));
    }

    // input.posをTSPで巡回する。初期点は(0, 0)からの最短距離の点
    let mut min_dist = MAX;
    let mut min_pos = (-1, -1);
    for i in 0..input.pos.len() {
        let dist = input.pos[i].0.pow(2) + input.pos[i].1.pow(2);
        if dist < min_dist {
            min_dist = dist;
            min_pos = (input.pos[i].0, input.pos[i].1);
        }
    }

    let mut points = input
        .pos
        .iter()
        .map(|&(x, y)| Point {
            x: (x - min_pos.0) as f64,
            y: (y - min_pos.1) as f64,
        })
        .collect::<Vec<_>>();

    let initial_temp = 100.0;
    let cooling_rate = 0.0001;
    let max_iter = 1000000;
    let tour = simulated_annealing(&points, initial_temp, cooling_rate, max_iter);

    let path = vec![(0, 0)]
        .into_iter()
        .chain(tour.into_iter().map(|i| {
            (
                points[i].x as i32 + min_pos.0,
                points[i].y as i32 + min_pos.1,
            )
        }))
        .collect::<Vec<_>>();

    let mut ans: Vec<i32> = vec![];
    for i in 0..path.len() - 1 {
        let cur = path[i];
        let next = path[i + 1];
        eprintln!(
            "turn: {} ({}, {}) -> ({}, {})",
            i, cur.0, cur.1, next.0, next.1
        );

        // curからnextに移動するためのビームサーチを行う
        #[derive(Debug)]
        struct State {
            pos: (i32, i32),
            vel: (i32, i32),
            score: i32,
            ops: Vec<i32>,
        }
        let mut beam = vec![State {
            pos: cur,
            vel: (0, 0),
            score: eval(next, cur, (0, 0)),
            ops: vec![],
        }];
        loop {
            let mut next_beam = vec![];
            for state in beam {
                for op in 1..=9 {
                    let (dx, dy) = convert_op(op);
                    let (vx, vy) = (state.vel.0 + dx, state.vel.1 + dy);
                    let next_pos = (state.pos.0 + vx, state.pos.1 + vy);
                    let next_score = eval(next, next_pos, (vx, vy));
                    let mut next_ops = state.ops.clone();
                    next_ops.push(op);
                    next_beam.push(State {
                        pos: next_pos,
                        vel: (vx, vy),
                        score: next_score,
                        ops: next_ops,
                    });
                }
            }
            next_beam.sort_by_key(|state| state.score);
            next_beam.truncate(10000);
            beam = next_beam;
            if beam[0].score == 0 {
                break;
            }
        }

        ans.extend(&beam[0].ops);
    }

    for op in &ans {
        print!("{}", op);
    }

    // simulate path
    let mut pos = (0, 0);
    let mut vel = (0, 0);
    for i in 0..ans.len() - 1 {
        let op = ans[i];
        let (dx, dy) = convert_op(op);
        vel = (vel.0 + dx, vel.1 + dy);
        pos = (pos.0 + vel.0, pos.1 + vel.1);
        eprintln!("turn: {} ({}, {})", i, pos.0, pos.1,);
    }
}
