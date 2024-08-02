use rand::random;
use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};
use std::cmp::min;

// simulates Conway's game of life
// uses a 2-dimensional Vec of booleans to represent cells
#[derive(Debug)]
pub struct LifeSimulator {
    board: Vec<Vec<bool>>,
    height: usize,
    width: usize,
}

impl LifeSimulator {
    // increments the game of life one step
    pub fn update(&mut self) {
        let mut to_return = Vec::new();
        for (row_index, row) in self.board.iter().enumerate() {
            to_return.push(Vec::new());
            for (cell_index, cell) in row.iter().enumerate() {
                let mut life_counter = 0;
                for neighbor_row in &self.board[if row_index == 0 { 0 } else { row_index - 1 }
                    ..min(self.board.len(), row_index + 2)]
                {
                    for neighbor_cell in
                        &neighbor_row[if cell_index == 0 { 0 } else { cell_index - 1 }
                            ..min(row.len(), cell_index + 2)]
                    {
                        if *neighbor_cell {
                            life_counter += 1
                        };
                    }
                }
                to_return[row_index].push(life_counter == 3 || (life_counter == 4 && *cell));
            }
        }
        self.board = to_return;
    }

    // prints a crude representation of the board
    // inefficient. for debug purposes only.
    pub fn print(&self) {
        for row in self.board.iter() {
            for cell in row.iter() {
                print!("{}", if *cell { '0' } else { ' ' });
            }
            println!();
        }
    }

    // constructs a new LifeSimulator with a board of specified height and width
    pub fn new(height: usize, width: usize) -> Self {
        let mut board = Vec::with_capacity(height);
        for _ in 0..height {
            let mut new_row = Vec::with_capacity(width);
            for _ in 0..width {
                new_row.push(random());
            }
            board.push(new_row);
        }
        LifeSimulator { board, height, width }
    }
}

impl Widget for &LifeSimulator {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // loops through every cell in the area
        for x in area.left()..area.right() {
            for y in area.top()..area.bottom() {
                let cell = buf.get_mut(x, y);
                // resets the terminal cell to blank (representing a "dead" life cell)
                cell.reset();

                // sets any terminal cells to white if their corresponding life cell
                // is "alive"
                //
                // NOTE: The cast of height and width to u16 will cause a bug if the 
                // board is over 65,535 cells tall or wide, but such boards shouldn't 
                // be rendered anyway. They aren't in this project.
                if y < (self.height as u16) && x < (self.width as u16) && self.board[y as usize][x as usize] {
                    cell.set_bg(Color::White);
                }
            }
        }
    }
}
