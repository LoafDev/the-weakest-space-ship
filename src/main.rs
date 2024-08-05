use raylib::prelude::{*, KeyboardKey::*};
use std::ops::Range;

const SCREEN_HEIGHT: i32 = 800;
const SCREEN_WIDTH: i32 = 800;
const PLAYER_LIFE: u8 = 3;
const MAX_BULLETS: u32 = 100;
const MAX_ENEMIES: u32 = 10;
const SUPERULTRADUPERCOOLSHOOTINGOBJECTFORNOREASONATALLHEALTH: u8 = 10;

enum GameState {
    Menu,
    InGame,
    Win
}

#[derive(Default)]
struct Player {
    position: Vector2,
    speed: f32,
    life: u8,
    size: Vector2,
}

#[derive(Default)]
struct SUPERULTRADUPERCOOLSHOOTINGOBJECTFORNOREASONATALL {
    position: Vector2,
    speed: f32,
    health: u8,
    radius: f32
}

#[derive(Default)]
struct Enemy {
    position: Vector2,
    speed: f32,
    radius: f32,
    active: bool
}

#[derive(Default)]
struct Bullet {
    position: Vector2,
    speed: f32,
    radius: f32,
    active: bool
}

struct Game {
    player: Player,
    superultradupercoolshootingobjectfornoreasonatall: SUPERULTRADUPERCOOLSHOOTINGOBJECTFORNOREASONATALL,
    enemy: Vec<Enemy>,
    bullet: Vec<Bullet>,
    gamestate: GameState,
    score: u32
}

