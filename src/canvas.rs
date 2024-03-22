use std::{
    fmt::{Display, Write},
    io::{Stdout, Write as Wt},
};

use crate::{
    drawable::Drawable,
    stout_ext::{AsLocationTuple, StdoutExt},
};

#[derive(Clone, PartialEq, Eq)]
pub enum Block {
    Empty,
    Acquired(char),
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Empty => f.write_char(' '),
            Block::Acquired(c) => f.write_char(*c),
        }
    }
}

pub struct Canvas {
    max_c: u16,
    mac_l: u16,
    table: Vec<Vec<Block>>,
    table_snapshot: Vec<Vec<Block>>,
}

impl Canvas {
    pub fn new(max_c: u16, mac_l: u16) -> Self {
        let table: Vec<Vec<Block>> = (0..mac_l)
            .map(|_| (0..max_c).map(|_| Block::Empty).collect())
            .collect();

        Self {
            max_c,
            mac_l,
            table: table.clone(),
            table_snapshot: table,
        }
    }

    pub fn draw(&mut self, drawable: &impl Drawable) -> &mut Canvas {
        drawable.draw(self);
        self
    }

    pub fn draw_line(
        &mut self,
        loc: impl AsLocationTuple,
        display: impl Into<String>,
    ) -> &mut Canvas {
        let (c, l) = loc.as_loc_tuple();
        let string: String = display.into();

        for (offset, ch) in string.chars().enumerate() {
            self.acquire_block((c as usize) + offset, l as usize, ch);
        }

        self
    }

    pub fn draw_char(&mut self, loc: impl AsLocationTuple, display: char) -> &mut Canvas {
        let (c, l) = loc.as_loc_tuple();
        self.acquire_block(c as usize, l as usize, display);

        self
    }

    pub fn clear_all(&mut self) -> &mut Canvas {
        self.table = (0..self.mac_l)
            .map(|_| (0..self.max_c).map(|_| Block::Empty).collect())
            .collect();
        self
    }

    pub fn acquire_block(&mut self, c: usize, l: usize, new_char: char) {
        self.table[l][c] = Block::Acquired(new_char)
    }

    // pub fn clear_block(&mut self, c: usize, l: usize) {
    //     self.table[l][c] = Block::Empty
    // }

    fn detect_changes(&self) -> Vec<(usize, usize)> {
        let mut changes: Vec<(usize, usize)> = vec![];
        for (l, line) in self.table.iter().enumerate() {
            for (c, block) in line.iter().enumerate() {
                if block != &self.table_snapshot[l][c] {
                    changes.push((c, l))
                }
            }
        }

        changes
    }

    pub fn draw_map(&mut self, stdout: &mut Stdout) -> Result<(), std::io::Error> {
        for (c, l) in self.detect_changes() {
            let block = self.table[l][c].clone();
            stdout.draw((c as u16, l as u16), &block)?;
            self.table_snapshot[l][c] = block;
        }

        stdout.flush()?;
        Ok(())
    }
}
