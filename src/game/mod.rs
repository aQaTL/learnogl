use crate::{load_texture, Keys, SampledSrgbTexture2d};
use nalgebra_glm as glm;

pub struct Game {
    pub faze: GameFaze,
    pub tower: Tower,

    pub score: f32,

    update_fn: UpdateFn,
}

type UpdateFn = fn(&mut Game, delta_time: f32);

#[derive(Debug)]
pub enum GameFaze {
    TitleScreen,
    GameRunning,
    DeathScreen,
}

pub struct Tower {
    pub floors: u32,
    pub stairs: u32,
    pub width: f32,
    pub gravity: f32,

    pub scrolling_speed: f32,
    pub current_level: f32,

    pub player: Player,
}

struct Stair {
    width: f32,
}

pub struct Player {
    pub pos: glm::Vec3,
    pub size: glm::Vec3,
    pub velocity: glm::Vec3,
    pub facing: Turn,
    pub state: PlayerState,

    pub max_jump_count: u8,
    pub jump_accel: f32,
    pub movement_accel: f32,

    pub tex: SampledSrgbTexture2d,
}

pub enum Turn {
    North,
    South,
    East,
    West,
}

pub enum PlayerState {
    Standing,
    //jump count
    Jumping(u8),
}

impl Tower {
    pub fn new(player_tex: SampledSrgbTexture2d) -> Self {
        Tower {
            floors: 10,
            stairs: 40,
            width: 100.0,
            gravity: 3.0,

            scrolling_speed: 10.0,
            current_level: 0.0,

            player: Player {
                pos: glm::Vec3::new(0.0, 0.0, 0.0),
                size: glm::Vec3::new(5.0, 5.0, 0.0),
                velocity: glm::Vec3::new(0.0, 0.0, 0.0),
                facing: Turn::South,
                state: PlayerState::Standing,

                max_jump_count: 2,
                jump_accel: 40.0,
                movement_accel: 100.0,

                tex: player_tex,
            },
        }
    }
}

impl Game {
    pub fn new(player_tex: SampledSrgbTexture2d) -> Self {
        Game {
            faze: GameFaze::TitleScreen,
            tower: Tower::new(player_tex),
            score: 0.0,
            update_fn: Game::update_title_screen,
        }
    }

    pub fn process_keyboard_input(&mut self, keys: &Keys, delta_time: f32) {
        use glium::glutin::VirtualKeyCode::*;
        if keys[Space] {
            match self.tower.player.state {
                PlayerState::Standing => {
                    self.tower.player.state = PlayerState::Jumping(1);
                    self.tower.player.velocity[1] = self.tower.player.jump_accel;
                }
                PlayerState::Jumping(mut count) if count < self.tower.player.max_jump_count => {
                    count += 1;
                    self.tower.player.velocity[1] = self.tower.player.jump_accel;
                }
                _ => (),
            }
        }
        if keys[Up] {
            self.tower.player.pos.y += self.tower.player.movement_accel * delta_time;
        }
        if keys[Down] {
            self.tower.player.pos.y -= self.tower.player.movement_accel * delta_time;
        }
        if keys[Left] {
            //self.tower.player.velocity.x -= self.tower.player.movement_accel * delta_time;
            self.tower.player.pos.x -= self.tower.player.movement_accel * delta_time;
            self.tower.player.facing = Turn::West;
        }
        if keys[Right] {
            //self.tower.player.velocity.x += self.tower.player.movement_accel * delta_time;
            self.tower.player.pos.x += self.tower.player.movement_accel * delta_time;
            self.tower.player.facing = Turn::East;
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        (self.update_fn)(self, delta_time);
    }

    fn update_title_screen(&mut self, delta_time: f32) {}

    fn update_game(&mut self, delta_time: f32) {
        self.tower.player.pos += (self.tower.player.velocity * delta_time);
        self.tower.player.pos;
    }

    fn update_death_screen(&mut self, delta_time: f32) {}

    pub fn set_faze(&mut self, faze: GameFaze) {
        self.update_fn = match faze {
            GameFaze::TitleScreen => Game::update_title_screen,
            GameFaze::GameRunning => Game::update_game,
            GameFaze::DeathScreen => Game::update_death_screen,
        };
        self.faze = faze;
    }
}
