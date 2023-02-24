use piston_window::{PistonWindow, WindowSettings, clear, rectangle, RenderEvent, Event};
use rand::prelude::*;

const GRID_WIDTH: usize = 50;
const GRID_HEIGHT: usize = 50;
const CELL_SIZE: usize = 10;

struct Grid {
    cells: [[bool; GRID_HEIGHT]; GRID_WIDTH],
}

impl Grid {
    fn new() -> Grid {
        let mut rng = rand::thread_rng();
        let mut cells = [[false; GRID_HEIGHT]; GRID_WIDTH];

        for i in 0..GRID_WIDTH {
            for j in 0..GRID_HEIGHT {
                cells[i][j] = rng.gen_bool(0.1); // randomly initialize each cell
            }
        }

        Grid { cells }
    }

    fn update(&mut self) {
        let mut next = [[false; GRID_HEIGHT]; GRID_WIDTH];

        for i in 0..GRID_WIDTH {
            for j in 0..GRID_HEIGHT {
                let mut live_neighbors = 0;

                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let x = i as i32 + dx;
                        let y = j as i32 + dy;

                        if x < 0 || x >= GRID_WIDTH as i32 || y < 0 || y >= GRID_HEIGHT as i32 {
                            continue;
                        }

                        if self.cells[x as usize][y as usize] {
                            live_neighbors += 1;
                        }
                    }
                }

                if self.cells[i][j] && (live_neighbors == 2 || live_neighbors == 3) {
                    next[i][j] = true;
                } else if !self.cells[i][j] && live_neighbors == 3 {
                    next[i][j] = true;
                }
            }
        }

        self.cells = next;
    }

    fn render(&self, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |e, g, _| {
            clear([1.0; 4], g);

            for i in 0..GRID_WIDTH {
                for j in 0..GRID_HEIGHT {
                    let x = i as f64 * CELL_SIZE as f64;
                    let y = j as f64 * CELL_SIZE as f64;

                    if self.cells[i][j] {
                        rectangle([0.0, 0.0, 0.0, 1.0], [x, y, CELL_SIZE as f64, CELL_SIZE as f64], e.transform, g);
                    }
                }
            }
        });
    }
}

fn initialize_game_engine() -> PistonWindow {
    WindowSettings::new("Cellular Automata", [(GRID_WIDTH * CELL_SIZE) as u32, (GRID_HEIGHT * CELL_SIZE) as u32])
        .exit_on_esc(true)
        .build()
        .unwrap()
}

fn main() {
    let mut window = initialize_game_engine();
    let mut grid = Grid::new();

    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            grid.update();
            grid.render(&mut window, &event);
        }
    }
}
