use std::io;

#[derive(Debug, Default)]
struct Game {
    board: [[String; 3]; 3],
    turn: String,
    x_positions: Vec<(usize, usize)>,
    o_positions: Vec<(usize, usize)>,
}

impl Game {
    fn format_board(&self) {
        for row in &self.board {
            println!("{:?}", row);
        }
    }

    fn value_to_board(&mut self, row: usize, col: usize) -> bool {
        if self.board[row - 1][col - 1] == "" {
            self.board[row - 1][col - 1] = self.turn.clone();
            if self.turn == "X" {
                self.x_positions.push((row - 1, col - 1));
            } else {
                self.o_positions.push((row - 1, col - 1));
            }
            true
        } else {
            false
        }
    }

    fn change_turn(&mut self) {
        if self.turn == "X".to_string() {
            self.turn = "O".to_string();
        } else {
            self.turn = "X".to_string();
        }
    }
    fn has_won(&self) -> bool {
        let positions: [[(usize, usize); 3]; 8] = [
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],
            [(0, 0), (1, 1), (2, 2)],
            [(0, 2), (1, 1), (2, 0)],
        ];
        let mut to_iterate: Vec<(usize, usize)> = Vec::new();
        if self.turn == "X".to_string() {
            to_iterate = self.x_positions.clone();
        } else {
            to_iterate = self.o_positions.clone();
        }
        'rows: for i in 0..positions.len() {
            for j in 0..positions[i].len() {
                if to_iterate
                    .iter()
                    .any(|&x| x == (positions[i][j].0, positions[i][j].1))
                {
                    if j == 2 {
                        return true;
                    }
                } else {
                    continue 'rows;
                }
            }
        }
        return false;
    }
}

fn get_input() -> u8 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("An error was encountered");
        match input.trim().parse::<u8>() {
            Ok(input) => {
                if 4 > input && input > 0 {
                    return input;
                } else {
                    println!("Please enter a number between 1 and 3");
                }
            }
            Err(_) => println!("Please enter a number between 1 and 3"),
        }
    }
}

fn main() {
    let mut game: Game = Game {
        turn: "X".to_string(),
        ..Default::default()
    };
    loop {
        println!("{} turn", game.turn);
        println!("Row:");
        let row = get_input();
        println!("Column:");
        let column = get_input();
        if !game.value_to_board(row.into(), column.into()) {
            println!("Invalid move!");
            continue;
        }
        game.format_board();
        if game.has_won() {
            println!("{} wins!", game.turn);
            break;
        }
        game.change_turn();
    }
}
