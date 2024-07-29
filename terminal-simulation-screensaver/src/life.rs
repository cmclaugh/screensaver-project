use rand::random;
use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};
use std::cmp::min;

#[derive(Debug)]
pub struct LifeSimulator {
    board: Vec<Vec<bool>>,
}

impl LifeSimulator {
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

    pub fn print(&self) {
        for row in self.board.iter() {
            for cell in row.iter() {
                print!("{}", if *cell { '0' } else { ' ' });
            }
            println!();
        }
    }

    pub fn new(height: usize, width: usize) -> Self {
        let mut board = Vec::with_capacity(height);
        for _ in 0..height {
            let mut new_row = Vec::with_capacity(width);
            for _ in 0..width {
                new_row.push(random());
            }
            board.push(new_row);
        }
        LifeSimulator { board }
    }
}

impl Widget for &LifeSimulator {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for x in area.left()..area.right() {
            for y in area.top()..area.bottom() {
                let cell = buf.get_mut(x, y);
                cell.reset();
                if self.board[y as usize][x as usize] {
                    cell.set_bg(Color::White);
                }
            }
        }
    }
}
