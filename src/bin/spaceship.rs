use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

use std::{i64::MAX, io::stdin, ops, time};

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

// 焼きなまし
fn simulated_annealing(
    initial_tour: &[usize],
    points: &[Point],
    initial_temp: f64,
    cooling_rate: f64,
    max_iter: f64,
) -> Vec<usize> {
    let mut rng = thread_rng();
    let n = points.len();
    // let mut tour: Vec<usize> = (0..n).collect();
    // tour.shuffle(&mut rng);
    let mut tour = initial_tour.to_vec();

    let mut best_tour = tour.clone();
    let mut best_distance = total_distance(points, &tour);
    let mut temp = initial_temp;

    for t in 0..max_iter as usize {
        let i = rng.gen_range(0..n - 1);
        let k = rng.gen_range(i + 1..n);

        let mut new_tour = tour.clone();
        two_opt_swap(&mut new_tour, i, k);

        let new_distance = total_distance(points, &new_tour);
        if new_distance < best_distance
            || (rng.gen::<f64>() < ((best_distance - new_distance) / temp).exp())
        {
            eprintln!("t: {} score: {}", t + 1, new_distance);
            tour = new_tour;
            best_distance = new_distance;
            best_tour = tour.clone();
        }

        temp *= 1.0 - cooling_rate;
    }

    best_tour
}

// 貪欲
fn greedy(points: &[Point], from: (i32, i32)) -> Vec<usize> {
    let n = points.len();
    let mut tour = vec![];
    let mut cur = from;
    let mut used = vec![false; n];
    for _ in 0..n {
        let mut best = 0;
        let mut best_dist = f64::INFINITY;
        for i in 0..n {
            if used[i] {
                continue;
            }
            let dist = distance(
                points[i],
                Point {
                    x: cur.0 as f64,
                    y: cur.1 as f64,
                },
            );
            if dist < best_dist {
                best = i;
                best_dist = dist;
            }
        }
        tour.push(best);
        used[best] = true;
        cur = (points[best].x as i32, points[best].y as i32);
    }
    tour
}

fn eval(goal: (i32, i32), pos: (i32, i32), vel: (i32, i32)) -> i32 {
    let dx = goal.0 - pos.0;
    let dy = goal.1 - pos.1;
    let (dx, dy) = (dx as f64, dy as f64);
    let (vx, vy) = (vel.0 as f64, vel.1 as f64);
    // 速度を0まで減速して行った時にかかる時間
    let (stop_dist_vx, stop_dist_vy) = (vx * (vx + 1.0) / 2.0, vy * (vy + 1.0) / 2.0);
    (dx * dx + dy * dy + (((stop_dist_vx - dx).powi(2) + (stop_dist_vy - dy).powi(2).sqrt()))) as i32
    // (dx * dx + dy * dy + vx * vx + vy * vy).sqrt() as i32
    // (dx * dx + dy * dy + (vx * vx + vy * vy).powf(2.0)).sqrt() as i32
    // (dx * dx + dy * dy + (vx * vx + vy * vy).powf(3.0)).sqrt() as i32
    // dx * dx + dy * dy + (vel.0 * vel.0 + vel.1 * vel.1).pow(4)
    // dx * dx + dy * dy + ((vel.0 * vel.0 + vel.1 * vel.1) as f64).sqrt() as i32
}

