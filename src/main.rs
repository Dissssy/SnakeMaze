use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use rayon::prelude::*;
use soloud::*;
use std::thread;
use std::time::{Duration, Instant};

const WIDTH: usize = 16;
const HEIGHT: usize = 16;
const MAZECOLOR: u32 = 150 << 16 | 150 << 8 | 150 << 0;
const WALLCOLOR: u32 = 200 << 16 | 200 << 8 | 200 << 0;
const SNAKEBODYCOLOR: u32 = 0 << 16 | 63 << 8 | 0 << 0;
// const RED: u32 = 255 << 16 | 0 << 8 | 0 << 0;
const RED: u32 = (127 + 63) << 16 | 0 << 8 | 0 << 0;
const LIGHTRED: u32 = 63 << 16 | 0 << 8 | 0 << 0;
// const POOP: u32 = 165 << 16 | 42 << 8 | 42 << 0;
// const POOP: u32 = 212 << 16 | 175 << 8 | 55 << 0;
const POOP: u32 = 0 << 16 | 0 << 8 | 127 << 0;
const DELTA: i16 = 20;
const FRAMETIMEMS: u128 = 120;
const SHHH: bool = true;

fn main() {
    let sl = Soloud::default().unwrap();
    let mut eatsounds = Vec::new();
    let mut s = audio::Wav::default();
    s.load_mem(include_bytes!("./resources/eat.wav").to_vec())
        .unwrap();
    eatsounds.push(s);
    let mut s = audio::Wav::default();
    s.load_mem(include_bytes!("./resources/1.wav").to_vec())
        .unwrap();
    eatsounds.push(s);
    let mut s = audio::Wav::default();
    s.load_mem(include_bytes!("./resources/2.wav").to_vec())
        .unwrap();
    eatsounds.push(s);
    let mut s = audio::Wav::default();
    s.load_mem(include_bytes!("./resources/3.wav").to_vec())
        .unwrap();
    eatsounds.push(s);
    let mut s = audio::Wav::default();
    s.load_mem(include_bytes!("./resources/4.wav").to_vec())
        .unwrap();
    eatsounds.push(s);
    let mut s = audio::Wav::default();
    s.load_mem(include_bytes!("./resources/5.wav").to_vec())
        .unwrap();
    eatsounds.push(s);
    let mut s = audio::Wav::default();
    s.load_mem(include_bytes!("./resources/6.wav").to_vec())
        .unwrap();
    eatsounds.push(s);
    let mut poop = audio::Wav::default();
    poop.load_mem(include_bytes!("./resources/poop.wav").to_vec())
        .unwrap();
    let mut breaksounds = Vec::new();
    let mut s = audio::Wav::default();
    s.load_mem(include_bytes!("./resources/break1.wav").to_vec())
        .unwrap();
    breaksounds.push(s);
    let mut s = audio::Wav::default();
    s.load_mem(include_bytes!("./resources/break2.wav").to_vec())
        .unwrap();
    breaksounds.push(s);
    let mut s = audio::Wav::default();
    s.load_mem(include_bytes!("./resources/break3.wav").to_vec())
        .unwrap();
    breaksounds.push(s);
    let mut rng = rand::thread_rng();
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "maze snake (arrow keys to move, space to restart, z to pause, esc to exit)",
        WIDTH,
        HEIGHT,
        WindowOptions {
            // borderless: true,
            resize: true,
            scale: minifb::Scale::X16,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.limit_update_rate(Some(std::time::Duration::from_micros(0)));
    let mut setup: bool = true;
    let mut snake: Vec<[i16; 2]> = Vec::new();
    let mut direction: [i16; 2] = [0, 0];
    let wall: Vec<[i16; 2]> = generate_wall();
    let mut maze: Vec<[i16; 2]> = Vec::new();
    let mut framecount: i16 = 0;
    let mut movemade = false;
    let mut gameover = false;
    let mut apple = [0i16, 0i16];
    let mut skip = 0;
    let mut pause = true;
    let mut fuck = false;
    let mut score: u32 = 2;
    let mut multiplier: u32 = 0;
    let mut charges: u32 = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut now = Instant::now();
        if setup {
            let mut metavalid = false;
            while !metavalid {
                snake = Vec::new();
                let mut valid = false;
                let mut newpos = [0i16, 0i16];
                while !valid {
                    newpos = [
                        rng.gen_range(0..WIDTH) as i16,
                        rng.gen_range(0..HEIGHT) as i16,
                    ];
                    if !wall.contains(&newpos) {
                        valid = true;
                    }
                }
                snake.push(newpos);
                snake.push(newpos);
                // valid = false;
                // let mut mask = [0i16, 0i16];
                // while !valid {
                //     mask = [0i16, 0i16];
                //     if rng.gen::<bool>() {
                //         if rng.gen::<bool>() {
                //             mask[0] = 1;
                //         } else {
                //             mask[0] = -1;
                //         }
                //     } else if rng.gen::<bool>() {
                //         mask[1] = 1;
                //     } else {
                //         mask[1] = -1;
                //     }
                //     if !wall.contains(&[snake[0][0] + mask[0], snake[0][1] + mask[1]]) {
                //         valid = true;
                //     }
                // }
                // snake.push([newpos[0] + mask[0], newpos[1] + mask[1]]);
                // direction = [-mask[0], -mask[1]];
                direction = [0, 0];
                if !wall.contains(&[
                    snake[snake.len() - 1][0] + -direction[0],
                    snake[snake.len() - 1][1] + -direction[1],
                ]) {
                    metavalid = true;
                }
            }
            let mut valid = false;
            let mut newpos: [i16; 2] = [0i16, 0i16];
            while !valid {
                newpos = [
                    rng.gen_range(0..WIDTH) as i16,
                    rng.gen_range(0..HEIGHT) as i16,
                ];
                if !(maze.contains(&newpos) || snake.contains(&newpos) || wall.contains(&newpos)) {
                    valid = true;
                }
            }
            apple = newpos;
            setup = false;
            if !SHHH {
                println!("setup took {}", now.elapsed().as_secs_f64())
            }
            now = Instant::now();
        }
        let timedelta = Instant::now();
        window.update();
        if direction == [0, 0] {
            if window.is_key_down(Key::Up) && !movemade {
                direction = [0, 1];
                movemade = true;
                if !SHHH {
                    println!("{:?}", direction)
                }
            }
            if window.is_key_down(Key::Down) && !movemade {
                direction = [0, -1];
                movemade = true;
                if !SHHH {
                    println!("{:?}", direction)
                }
            }
            if window.is_key_down(Key::Left) && !movemade {
                direction = [1, 0];
                movemade = true;
                if !SHHH {
                    println!("{:?}", direction)
                }
            }
            if window.is_key_down(Key::Right) && !movemade {
                direction = [-1, 0];
                movemade = true;
                if !SHHH {
                    println!("{:?}", direction)
                }
            }
        } else if direction[0] != 0 && !movemade {
            if window.is_key_down(Key::Up) && !movemade {
                direction = [0, 1];
                movemade = true;
                if !SHHH {
                    println!("{:?}", direction)
                }
            }
            if window.is_key_down(Key::Down) && !movemade {
                direction = [0, -1];
                movemade = true;
                if !SHHH {
                    println!("{:?}", direction)
                }
            }
        } else {
            if window.is_key_down(Key::Left) && !movemade {
                direction = [1, 0];
                movemade = true;
                if !SHHH {
                    println!("{:?}", direction)
                }
            }
            if window.is_key_down(Key::Right) && !movemade {
                direction = [-1, 0];
                movemade = true;
                if !SHHH {
                    println!("{:?}", direction)
                }
            }
        }
        if framecount == 0 && !gameover {
            buffer.par_iter_mut().for_each(|v| *v = 0u32);
            if !pause {
                if !SHHH {
                    println!("keylogic took {}", now.elapsed().as_secs_f64())
                }
                now = Instant::now();
                movemade = false;
                if !SHHH {
                    println!("{:?}", direction)
                }
                let newpos = [
                    snake[snake.len() - 1][0] + -direction[0],
                    snake[snake.len() - 1][1] + -direction[1],
                ];
                let mut nextsnake = snake.clone();
                nextsnake.drain(0..1);
                let remove: bool = maze.contains(&newpos) && charges > 0;
                if nextsnake.contains(&newpos)
                    || wall.contains(&newpos)
                    || (maze.contains(&newpos) && charges == 0)
                {
                    gameover = true;
                    pause = true;
                } else {
                    if apple == newpos {
                        let mut sound = multiplier;
                        if sound >= eatsounds.len() as u32 {
                            println!("{}", sound);
                            sound = eatsounds.len() as u32 - 1;
                        }
                        sl.play(&eatsounds[sound as usize]);
                        maze.push(apple);
                        skip += 2u32.pow(multiplier + 1) - 1;
                        let mut valid = false;
                        let mut newapple: [i16; 2] = [0i16, 0i16];
                        while !valid {
                            newapple = [
                                rng.gen_range(0..WIDTH) as i16,
                                rng.gen_range(0..HEIGHT) as i16,
                            ];
                            if !(maze.contains(&newapple)
                                || snake.contains(&newapple)
                                || wall.contains(&newapple))
                            {
                                valid = true;
                            }
                        }
                        apple = newapple;
                    }
                    if !pause {
                        if remove {
                            maze.remove(maze.iter().position(|&r| r == newpos).unwrap());
                            charges -= 1;
                            score -= 1;
                            sl.play(&breaksounds[rng.gen_range(0..breaksounds.len())]);
                        }
                        snake.push(newpos);
                        if !gameover {
                            if skip > 0 {
                                if skip > 1 {
                                    charges += 1;
                                }
                                skip -= 1;
                                score += 1;
                            } else {
                                snake.drain(0..1);
                            }
                        }
                    }
                }
            }
            for (i, v) in snake.iter().enumerate() {
                if i < charges as usize {
                    for _ in 0..(3 - ((snake.len() - 1 - i) % 3)) {
                        buffer[xy_to_index(*v)] += LIGHTRED;
                    }
                } else {
                    if i == snake.len() - 1 {
                        buffer[xy_to_index(*v)] += SNAKEBODYCOLOR;
                    }
                    for _ in 0..(3 - ((snake.len() - 1 - i) % 3)) {
                        buffer[xy_to_index(*v)] += SNAKEBODYCOLOR;
                    }
                }
            }
            let lastcount = multiplier;
            multiplier = 0;
            for v in maze.iter() {
                if !snake.contains(v) {
                    buffer[xy_to_index(*v)] = MAZECOLOR;
                } else {
                    buffer[xy_to_index(*v)] += POOP;
                    multiplier += 1;
                }
            }
            for v in wall.iter() {
                buffer[xy_to_index(*v)] = WALLCOLOR;
            }
            if multiplier < lastcount {
                sl.play(&poop);
            }
            buffer[xy_to_index(apple)] = RED;
            if !SHHH {
                println!("frame took {}\n", now.elapsed().as_secs_f64())
            }
            if gameover {
                buffer
                    .par_iter_mut()
                    .enumerate()
                    .for_each(|(_i, v)| *v += LIGHTRED);
            }
            window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        }
        let delta = Duration::from_millis(
            (FRAMETIMEMS as u64 / DELTA as u64) - timedelta.elapsed().as_millis() as u64,
        );
        if delta.as_millis() < (FRAMETIMEMS / DELTA as u128) + 1 {
            thread::sleep(delta);
        }
        framecount = (framecount + 1) % DELTA;
        if pause
            && (window.is_key_down(Key::Up)
                || window.is_key_down(Key::Down)
                || window.is_key_down(Key::Left)
                || window.is_key_down(Key::Right))
        {
            pause = false;
        }
        if window.is_key_down(Key::Space) && !fuck {
            fuck = true;

            setup = true;
            snake = Vec::new();
            direction = [0, 0];
            maze = Vec::new();
            framecount = 0;
            movemade = false;
            gameover = false;
            apple = [0i16, 0i16];
            skip = 0;
            pause = true;
            score = 2;
            multiplier = 0;
            charges = 0;
        } else if !window.is_key_down(Key::Space) {
            fuck = false;
        }
        if window.is_key_down(Key::Z) && !pause {
            pause = true;
        }
        if pause || gameover {
            window.set_title(
                format!(
                    "Maze Snake | Score: {} | (arrow keys to move, space to restart, z to pause, esc to exit)", score
                )
                .as_str(),
            );
        } else {
            window.set_title(format!("Maze Snake | Score: {}", score).as_str());
        }
    }
}

fn xy_to_index(xy: [i16; 2]) -> usize {
    xy[0] as usize + (xy[1] as usize * WIDTH)
}

fn generate_wall() -> Vec<[i16; 2]> {
    let mut x: Vec<[i16; 2]> = Vec::new();
    for i in 0..WIDTH {
        x.push([i as i16, 0]);
        x.push([i as i16, HEIGHT as i16 - 1]);
    }
    for i in 0..HEIGHT {
        x.push([0, i as i16]);
        x.push([WIDTH as i16 - 1, i as i16]);
    }
    x
}
