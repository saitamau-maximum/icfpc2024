use std::{collections::HashMap, fmt::Debug};

use super::tokenizer::{Direction, Operator, Token};

struct Emulator {
    board: Vec<Vec<Token>>,
    submit_pos: (usize, usize),
    tick_count: usize,
    history: Vec<Vec<Vec<Token>>>,
}

const directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const redirect_map: [(Direction, (isize, isize)); 4] = [
    (Direction::Up, (-1, 0)),
    (Direction::Down, (1, 0)),
    (Direction::Left, (0, -1)),
    (Direction::Right, (0, 1)),
];

impl Emulator {
    pub fn new(board: Vec<Vec<Token>>) -> Emulator {
        let submit_pos = board
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().filter_map(move |(j, cell)| {
                    if let Token::Operator(Operator::Submit) = cell {
                        Some((i, j))
                    } else {
                        None
                    }
                })
            })
            .next()
            .unwrap();
        let history = vec![board.clone()];
        Emulator {
            board,
            submit_pos,
            tick_count: 0,
            history,
        }
    }

    pub fn tick(&mut self) -> bool {
        self.tick_count += 1;
        let (new_board, rollback) = self.peek();
        self.board = new_board;
        if rollback > 0 {
            self.board = self.history.pop().unwrap();
            self.tick_count -= 1;
            return false;
        }
        self.history.push(self.board.clone());
        self.is_finished()
    }

    fn is_finished(&self) -> bool {
        let (i, j) = self.submit_pos;
        if let Token::Integer(_) = self.board[i][j] {
            return true;
        }
        false
    }

    // 詰みかどうかを判定する関数
    fn is_stuck(&self) -> bool {
        // 次の状態が同じなら詰み
        let (next_board, _) = self.peek();
        next_board == self.board
    }

    pub fn peek(&self) -> (Vec<Vec<Token>>, usize) {
        let mut new_board = self.board.clone();
        // 更新項目を計算
        let mut clear_pos = vec![];
        let mut updates = HashMap::new();
        {
            let mut number_pos = vec![];
            // number_posを初期化
            for (i, row) in self.board.iter().enumerate() {
                for (j, cell) in row.iter().enumerate() {
                    if let Token::Integer(_) = cell {
                        number_pos.push((i, j));
                    }
                }
            }
            for (i, j) in number_pos {
                // 周り4マスにOperatorがあるか探索、あれば更新
                for (dx, dy) in directions.iter() {
                    let (next_x, next_y) = (i as isize + dx, j as isize + dy);
                    if next_x < 0
                        || next_x >= self.board.len() as isize
                        || next_y < 0
                        || next_y >= self.board[0].len() as isize
                    {
                        continue;
                    }
                    if let Token::Operator(Operator::Redirect(dir)) =
                        &self.board[next_x as usize][next_y as usize]
                    {
                        // もしredirect_mapと(dx, dy)が一致していれば、その方向にジャンプ
                        let (_, redirect) = redirect_map.iter().find(|(d, _)| d == dir).unwrap();
                        let (redirect_x, redirect_y) = redirect;
                        if (dx, dy) != (redirect_x, redirect_y) {
                            continue;
                        }
                        // operatorを跨いだ先にいくので2マス先を計算
                        if next_x + redirect_x < 0
                            || next_x + redirect_x >= self.board.len() as isize
                            || next_y + redirect_y < 0
                            || next_y + redirect_y >= self.board[0].len() as isize
                        {
                            continue;
                        }
                        let next_pos = (next_x + redirect_x, next_y + redirect_y);
                        let next_pos = (next_pos.0 as usize, next_pos.1 as usize);
                        if updates.contains_key(&next_pos) {
                            panic!("FORCE STOP FOR CONFLICT");
                        }
                        clear_pos.push((i, j));
                        updates.insert(next_pos, self.board[i][j].clone());
                    }
                }
            }
        }

        {
            // Operatorの+, -, *, /, %の場所を取得
            let mut operator_pos = vec![];
            for (i, row) in self.board.iter().enumerate() {
                for (j, cell) in row.iter().enumerate() {
                    if let Token::Operator(op) = cell {
                        match op {
                            Operator::Add
                            | Operator::Sub
                            | Operator::Mul
                            | Operator::Div
                            | Operator::Mod => {
                                operator_pos.push((i, j));
                            }
                            _ => {}
                        }
                    }
                }
            }
            // Operatorの+, -, *, /, %の計算
            for (i, j) in operator_pos {
                // オペランドは左と上、出力は右と下に行う
                let (left, up) = (self.board[i][j - 1].clone(), self.board[i - 1][j].clone());
                let result = match self.board[i][j] {
                    Token::Operator(Operator::Add) => match (left, up) {
                        (Token::Integer(l), Token::Integer(u)) => Token::Integer(l + u),
                        _ => continue,
                    },
                    Token::Operator(Operator::Sub) => match (left, up) {
                        (Token::Integer(l), Token::Integer(u)) => Token::Integer(l - u),
                        _ => continue,
                    },
                    Token::Operator(Operator::Mul) => match (left, up) {
                        (Token::Integer(l), Token::Integer(u)) => Token::Integer(l * u),
                        _ => continue,
                    },
                    Token::Operator(Operator::Div) => match (left, up) {
                        (Token::Integer(l), Token::Integer(u)) => {
                            if l * u >= 0 {
                                Token::Integer(l / u)
                            } else {
                                Token::Integer(l / u - 1)
                            }
                        }
                        _ => continue,
                    },
                    Token::Operator(Operator::Mod) => match (left, up) {
                        (Token::Integer(l), Token::Integer(u)) => Token::Integer(l % u),
                        _ => continue,
                    },
                    _ => panic!("Invalid operator"),
                };
                let next_poses = vec![(i, j + 1), (i + 1, j)];
                for next_pos in next_poses {
                    if updates.contains_key(&next_pos) {
                        panic!("FORCE STOP FOR CONFLICT");
                    }
                    updates.insert(next_pos, result.clone());
                }
                clear_pos.push((i, j - 1));
                clear_pos.push((i - 1, j));
            }
        }
        {
            // OperatorのEq, Neqの場所を取得
            let mut operator_pos = vec![];
            for (i, row) in self.board.iter().enumerate() {
                for (j, cell) in row.iter().enumerate() {
                    if let Token::Operator(op) = cell {
                        match op {
                            Operator::Eq | Operator::Neq => {
                                operator_pos.push((i, j));
                            }
                            _ => {}
                        }
                    }
                }
            }
            // OperatorのEq, Neqの計算
            for (i, j) in operator_pos {
                // オペランドは左と上、出力は右と下に行う
                let (left, up) = (self.board[i][j - 1].clone(), self.board[i - 1][j].clone());
                let result = match self.board[i][j] {
                    Token::Operator(Operator::Eq) => match (left, up) {
                        (Token::Integer(l), Token::Integer(u)) => {
                            if l == u {
                                Ok((Token::Integer(l), Token::Integer(u)))
                            } else {
                                Err(())
                            }
                        }
                        _ => continue,
                    },
                    Token::Operator(Operator::Neq) => match (left, up) {
                        (Token::Integer(l), Token::Integer(u)) => {
                            if l != u {
                                Ok((Token::Integer(l), Token::Integer(u)))
                            } else {
                                Err(())
                            }
                        }
                        _ => continue,
                    },
                    _ => panic!("Invalid operator"),
                };
                let next_poses = vec![(i, j + 1), (i + 1, j)];
                if result.is_ok() {
                    let result = result.unwrap();
                    for next_pos in next_poses {
                        if updates.contains_key(&next_pos) {
                            panic!("FORCE STOP FOR CONFLICT");
                        }
                    }
                    updates.insert((i + 1, j), result.0);
                    updates.insert((i, j + 1), result.1);
                    clear_pos.push((i, j - 1));
                    clear_pos.push((i - 1, j));
                }
            }
        }

        for current in &clear_pos {
            let (i, j) = *current;
            new_board[i][j] = Token::Empty;
        }
        for (next, write) in &updates {
            let (next_i, next_j) = *next;
            new_board[next_i][next_j] = write.clone();
        }
        (new_board, 0)
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for row in self.board.iter() {
            let mut row_strs = vec![];
            for cell in row.iter() {
                row_strs.push(cell.to_string());
            }
            result.push_str(&row_strs.join(" "));
            result.push('\n');
        }
        result
    }
}

