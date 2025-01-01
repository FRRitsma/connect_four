use std::collections::HashMap;
const CONNECT: usize = 4;


#[derive(Clone)]
enum Player {
    Player1,
    Player2,
}

struct Cell{
    owner: Option<Player>,
}

impl Cell{
    fn new() -> Self{
        Cell{
            owner: None,
        }
    }
    fn occupy(&mut self, player: Player){
        if self.owner.is_some(){
            panic!("Cell is already occupied");
        }
        self.owner = Some(player);
    }
}

struct Board{
    current_player: Player,
    width: usize,
    height: usize,
    board: HashMap<(usize, usize), Cell>,
}

impl Board{
    fn new(width: usize, height: usize) -> Self{
        let mut board = HashMap::new();
        for x in 0..width{
            for y in 0..height{
                board.insert((x, y), Cell::new());
            }
        }
        Board{
            current_player: Player::Player1,
            width,
            height,
            board,
        }
    }

    fn display_board(&self){
        for y in (0..self.height).rev(){
            for x in 0..self.width{
                if let Some(cell) = self.board.get(&(x, y)){
                    match cell.owner{
                        Some(Player::Player1) => print!("X"),
                        Some(Player::Player2) => print!("O"),
                        None => print!("-"),
                    }
                }
        }
        println!();
        }
    }

    fn check_win_condition_single_entry(&self, row: usize, col: usize) -> Option<Player> {
        let cell = self.board.get(&(row, col)).unwrap();
        return None;
    }
    fn check_win_condition(&self){
        for row in 0..self.height - 4 {
            println!("Row: {}", row);
        }
    }

    fn occupy(&mut self, x: usize){
        if x >= self.width{
            panic!("Invalid column");
        }
        for y in 0..self.height{
            let cell = self.board.get_mut(&(x, y)).unwrap();
            if cell.owner.is_none(){
                cell.occupy(self.current_player.clone());
                self.current_player = match self.current_player{
                    Player::Player1 => Player::Player2,
                    Player::Player2 => Player::Player1,
                };
                return;
            }
        }
        panic!("Column is full");
    }
}



#[test] // Marks this function as a test case.
fn test_init_board(){
    let board = Board::new(7, 6);
    assert_eq!(board.width, 7);
    assert_eq!(board.height, 6);
}

#[test]
fn test_print_board(){
    let mut board = Board::new(7, 6);
    board.occupy(0);
    board.occupy(0);
    board.occupy(0);
    board.occupy(0);
    board.display_board();
}

#[test]
fn test_check_win_condition(){
    let mut board = Board::new(7, 6);
    board.occupy(0);
    board.occupy(1);
    board.occupy(2);
    board.occupy(3);
    board.check_win_condition();
}