impl Default for Game {
    fn default() -> Self {
        let mut bullets = Vec::new();
        let mut enemies = Vec::new();

        for _ in 0..MAX_BULLETS {
            bullets.push(Bullet::default());
        }

        for _ in 0..MAX_ENEMIES {
            enemies.push(Enemy::default());
        }

        Game {
            player: Player::default(),
            superultradupercoolshootingobjectfornoreasonatall: SUPERULTRADUPERCOOLSHOOTINGOBJECTFORNOREASONATALL::default(),
            enemy: enemies,
            bullet: bullets,
            gamestate: GameState::Menu,
            score: 0
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
    .size(SCREEN_WIDTH, SCREEN_HEIGHT)
    .title("HELP!")
    .build();

    rl.set_target_fps(60);
    let mut game = Game::default();

    while !rl.window_should_close() {
        update_game(&mut game, &rl);
        draw_stuff(&game, &mut rl, &thread);
    }
}

fn init_game(game: &mut Game, rl: &RaylibHandle) {
    //initialize player
    game.player.position = Vector2::new(SCREEN_WIDTH as f32 / 2., (SCREEN_HEIGHT as f32 - 150.) + 100. );
    game.player.size = Vector2::new(20., 20.);
    game.player.life = PLAYER_LIFE;
    game.player.speed = 10.;

    //initialize bullets
    for bullets in &mut game.bullet {
        bullets.position = Vector2::default();
        bullets.speed = 40.;
        bullets.active = false;
        bullets.radius = 10.;
    }

    //I refuse to write it's name
    game.superultradupercoolshootingobjectfornoreasonatall.position = Vector2::new(100., 100.);
    game.superultradupercoolshootingobjectfornoreasonatall.speed = 5.;
    game.superultradupercoolshootingobjectfornoreasonatall.health = SUPERULTRADUPERCOOLSHOOTINGOBJECTFORNOREASONATALLHEALTH;
    game.superultradupercoolshootingobjectfornoreasonatall.radius = 25.;

    //initialize enemies
    for enemies in &mut game.enemy {
        enemies.position = Vector2::new(
            rl.get_random_value::<i32>(Range {start: 10, end: SCREEN_WIDTH - 10}) as f32,
            rl.get_random_value::<i32>(Range {start: 100, end: 200}) as f32
        );
        enemies.speed = 10.;
        enemies.active = true;
        enemies.radius = 20.;
    }
}

fn update_game(game: &mut Game, rl: &RaylibHandle) {
    match game.gamestate {
        GameState::Menu => {
            //play game if pressed enter
            if rl.is_key_pressed(KEY_ENTER) {
                init_game(game, rl);
                game.gamestate = GameState::InGame;
            }
        },
    
        GameState::InGame => {
            //player movement: stop if pressed both A and D or arrow left and arrow right
            if (rl.is_key_down(KEY_A) || rl.is_key_down(KEY_LEFT)) && !rl.is_key_down(KEY_D) && !rl.is_key_down(KEY_RIGHT) {
                game.player.position.x -= game.player.speed;
            } else if (rl.is_key_down(KEY_D) || rl.is_key_down(KEY_RIGHT)) && !rl.is_key_down(KEY_A) && !rl.is_key_down(KEY_LEFT) {
                game.player.position.x += game.player.speed;
            }
    
            //shoot bullets
            if rl.is_key_pressed(KEY_SPACE) {
                for bullets in &mut game.bullet {
                    if !bullets.active {
                        bullets.position = Vector2::new(
                            game.player.position.x,
                            game.player.position.y
                        );
    
                        bullets.active = true;
    
                        //shoot bullets one by one so they don't overlap each other and go all at once
                        break;
                    }
                }
            }
    
            if game.player.position.x > SCREEN_WIDTH as f32 {
                game.player.position.x = 5.;
            } else if game.player.position.x < 0. {
                game.player.position.x = SCREEN_WIDTH as f32 - 5.;
            }
            
            if game.score >= 20 {
                game.gamestate = GameState::Win;
            }
    
            //bullets go brrrrrr
            for bullets in &mut game.bullet {
                if bullets.active {
                    bullets.position.y -= bullets.speed;
                    
                    //limit bullets' travel distance so they don't kill memory lol
                    if bullets.position.y <= 0. {
                        //reset to bullets' default position so we can reuse it
                        bullets.position = Vector2::default();
                        //deactive it and wait for player to shoot
                        bullets.active = false;
                    }
    
                    //check collision with superultradupercoolshootingobjectfornoreasonatall
                    if game.superultradupercoolshootingobjectfornoreasonatall.health > 0 && check_collision_circles(bullets.position, bullets.radius, game.superultradupercoolshootingobjectfornoreasonatall.position, game.superultradupercoolshootingobjectfornoreasonatall.radius) {
                        game.superultradupercoolshootingobjectfornoreasonatall.health -= 1;
                        bullets.position = Vector2::default();
                        bullets.active = false;
                    }
    
                    //check collision with enemies
                    for enemies in &mut game.enemy {
                        if enemies.active && check_collision_circles(bullets.position, bullets.radius, enemies.position, enemies.radius) {
                            bullets.position = Vector2::default();
                            bullets.active = false;
                            enemies.active = false;
                            game.score += 1;
                        }
                    }
                }
            }
    
            //if superultradupercoolshootingobjectfornoreasonatall hits left limit then move to right and vice versa
            if game.superultradupercoolshootingobjectfornoreasonatall.health > 0 {
                if game.superultradupercoolshootingobjectfornoreasonatall.position.x <= 0. || game.superultradupercoolshootingobjectfornoreasonatall.position.x >= SCREEN_WIDTH as f32 - 5. {
                    game.superultradupercoolshootingobjectfornoreasonatall.speed *= -1.;
                }
    
                game.superultradupercoolshootingobjectfornoreasonatall.position.x += game.superultradupercoolshootingobjectfornoreasonatall.speed;
            }
    
            if game.superultradupercoolshootingobjectfornoreasonatall.health <= 0 {
                game.score += 10;
                game.superultradupercoolshootingobjectfornoreasonatall.health = 10;
            }
        },
    
        GameState::Win => {
            if rl.is_key_pressed(KEY_ENTER) {
                game.score = 0;
                game.gamestate = GameState::Menu;
            }
        }
    }
}

fn draw_stuff(game: &Game, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = rl.begin_drawing(thread);

    match game.gamestate {

        GameState::Menu => {
            //draw game menu
            d.clear_background(Color::WHITE);
            d.draw_text("HELLO DUDE IT'S not ME MARIO!", 250, 100, 20, Color::BLUE);
        },
    
        GameState::InGame => {
            //set background colour
            d.clear_background(Color::BLUE);
    
            //draw player
            d.draw_rectangle(game.player.position.x as i32, game.player.position.y as i32, game.player.size.x as i32, game.player.size.y as i32, Color::BLACK);
    
            //draw player lives
            d.draw_text(&game.player.life.to_string(), SCREEN_WIDTH - 100, SCREEN_HEIGHT - 100, 20, Color::BLACK);
    
            //draw score
            d.draw_text(&game.score.to_string(), 10, 10, 20, Color::BLACK);
    
            //draw that thing's health
            d.draw_text(&game.superultradupercoolshootingobjectfornoreasonatall.health.to_string(), SCREEN_WIDTH - 20, 10, 20, Color::RED);
    
            //draw bullets
            for bullets in &game.bullet {
                if bullets.active {
                    // d.draw_rectangle(bullets.position.x as i32, bullets.position.y as i32, bullets.size.x as i32, bullets.size.y as i32, Color::RED);
                    d.draw_circle_v(bullets.position, bullets.radius, Color::RED);
                }
            }
    
            //draw that thing
            if game.superultradupercoolshootingobjectfornoreasonatall.health > 0 {
                // d.draw_rectangle(game.superultradupercoolshootingobjectfornoreasonatall.position.x as i32, game.superultradupercoolshootingobjectfornoreasonatall.position.y as i32, game.superultradupercoolshootingobjectfornoreasonatall.radius.x as i32, game.superultradupercoolshootingobjectfornoreasonatall.size.y as i32, Color::RED);
                d.draw_circle_v(game.superultradupercoolshootingobjectfornoreasonatall.position, game.superultradupercoolshootingobjectfornoreasonatall.radius, Color::RED);
            }
    
            for enemies in &game.enemy {
                if enemies.active {
                    d.draw_circle_v(enemies.position, enemies.radius, Color::WHITE);
                }
            }
        },
    
        GameState::Win => {
            d.clear_background(Color::YELLOW);
            d.draw_text("YOU WON HOORAY!", SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, 30, Color::BLUE);
        }
    }
}