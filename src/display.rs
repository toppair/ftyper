use crate::layout::{Layout, Print};
use crate::types::Result;
use termion::clear;
use termion::cursor;
use termion::terminal_size;

pub struct Display {
    col_offset: u16,
    bounds_set: bool,
    layout_size: (u16, u16),
}

impl Display {
    pub fn new() -> Self {
        Self {
            bounds_set: false,
            col_offset: 0,
            layout_size: (0, 0),
        }
    }

    fn set_bounds(&mut self, layout: &impl Layout) {
        if !self.bounds_set {
            let (_, term_cols) = terminal_size().unwrap();
            let mut index = 0;
            let mut total_rows = 0;
            let mut total_cols = 0;
            while let Some((max_num_rows, max_num_cols)) = layout.get_row_size(index) {
                total_rows += max_num_rows;
                total_cols = if total_cols < max_num_cols {
                    max_num_cols
                } else {
                    total_cols
                };
                index += 1;
            }
            self.layout_size = (total_rows, total_cols);
            let col_offset = (term_cols - total_cols) / 2 - total_cols / 2;
            if col_offset > 0 {
                self.col_offset = col_offset;
            }
            self.bounds_set = true;
        }
    }

    fn render_internal(&self, layout: &impl Layout, clear: bool) -> Result<()> {
        let mut index = 0;

        if clear {
            Self::clear();
        }

        while let Some((rows, _)) = layout.get_row_size(index) {
            let components = layout.get_row(index).unwrap();
            for x in 0..rows {
                // print!("{:1$}", " ", self.col_offset as usize);
                components.iter().for_each(|component| {
                    component.print(x);
                });
                println!();
                print!("{}{}", clear::CurrentLine, cursor::Left(100));
            }

            index += 1;
        }

        Ok(())
    }

    pub fn clear() {
        print!("{}{}", clear::BeforeCursor, cursor::Goto(1, 1));
    }

    pub fn render_no_clear(&mut self, layout: &impl Layout) -> Result<()> {
        self.set_bounds(layout);
        self.render_internal(layout, false)
    }

    pub fn render(&mut self, layout: &impl Layout) -> Result<()> {
        self.set_bounds(layout);
        self.render_internal(layout, true)
    }
}