#[cfg(test)]
mod tests {

    use crate::three_d::tokenizer::Tokenizer;

    use super::*;

    #[test]
    fn test_emulator_redirection() {
        let mut tokenizer = Tokenizer::new(
            r#"
S 0 .
. v .
. . .
"#,
        );
        let tokens = tokenizer.tokenize();
        let mut emulator = Emulator::new(tokens);
        emulator.tick();

        assert_eq!(
            emulator.board,
            vec![
                vec![
                    Token::Operator(Operator::Submit),
                    Token::Empty,
                    Token::Empty
                ],
                vec![
                    Token::Empty,
                    Token::Operator(Operator::Redirect(Direction::Down)),
                    Token::Empty
                ],
                vec![Token::Empty, Token::Integer(0), Token::Empty]
            ]
        );

        let mut tokenizer = Tokenizer::new(
            r#"
S . .
1 v .
. . .
"#,
        );
        let tokens = tokenizer.tokenize();
        let mut emulator = Emulator::new(tokens);
        emulator.tick();
        assert_eq!(
            emulator.board,
            vec![
                vec![
                    Token::Operator(Operator::Submit),
                    Token::Empty,
                    Token::Empty
                ],
                vec![
                    Token::Integer(1),
                    Token::Operator(Operator::Redirect(Direction::Down)),
                    Token::Empty
                ],
                vec![Token::Empty, Token::Empty, Token::Empty]
            ]
        );

        let mut tokenizer = Tokenizer::new(
            r#"
S . . . .
. < 3 > .
. . . . .
"#,
        );
        let tokens = tokenizer.tokenize();
        let mut emulator = Emulator::new(tokens);
        emulator.tick();
        assert_eq!(
            emulator.board,
            vec![
                vec![
                    Token::Operator(Operator::Submit),
                    Token::Empty,
                    Token::Empty,
                    Token::Empty,
                    Token::Empty
                ],
                vec![
                    Token::Integer(3),
                    Token::Operator(Operator::Redirect(Direction::Left)),
                    Token::Empty,
                    Token::Operator(Operator::Redirect(Direction::Right)),
                    Token::Integer(3),
                ],
                vec![
                    Token::Empty,
                    Token::Empty,
                    Token::Empty,
                    Token::Empty,
                    Token::Empty
                ]
            ]
        );

        let mut tokenizer = Tokenizer::new(
            r#"
S 2 > 1 .
. < 3 v ^
. . . . 5
"#,
        );
        let tokens = tokenizer.tokenize();
        let mut emulator = Emulator::new(tokens);
        emulator.tick();
        assert_eq!(
            emulator.board,
            vec![
                vec![
                    Token::Operator(Operator::Submit),
                    Token::Empty,
                    Token::Operator(Operator::Redirect(Direction::Right)),
                    Token::Integer(2),
                    Token::Integer(5),
                ],
                vec![
                    Token::Integer(3),
                    Token::Operator(Operator::Redirect(Direction::Left)),
                    Token::Empty,
                    Token::Operator(Operator::Redirect(Direction::Down)),
                    Token::Operator(Operator::Redirect(Direction::Up)),
                ],
                vec![
                    Token::Empty,
                    Token::Empty,
                    Token::Empty,
                    Token::Integer(1),
                    Token::Empty
                ]
            ]
        );
    }

