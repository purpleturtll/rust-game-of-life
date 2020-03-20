use ggez;
use std::sync::Mutex;
use std::thread;

use ggez::{event, graphics, Context, GameResult};
use std::time::{Duration, Instant};

const GRID_SIZE: (i16, i16) = (20, 20);
const GRID_CELL_SIZE: (i16, i16) = (32, 32);

const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

const UPDATES_PER_SECOND: f32 = 10.0;
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GridPosition {
    x: i16,
    y: i16,
}

impl From<(i16, i16)> for GridPosition {
    fn from(pos: (i16, i16)) -> Self {
        GridPosition { x: pos.0, y: pos.1 }
    }
}

impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32 * GRID_CELL_SIZE.0 as i32,
            pos.y as i32 * GRID_CELL_SIZE.1 as i32,
            GRID_CELL_SIZE.0 as i32,
            GRID_CELL_SIZE.1 as i32,
        )
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum State {
    Dead,
    Alive,
}

#[derive(Clone, Copy, Debug)]
struct Cell {
    pos: GridPosition,
    state: State,
}

impl Cell {
    fn new(pos: GridPosition) -> Self {
        Cell {
            pos: pos,
            state: State::Dead,
        }
    }
}

struct GameState {
    grid: Vec<Vec<Cell>>,
    start: bool,
    last_update: Instant,
}

impl GameState {
    pub fn new() -> Self {
        let mut grid =
            vec![vec![Cell::new((0, 0).into()); GRID_SIZE.1 as usize]; GRID_SIZE.0 as usize];

        for x in 0..grid.len() {
            for y in 0..grid[x].len() {
                grid[x][y].pos = (x as i16, y as i16).into();
            }
        }

        GameState {
            grid: grid,
            start: false,
            last_update: Instant::now(),
        }
    }
}

fn wrap(i: i16) -> i16 {
    match i {
        0..=19 => return i,
        -1 => return 19,
        20 => return 0,
        _ => return -69,
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE)
            && self.start
        {
            let mut new_grid = self.grid.clone();
            for x in 0..self.grid.len() {
                for y in 0..self.grid[x].len() {
                    let mut n = 0;
                    if self.grid[wrap(x as i16 - 1) as usize][wrap((y) as i16) as usize].state
                        == State::Alive
                    {
                        n += 1
                    }
                    if self.grid[wrap(x as i16 - 1) as usize][wrap(y as i16 - 1) as usize].state
                        == State::Alive
                    {
                        n += 1
                    }
                    if self.grid[wrap(x as i16 - 1) as usize][wrap((y + 1) as i16) as usize].state
                        == State::Alive
                    {
                        n += 1
                    }
                    if self.grid[wrap((x + 1) as i16) as usize][wrap((y) as i16) as usize].state
                        == State::Alive
                    {
                        n += 1
                    }
                    if self.grid[wrap((x + 1) as i16) as usize][wrap(y as i16 - 1) as usize].state
                        == State::Alive
                    {
                        n += 1
                    }
                    if self.grid[wrap((x + 1) as i16) as usize][wrap((y + 1) as i16) as usize].state
                        == State::Alive
                    {
                        n += 1
                    }
                    if self.grid[wrap((x) as i16) as usize][wrap(y as i16 - 1) as usize].state
                        == State::Alive
                    {
                        n += 1
                    }
                    if self.grid[wrap((x) as i16) as usize][wrap((y + 1) as i16) as usize].state
                        == State::Alive
                    {
                        n += 1
                    }
                    if self.grid[x][y].state == State::Alive && (n == 2 || n == 3) {
                        new_grid[x][y].state = State::Alive;
                    } else if self.grid[x][y].state == State::Dead && n == 3 {
                        new_grid[x][y].state = State::Alive;
                    } else {
                        new_grid[x][y].state = State::Dead;
                    }
                }
            }
            self.grid = new_grid.clone();
            self.last_update = Instant::now();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        for x in 0..self.grid.len() {
            for y in 0..self.grid[x].len() {
                let color;
                if self.grid[x][y].state == State::Alive {
                    color = [1.0, 1.0, 1.0, 1.0].into();
                } else {
                    color = [0.0, 0.0, 0.0, 1.0].into();
                }
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    self.grid[y][x].pos.into(),
                    color,
                )?;
                graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
            }
        }
        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: ggez::input::keyboard::KeyCode,
        _keymod: ggez::input::keyboard::KeyMods,
        _repeat: bool,
    ) {
        if keycode == ggez::input::keyboard::KeyCode::Space {
            self.start = !self.start;
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: ggez::input::mouse::MouseButton,
        _x: f32,
        _y: f32,
    ) {
        let t_x = _x / GRID_CELL_SIZE.0 as f32;
        let t_y = _y / GRID_CELL_SIZE.1 as f32;
        if self.grid[t_y as usize][t_x as usize].state == State::Dead {
            self.grid[t_y as usize][t_x as usize].state = State::Alive;
        } else {
            self.grid[t_y as usize][t_x as usize].state = State::Dead;
        }
    }
}

fn main() -> GameResult {
    let (ctx, events_loop) = &mut ggez::ContextBuilder::new("Game of Life", "Bartosz Ciesielczyk")
        .window_setup(ggez::conf::WindowSetup::default().title("Game of Life!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let state = &mut GameState::new();
    event::run(ctx, events_loop, state)
}
