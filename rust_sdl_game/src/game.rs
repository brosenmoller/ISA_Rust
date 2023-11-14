use rand::{rngs::ThreadRng, Rng};
use sdl2::{pixels::Color, event::Event};
use crate::{vector2int::Vector2Int, sdl_context::SdlContext, grid_tile::GridTile};

const FIELD_COLOR: Color = Color::RGB(10, 10, 10);
const PLAYER_COLOR: Color = Color::RGB(10, 150, 10);
const TAIL_COLOR: Color = Color::RGB(10, 80, 10);
const FRUIT_COLOR: Color = Color::RGB(150, 10, 10);

#[derive(PartialEq, Eq)]
enum Direction {
    STOP,
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

pub struct Game {
    pub running: bool,
    sdl_context: SdlContext,
    tail: Vec<GridTile>,
    grid_size: u32,
    grid_tile_size: u32,
    direction: Direction,
    fruit: GridTile,
    player: GridTile,
    rng: ThreadRng,
    score: u32,
}

impl Game {
    pub fn new(screen_width: u32, screen_height: u32, grid_size: u32) -> Result<Game, String> {
        
        let sdl_context = SdlContext::new(screen_width, screen_height)?;
        let grid_tile_size = screen_width / grid_size;

        let player = GridTile::new(
            Vector2Int::new((grid_size / 2) as i32, (grid_size / 2) as i32),
            PLAYER_COLOR,
            grid_tile_size  
        );

        let fruit = GridTile::new(
            Vector2Int::new(0, 0),
            FRUIT_COLOR,
            grid_tile_size  
        );

        let rng = rand::thread_rng();
        
        let mut game: Game = Game {
            running: true,
            sdl_context,
            tail: vec![],
            grid_size,
            grid_tile_size,
            player,
            fruit,
            direction: Direction::STOP,
            rng,
            score: 0,
        };

        game.fruit.position = game.rand_grid_point();
        
        Ok(game)
    }

    pub fn events(&mut self) {
        for event in self.sdl_context.event_queue.poll_iter() {
            match event {
                Event::Quit {..} => {
                    self.running = false;
                },
                _ => {}
            }
        }
    }

    pub fn input(&mut self) {
        let keyboard_state: sdl2::keyboard::KeyboardState<'_> = self.sdl_context.event_queue.keyboard_state();
        
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Up) 
        { if self.direction != Direction::DOWN { self.direction = Direction::UP; } }

        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Down) 
        { if self.direction != Direction::UP { self.direction = Direction::DOWN; } }

        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Right) 
        { if self.direction != Direction::LEFT { self.direction = Direction::RIGHT; }  }

        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Left)
        { if self.direction != Direction::RIGHT { self.direction = Direction::LEFT; } }
    }

    pub fn logic(&mut self) {

        let last_pos = self.player.position.clone();

        match self.direction {
            Direction::UP => self.player.position.y -= 1,
            Direction::DOWN => self.player.position.y += 1,
            Direction::RIGHT => self.player.position.x += 1,
            Direction::LEFT => self.player.position.x -= 1,
            Direction::STOP => {},
        }

        if self.player.position.x < 0 || 
           self.player.position.x > self.grid_size as i32 - 1 || 
           self.player.position.y < 0 ||
           self.player.position.y > self.grid_size as i32 - 1 ||
           self.is_player_in_tail()
        {
            self.on_death();
        }
        
        self.tail.insert(0, GridTile::new(last_pos, TAIL_COLOR, self.grid_tile_size));

        if self.player.position == self.fruit.position {
            self.score += 1;
            self.fruit.position = self.rand_grid_point();
        }
        else {
            self.tail.pop();
        }
    }

    pub fn render(&mut self) {

        self.sdl_context.canvas.set_draw_color(FIELD_COLOR);
        self.sdl_context.canvas.clear();

        self.player.render(&mut self.sdl_context.canvas);
        self.fruit.render(&mut self.sdl_context.canvas);

        for tail_segment in &mut self.tail {
            tail_segment.render(&mut self.sdl_context.canvas);
        }

        self.sdl_context.canvas.present();
    }

    fn rand_grid_point(&mut self) -> Vector2Int {

        let mut new_pos: Vector2Int;

        loop {
            new_pos = Vector2Int::new(
                self.rng.gen_range(0..self.grid_size as i32),
                self.rng.gen_range(0..self.grid_size as i32),
            );

            if !self.is_point_occupied(&new_pos) {
                break;
            }
        }

        return new_pos;
    }

    fn is_point_occupied(&self, point: &Vector2Int) -> bool{
        
        if self.player.position == *point {
            return true;
        }
        
        for tail_segement in &self.tail  {
            if tail_segement.position == *point {
                return true;
            }
        }

        return false;
    }

    fn is_player_in_tail(&self) -> bool {
        for tail_segement in &self.tail  {
            if tail_segement.position == self.player.position {
                return true;
            }
        }

        return false;
    }

    fn on_death(&mut self){
        self.score = 0;
        self.tail = vec![];
        self.player.position = Vector2Int::new((self.grid_size / 2) as i32, (self.grid_size / 2) as i32);
        self.fruit.position = self.rand_grid_point();
    }
}