fn main() {
    // let initial_temp = 1e1;
    // let cooling_rate = 1e-3 * 5.0;
    // let max_iter = 1e5;
    // let through_speed_limit = 1e8;
    // let beam_width = 1e3;

    let initial_temp = 1e4;
    let cooling_rate = 1e-4;
    let max_iter = 1e5;
    let through_speed_limit = 1e8;
    let beam_width = 1e4;

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
    let start_pos: (i32, i32) = (0, 0);
    let mut min_dist = MAX;
    let mut min_pos = (-1, -1);
    for i in 0..input.pos.len() {
        let dist = ((input.pos[i].0 - start_pos.0) as i64).pow(2)
            + ((input.pos[i].1 - start_pos.1) as i64).pow(2);
        if dist < min_dist {
            min_dist = dist;
            min_pos = (input.pos[i].0, input.pos[i].1);
        }
    }

    eprintln!("min_pos: {:?}", min_pos);

    let mut points = input
        .pos
        .iter()
        .map(|&(x, y)| Point {
            x: x as f64,
            y: y as f64,
        })
        .collect::<Vec<_>>();

    let greedy_tour = greedy(&points, start_pos);
    let tour = simulated_annealing(&greedy_tour, &points, initial_temp, cooling_rate, max_iter);
    // let tour = greedy_tour.clone();
    // let tour = greedy(&points, start_pos);

    let path = vec![]
        .into_iter()
        .chain(
            tour.into_iter()
                .map(|i| (points[i].x as i32, points[i].y as i32)),
        )
        .collect::<Vec<_>>();

    let greedy_path = vec![]
        .into_iter()
        .chain(
            greedy_tour
                .iter()
                .map(|&i| (points[i].x as i32, points[i].y as i32)),
        )
        .collect::<Vec<_>>();

    // 始まりのindexはmin_posとなるようにする
    let mut start_idx = 0;
    for i in 0..path.len() {
        if path[i] == min_pos {
            start_idx = i;
            break;
        }
    }

    // 一番近いところから右回りになるように
    let new_path_cand_1 = vec![&start_pos]
        .into_iter()
        .chain(path[start_idx..].iter())
        .chain(path[..start_idx].iter())
        .collect::<Vec<_>>();

    // 一番近いところから左回りになるように
    let new_path_cand_2 = vec![&start_pos]
        .into_iter()
        .chain(path[..start_idx].iter().rev())
        .chain(path[start_idx..].iter().rev())
        .collect::<Vec<_>>();

    let greedy_path_cand_1 = vec![&start_pos]
        .into_iter()
        .chain(greedy_path.iter())
        .collect::<Vec<_>>();

    // どっちの移動距離が短いか
    let cand1_dist = new_path_cand_1
        .windows(2)
        .map(|w| (w[0].0 - w[1].0).abs() + (w[0].1 - w[1].1).abs())
        .sum::<i32>();
    eprintln!("cand1: {}", cand1_dist);

    let cand2_dist = new_path_cand_2
        .windows(2)
        .map(|w| (w[0].0 - w[1].0).abs() + (w[0].1 - w[1].1).abs())
        .sum::<i32>();
    eprintln!("cand2: {}", cand2_dist);

    let greedy_dist = greedy_path_cand_1
        .windows(2)
        .map(|w| (w[0].0 - w[1].0).abs() + (w[0].1 - w[1].1).abs())
        .sum::<i32>();
    eprintln!("greedy: {}", greedy_dist);

    let new_path = vec![
        (cand1_dist, new_path_cand_1),
        (cand2_dist, new_path_cand_2),
        (greedy_dist, greedy_path_cand_1),
    ]
    .into_iter()
    .min_by_key(|(dist, _)| *dist)
    .unwrap()
    .1;

    let mut ans: Vec<i32> = vec![];
    let mut vel = (0, 0);
    for i in 0..new_path.len() - 1 {
        let cur = *new_path[i];
        let next = *new_path[i + 1];
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
            vel,
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
                    let mut next_ops = state.ops.clone();
                    next_ops.push(op);
                    let next_score = eval(next, next_pos, (vx, vy));
                    next_beam.push(State {
                        pos: next_pos,
                        vel: (vx, vy),
                        score: next_score,
                        ops: next_ops,
                    });
                }
            }
            next_beam.sort_by_key(|state| state.score);
            next_beam.truncate(beam_width as usize);
            beam = next_beam;
            // if beam[0].pos == next {
            eprintln!(
                "pos: {:?} vel: {:?} score: {} width: {}",
                beam[0].pos,
                beam[0].vel,
                beam[0].score,
                beam.len()
            );
            // if beam[0].pos == next
            //     && ((beam[0].vel.0.pow(2) + beam[0].vel.1.pow(2)) as f64).sqrt()
            //         <= through_speed_limit
            // {
            //     vel = beam[0].vel;
            //     break;
            // }
            if beam[0].score == 0 {
                vel = beam[0].vel;
                break;
            }
        }

        ans.extend(&beam[0].ops);
    }

    print!("solve spaceshipX ");
    for op in &ans {
        print!("{}", op);
        eprint!("{}", op);
    }
    println!();
    eprintln!();
}