    #[test]
    fn test_emulator_numeric_operations() {
        let mut tokenizer = Tokenizer::new(
            r#"
S 1 .
2 + .
. . .
"#,
        );
        let tokens = tokenizer.tokenize();
        let mut emulator = Emulator::new(tokens);
        emulator.tick();
        assert_eq!(
            emulator.board,
            vec![
                vec![
                    Token::Operator(Operator::Submit),
                    Token::Empty,
                    Token::Empty
                ],
                vec![
                    Token::Empty,
                    Token::Operator(Operator::Add),
                    Token::Integer(3)
                ],
                vec![Token::Empty, Token::Integer(3), Token::Empty]
            ]
        );

        let mut tokenizer = Tokenizer::new(
            r#"
S 1 .
2 - .
. . .
"#,
        );
        let tokens = tokenizer.tokenize();
        let mut emulator = Emulator::new(tokens);
        emulator.tick();
        assert_eq!(
            emulator.board,
            vec![
                vec![
                    Token::Operator(Operator::Submit),
                    Token::Empty,
                    Token::Empty
                ],
                vec![
                    Token::Empty,
                    Token::Operator(Operator::Sub),
                    Token::Integer(1)
                ],
                vec![Token::Empty, Token::Integer(1), Token::Empty]
            ]
        );

        let mut tokenizer = Tokenizer::new(
            r#"
S 2 .
3 * .
. . .
"#,
        );
        let tokens = tokenizer.tokenize();
        let mut emulator = Emulator::new(tokens);
        emulator.tick();
        assert_eq!(
            emulator.board,
            vec![
                vec![
                    Token::Operator(Operator::Submit),
                    Token::Empty,
                    Token::Empty
                ],
                vec![
                    Token::Empty,
                    Token::Operator(Operator::Mul),
                    Token::Integer(6)
                ],
                vec![Token::Empty, Token::Integer(6), Token::Empty]
            ]
        );

        let mut tokenizer = Tokenizer::new(
            r#"
S 2 .
3 / .
. . .
"#,
        );
        let tokens = tokenizer.tokenize();
        let mut emulator = Emulator::new(tokens);
        emulator.tick();
        assert_eq!(
            emulator.board,
            vec![
                vec![
                    Token::Operator(Operator::Submit),
                    Token::Empty,
                    Token::Empty
                ],
                vec![
                    Token::Empty,
                    Token::Operator(Operator::Div),
                    Token::Integer(1)
                ],
                vec![Token::Empty, Token::Integer(1), Token::Empty]
            ]
        );

        let mut tokenizer = Tokenizer::new(
            r#"
S 2 .
-3 % .
. . .
"#,
        );
        let tokens = tokenizer.tokenize();
        let mut emulator = Emulator::new(tokens);
        emulator.tick();
        assert_eq!(
            emulator.board,
            vec![
                vec![
                    Token::Operator(Operator::Submit),
                    Token::Empty,
                    Token::Empty
                ],
                vec![
                    Token::Empty,
                    Token::Operator(Operator::Mod),
                    Token::Integer(-1)
                ],
                vec![Token::Empty, Token::Integer(-1), Token::Empty]
            ]
        );
    }

