pub mod utils;

use sdl3::pixels::Color;

use crate::utils::{FrameBuffer, Pixel};

pub struct App {
    width: u32,
    height: u32,
    current_row: usize,
    current_state: usize,
    states: [State; 2],
    frame_buffer: FrameBuffer,
}

impl App {
    pub fn new(width: u32, height: u32) -> Self {
        App {
            width,
            height,
            current_row: 0,
            current_state: 0,
            states: [State::default(width as usize), State::empty(width as usize)],
            frame_buffer: FrameBuffer::new(width, height),
        }
    }

    pub fn frame_buffer(&self) -> &FrameBuffer {
        &self.frame_buffer
    }

    pub fn update_frame_buffer(&mut self) {
        for (i, cell) in self.states[self.current_state].cells().iter().enumerate() {
            self.frame_buffer.set_pixel(
                i,
                self.current_row as usize,
                match cell {
                    Cell::Dead => Pixel::BLACK,
                    Cell::Alive => Pixel::WHITE,
                },
            );
        }
    }

    pub fn current_cells(&self) -> &[Cell] {
        self.states[self.current_state].cells()
    }

    // Get the next frame by updating the frame buffers
    pub fn next(&mut self) {
        let width = self.width as usize;
        for i in 0..width {
            let left = if i == 0 { width - 1 } else { i - 1 };

            let right = if i + 1 == width { 0 } else { i + 1 };

            let left_neighbour = self.states[self.current_state].cell_at(left);

            // Rule 30 ruleset can be simplified to "if only 1 of the neighbourhood is alive, or 2 are alive and the left one is dead", next is live
            let neighbour_count = self.states[self.current_state].cell_at(left).value()
                + self.states[self.current_state].cell_at(i).value()
                + self.states[self.current_state].cell_at(right).value();

            let next_cell = match (left_neighbour, neighbour_count) {
                (Cell::Dead, 2) | (_, 1) => Cell::Alive,
                _ => Cell::Dead,
            };

            self.states[(self.current_state + 1) % self.states.len()].set_cell(i, next_cell);
        }

        self.current_state += 1;
        self.current_state %= self.states.len();

        self.current_row += 1;
        self.current_row %= self.height as usize;
    }
}

pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    pub fn alive() -> Cell {
        Cell::Alive
    }

    pub fn dead() -> Cell {
        Cell::Dead
    }

    pub fn value(&self) -> u8 {
        match self {
            Cell::Dead => 0,
            Cell::Alive => 1,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Cell::Dead => Color::BLACK,
            Cell::Alive => Color::WHITE,
        }
    }
}

pub struct State {
    cells: Vec<Cell>,
}

impl State {
    pub fn default(size: usize) -> Self {
        let mut cells = Vec::with_capacity(size);

        for i in 0..size {
            cells.push(Cell::Dead)
        }

        cells[0] = Cell::Alive;

        Self { cells }
    }

    pub fn empty(size: usize) -> Self {
        let mut cells = Vec::with_capacity(size);
        for i in 0..size {
            cells.push(Cell::Dead);
        }
        Self { cells }
    }

    pub fn cells(&self) -> &[Cell] {
        &self.cells[..]
    }

    pub fn cell_at(&self, idx: usize) -> &Cell {
        &self.cells[idx]
    }

    pub fn set_cell(&mut self, idx: usize, value: Cell) {
        self.cells[idx] = value
    }
}
