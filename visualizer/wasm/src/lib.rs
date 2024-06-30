use svg::{
    self,
    node::element::{Circle, Group, Line, Title},
    Document,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn gen(seed: i32) -> String {
    let input_1 = include_str!("../../../problems/spaceship/1.txt");
    let input_2 = include_str!("../../../problems/spaceship/2.txt");
    let input_3 = include_str!("../../../problems/spaceship/3.txt");
    let input_4 = include_str!("../../../problems/spaceship/4.txt");
    let input_5 = include_str!("../../../problems/spaceship/5.txt");
    let input_6 = include_str!("../../../problems/spaceship/6.txt");
    let input_7 = include_str!("../../../problems/spaceship/7.txt");
    let input_8 = include_str!("../../../problems/spaceship/8.txt");
    let input_9 = include_str!("../../../problems/spaceship/9.txt");
    let input_10 = include_str!("../../../problems/spaceship/10.txt");
    let input_11 = include_str!("../../../problems/spaceship/11.txt");
    let input_12 = include_str!("../../../problems/spaceship/12.txt");
    let input_13 = include_str!("../../../problems/spaceship/13.txt");
    let input_14 = include_str!("../../../problems/spaceship/14.txt");
    let input_15 = include_str!("../../../problems/spaceship/15.txt");
    let input_16 = include_str!("../../../problems/spaceship/16.txt");
    let input_17 = include_str!("../../../problems/spaceship/17.txt");
    let input_18 = include_str!("../../../problems/spaceship/18.txt");
    let input_19 = include_str!("../../../problems/spaceship/19.txt");
    let input_20 = include_str!("../../../problems/spaceship/20.txt");
    let input_21 = include_str!("../../../problems/spaceship/21.txt");
    let input_22 = include_str!("../../../problems/spaceship/22.txt");
    let input_23 = include_str!("../../../problems/spaceship/23.txt");
    let input_24 = include_str!("../../../problems/spaceship/24.txt");
    let input_25 = include_str!("../../../problems/spaceship/25.txt");

    let inputs = vec![
        input_1, input_2, input_3, input_4, input_5, input_6, input_7, input_8, input_9, input_10,
        input_11, input_12, input_13, input_14, input_15, input_16, input_17, input_18, input_19,
        input_20, input_21, input_22, input_23, input_24, input_25,
    ];

    inputs[seed as usize].to_string()
}

#[wasm_bindgen(getter_with_clone)]
pub struct Ret {
    pub score: i64,
    pub err: String,
    pub svg: String,
}

fn parse_input(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let x = iter.next().unwrap().parse().unwrap();
            let y = iter.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect()
}

fn parse_output(output: &str) -> Vec<i32> {
    let mut collection = vec![];
    for c in output.chars() {
        if c.is_whitespace() {
            continue;
        }
        let n = c.to_digit(10).unwrap() as i32;
        collection.push(n);
    }
    collection
}

const COLOR_HOTTEST_HSLA: &str = "hsl(349, 100%, 56%, 0.8)"; // #ff1e46 * 0.8
const COLOR_COOLEST_HSLA: &str = "hsl(210, 100%, 56%, 0.8)"; // #1e90ff * 0.8

#[derive(Debug, Clone, Copy)]
struct HslaColor {
    h: f64,
    s: f64,
    l: f64,
    a: f64,
}

fn decode_to_hsla(s: &str) -> HslaColor {
    let s2 = s
        .trim_start_matches("hsl(")
        .trim_end_matches(')')
        .split(',')
        .collect::<Vec<_>>();
    let h = s2[0].parse::<f64>().unwrap();
    let s = s2[1].trim().trim_end_matches('%').parse::<f64>().unwrap();
    let l = s2[2].trim().trim_end_matches('%').parse::<f64>().unwrap();
    let a = s2[3].trim().parse::<f64>().unwrap();
    HslaColor { h, s, l, a }
}

fn encode_to_hsla(c: HslaColor) -> String {
    format!("hsla({}, {}%, {}%, {})", c.h, c.s, c.l, c.a)
}

fn get_colors(cnt: usize) -> Vec<HslaColor> {
    let mut colors = vec![];
    let hottest = decode_to_hsla(COLOR_HOTTEST_HSLA);
    let coolest = decode_to_hsla(COLOR_COOLEST_HSLA);
    let mut h = coolest.h;
    let mut s = coolest.s;
    let mut l = coolest.l;
    let mut a = coolest.a;
    let dh = (coolest.h - hottest.h + 360.0) / (cnt as f64);
    let ds = (hottest.s - coolest.s) / (cnt as f64);
    let dl = (hottest.l - coolest.l) / (cnt as f64);
    let da = (hottest.a - coolest.a) / (cnt as f64);
    for _ in 0..cnt {
        colors.push(HslaColor { h, s, l, a });
        h = (h - dh) % 360.0;
        s += ds;
        l += dl;
        a += da;
    }
    colors
}

#[wasm_bindgen]
pub fn vis(_input: String, _output: String, turn: usize) -> Ret {
    let input = parse_input(&_input);
    let output = parse_output(&_output);

    // inputの最大最小を求める
    let mut min_x = std::i32::MAX;
    let mut max_x = std::i32::MIN;
    let mut min_y = std::i32::MAX;
    let mut max_y = std::i32::MIN;
    for &(x, y) in &input {
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    let mut svg = Document::new().set("viewBox", (0, 0, 1000, 1000));
    svg = svg
        .set("x", 0)
        .set("y", 0)
        .set("width", 600)
        .set("height", 600)
        .set("fill", "white");

    // スケールする
    let max_range = max_x
        .abs()
        .max(max_y.abs())
        .max(min_x.abs())
        .max(min_y.abs());
    let width = 1000;
    let height = 1000;
    let scale = width as f64 / (max_range * 2) as f64;
    let scale_y = height as f64 / (max_range * 2) as f64;

    // 軸を描画
    svg = svg.add(
        Line::new()
            .set("x1", 0)
            .set("y1", height / 2)
            .set("x2", width)
            .set("y2", height / 2)
            .set("stroke", "black")
            .set("stroke-width", 1),
    );

    svg = svg.add(
        Line::new()
            .set("x1", width / 2)
            .set("y1", 0)
            .set("x2", width / 2)
            .set("y2", height)
            .set("stroke", "black")
            .set("stroke-width", 1),
    );

    // outputを描画
    let mut cx = 0;
    let mut cy = 0;
    let mut vx = 0;
    let mut vy = 0;

    let mut visited = vec![];
    let mut max_vel = 0;
    for i in 0..turn {
        let (dx, dy) = match output[i] {
            1 => (-1, -1),
            2 => (0, -1),
            3 => (1, -1),
            4 => (-1, 0),
            5 => (0, 0),
            6 => (1, 0),
            7 => (-1, 1),
            8 => (0, 1),
            9 => (1, 1),
            _ => unreachable!(),
        };

        vx += dx;
        vy += dy;

        if (vx * vx + vy * vy) > max_vel {
            max_vel = vx * vx + vy * vy;
        }

        cx += vx;
        cy += vy;

        visited.push((cx, cy, vx, vy));
    }

    svg = svg.add(
        Circle::new()
            .set("cx", (cx + max_range) as f64 * scale)
            .set("cy", (cy + max_range) as f64 * scale_y)
            .set("r", 5)
            .set("fill", "red"),
    );

    // inputを描画
    for i in 0..input.len() {
        let (x, y) = input[i];
        if visited.iter().any(|&(x2, y2, _, _)| x == x2 && y == y2) {
            continue;
        }
        let scaled_x = (x + max_range) as f64 * scale;
        let scaled_y = (y + max_range) as f64 * scale_y;
        // hoverしたらiと座標を表示するようにする
        svg = svg.add(
            Group::new()
                .add(Title::new().add(svg::node::Text::new(format!("{}: ({}, {})", i, x, y))))
                .add(
                    Circle::new()
                        .set("cx", scaled_x)
                        .set("cy", scaled_y)
                        .set("r", 5)
                        .set("fill", "blue"),
                ),
        )
    }

    // visitedを描画
    let colors = get_colors(max_vel as usize);
    for i in 0..visited.len() {
        let (x, y, vx, vy) = visited[i];
        let beforex = (x + max_range - vx) as f64 * scale;
        let beforey = (y + max_range - vy) as f64 * scale_y;
        let afterx = (x + max_range) as f64 * scale;
        let aftery = (y + max_range) as f64 * scale_y;
        let color = colors[((vx * vx + vy * vy) as usize - 1).min(colors.len() - 1)];
        svg = svg.add(
            Line::new()
                .set("x1", beforex)
                .set("y1", beforey)
                .set("x2", afterx)
                .set("y2", aftery)
                .set("stroke", encode_to_hsla(color))
                .set("stroke-width", 2),
        );
    }

    Ret {
        score: turn as i64,
        err: "".to_string(),
        svg: svg.to_string(),
    }
}

#[wasm_bindgen]
pub fn get_max_turn(_input: String, _output: String) -> usize {
    let output = parse_output(&_output);
    output.len()
}