    #[test]
    fn test_emulator_comparison_operations() {
        let mut tokenizer = Tokenizer::new(
            r#"
S 2 .
2 = .
. . .
"#,
        );
        let tokens = tokenizer.tokenize();
        let mut emulator = Emulator::new(tokens);
        emulator.tick();
        assert_eq!(
            emulator.board,
            vec![
                vec![
                    Token::Operator(Operator::Submit),
                    Token::Empty,
                    Token::Empty
                ],
                vec![
                    Token::Empty,
                    Token::Operator(Operator::Eq),
                    Token::Integer(2)
                ],
                vec![Token::Empty, Token::Integer(2), Token::Empty]
            ]
        );

        let mut tokenizer = Tokenizer::new(
            r#"
S 2 .
2 = .
. . .
"#,
        );
        let tokens = tokenizer.tokenize();
        let mut emulator = Emulator::new(tokens);
        emulator.tick();
        assert_eq!(
            emulator.board,
            vec![
                vec![
                    Token::Operator(Operator::Submit),
                    Token::Empty,
                    Token::Empty
                ],
                vec![
                    Token::Empty,
                    Token::Operator(Operator::Eq),
                    Token::Integer(2)
                ],
                vec![Token::Empty, Token::Integer(2), Token::Empty]
            ]
        );

        let mut tokenizer = Tokenizer::new(
            r#"
S 2 .
3 = .
. . .
"#,
        );
        let tokens = tokenizer.tokenize();
        let mut emulator = Emulator::new(tokens);
        emulator.tick();
        assert_eq!(
            emulator.board,
            vec![
                vec![
                    Token::Operator(Operator::Submit),
                    Token::Integer(2),
                    Token::Empty
                ],
                vec![
                    Token::Integer(3),
                    Token::Operator(Operator::Eq),
                    Token::Empty
                ],
                vec![Token::Empty, Token::Empty, Token::Empty]
            ]
        );

        let mut tokenizer = Tokenizer::new(
            r#"
S 2 .
3 # .
. . .
"#,
        );
        let tokens = tokenizer.tokenize();
        let mut emulator = Emulator::new(tokens);
        emulator.tick();
        assert_eq!(
            emulator.board,
            vec![
                vec![
                    Token::Operator(Operator::Submit),
                    Token::Empty,
                    Token::Empty
                ],
                vec![
                    Token::Empty,
                    Token::Operator(Operator::Neq),
                    Token::Integer(2)
                ],
                vec![Token::Empty, Token::Integer(3), Token::Empty]
            ]
        );

        let mut tokenizer = Tokenizer::new(
            r#"
S 2 .
2 # .
. . .
"#,
        );
        let tokens = tokenizer.tokenize();
        let mut emulator = Emulator::new(tokens);
        emulator.tick();
        assert_eq!(
            emulator.board,
            vec![
                vec![
                    Token::Operator(Operator::Submit),
                    Token::Integer(2),
                    Token::Empty
                ],
                vec![
                    Token::Integer(2),
                    Token::Operator(Operator::Neq),
                    Token::Empty
                ],
                vec![Token::Empty, Token::Empty, Token::Empty]
            ]
        );
    }

    #[test]
    fn test_emulator_integration() {
        let mut tokenizer = Tokenizer::new(
            r#"
. . . . 0 . . . .
. 3 > . = . . . .
. v 1 . . > . . .
. . - . . . + S .
. . . . . ^ . . .
. . v . . 0 > . .
. . . . . . 4 + .
. 1 @ 6 . . < . .
. . 3 . 0 @ 3 . .
. . . . . 3 . . .
"#,
        );
        let tokens = tokenizer.tokenize();
        let mut emulator = Emulator::new(tokens);
        emulator.tick();
        eprintln!("{}", emulator.to_string());
    }
}
