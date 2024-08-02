use rand::random;
use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};

// simulates Conway's game of life
// uses torodial space (i.e. the sides wrap around)
// uses a 2-dimensional Vec of booleans to represent cells
#[derive(Debug)]
pub struct LifeSimulator {
    board: Vec<Vec<bool>>,
    height: usize,
    width: usize,
}

impl LifeSimulator {
    // increments the game of life one generation
    pub fn update(&mut self) {
        // works by creating a new 2-dimensional Vec and filling it one cell at a time

        // loops all existing cells to check if they should live or die
        let mut to_return = Vec::with_capacity(self.height);
        for (row_index, row) in self.board.iter().enumerate() {
            to_return.push(Vec::with_capacity(self.width));
            for (cell_index, cell) in row.iter().enumerate() {
                // counts the number of live cells in the neighborhood (within a 3x3 grid)
                // note that this includes the cell itself
                let mut life_counter = 0;

                // gets references to the previous and next rows with wraparound
                let previous_row = &self.board[if row_index == 0 {
                    self.height - 1
                } else {
                    row_index - 1
                }];
                let next_row = &self.board[if row_index == self.height - 1 {
                    0
                } else {
                    row_index + 1
                }];

                // loops over the three neighborhood rows
                for neighbor_row in [previous_row, row, next_row] {
                    // gets references to left, right, and center cells with wraparound
                    let left_cell = &neighbor_row[if cell_index == 0 {
                        self.width - 1
                    } else {
                        cell_index - 1
                    }];
                    let center_cell = &neighbor_row[cell_index];
                    let right_cell = &neighbor_row[if cell_index == self.width - 1 {
                        0
                    } else {
                        cell_index + 1
                    }];

                    // adds cells to tally (if alive)
                    for neighbor_cell in [left_cell, center_cell, right_cell] {
                        if *neighbor_cell {
                            life_counter += 1
                        };
                    }
                }

                // a cell lives if it has 3 live neighbors (including itself)
                // or it has 4 live neighbors and is alive itself
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
    // all cells have random value
    pub fn new(height: usize, width: usize) -> Self {
        let mut board = Vec::with_capacity(height);
        for _ in 0..height {
            let mut new_row = Vec::with_capacity(width);
            for _ in 0..width {
                new_row.push(random());
            }
            board.push(new_row);
        }

        LifeSimulator {
            board,
            height,
            width,
        }
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
                // be rendered anyway. They aren't in this project at least.
                if y < (self.height as u16)
                    && x < (self.width as u16)
                    && self.board[y as usize][x as usize]
                {
                    cell.set_bg(Color::White);
                }
            }
        }
    }
}
