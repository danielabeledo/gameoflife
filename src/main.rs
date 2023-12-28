use std::ffi::c_int;

use beryllium::events::{Event, SDLK_q};
use beryllium::init::InitFlags;
use beryllium::Sdl;
use beryllium::video::{CreateWinArgs, RendererFlags};
use fermium::prelude::{SDLK_DOWN, SDLK_LEFT, SDLK_RIGHT, SDLK_SPACE, SDLK_UP};
use rand::prelude::ThreadRng;
use rand::Rng;

// this is the length in pixels of the side of the square representing a cell.
// it's probably a good idea for this number to be a factor of the canvas/windows size
const ZOOM: usize = 5;
// HEIGHT and WIDTH are the values that represent the size of the world where the
// cells live
const HEIGHT: usize = 2000;
const WIDTH: usize = 2000;
// WIN HEIGHT/WIDTH represents the size of the windows that will be rendered
const WIN_HEIGHT: usize = 500;
const WIN_WIDTH: usize = 500;
// RATIO is the percentage of alive cells that will be generated in Step 0.
// Very Small or Large values of RATIO will kill the colony in the first few steps.
const RATIO: f64 = 0.2f64;
// Step size represents how much the window moves when pressing the arrow keys.
const STEP: usize = 10;

fn main() {
    // Creating Window
    let sdl = Sdl::init(InitFlags::VIDEO);
    let window = sdl.create_renderer_window(
        CreateWinArgs {
            title: "Game of Life",
            height: WIN_HEIGHT as i32,
            width: WIN_WIDTH as i32,
            allow_high_dpi: false,
            resizable: true,
            borderless: true,
        },
        RendererFlags::SOFTWARE,
    ).unwrap();

    // Generating a random initial Cell state
    let mut state: Vec<bool> = Vec::from([false; WIDTH * HEIGHT]);
    let mut rng: ThreadRng = rand::thread_rng();
    for index in 0..WIDTH * HEIGHT {
        state[index] = rng.gen_bool(RATIO)
    }
    // window state
    let mut wx: usize = 0;
    let mut wy: usize = 0;
    // pausing state
    let mut running = true;

    'main: loop {
        //Processing Events
        while let Some((event, _timestamp)) = sdl.poll_events() {
            match event {
                #[allow(non_upper_case_globals)]
                Event::Quit | Event::Key {
                    keycode: SDLK_q, ..
                } => break 'main,
                Event::Key { pressed: true, keycode, .. } => {
                    if keycode == SDLK_RIGHT && wx + STEP < WIDTH - WIN_WIDTH { wx += STEP }
                    else if keycode == SDLK_LEFT && wx > STEP { wx -= STEP }
                    else if keycode == SDLK_UP && wy > STEP { wy -= STEP }
                    else if keycode == SDLK_DOWN && wy + STEP < HEIGHT - WIN_HEIGHT { wy += STEP }
                    else if keycode == SDLK_SPACE { running = !running }
                }
                _ => (),
            }
        }

        // if it's not paused, we will render a new state
        if running {
            // clearing window for next drawing
            window.set_draw_color(u8::MAX, u8::MAX, u8::MAX, u8::MAX).unwrap();
            window.clear().unwrap();
            // setting colour to black
            window.set_draw_color(0, 0, 0, u8::MAX).unwrap();
            // Allocating new vector with next step state.
            let mut new_state = Vec::with_capacity(state.len());
            // iterating all elements
            for (index, cell) in state.iter().enumerate() {

                // calculating position in canvas
                let x = index % WIDTH;
                let y = index / WIDTH;

                // number of alive neighbours
                let mut count: u8 = 0;
                // top left
                if state[(if y == 0 { HEIGHT - 1 } else { y - 1 }) * WIDTH + if x == 0 { WIDTH - 1 } else { x - 1 }] {
                    count += 1;
                }
                // top
                if state[(if y == 0 { HEIGHT - 1 } else { y - 1 }) * WIDTH + x] {
                    count += 1;
                }
                // top right
                if state[(if y == 0 { HEIGHT - 1 } else { y - 1 }) * WIDTH + if x == WIDTH - 1 { 0 } else { x + 1 }] {
                    count += 1;
                }
                // right
                if state[y * WIDTH + if x == WIDTH - 1 { 0 } else { x + 1 }] {
                    count += 1;
                }
                // bottom right
                if state[(if y == HEIGHT - 1 { 0 } else { y + 1 }) * WIDTH + if x == WIDTH - 1 { 0 } else { x + 1 }] {
                    count += 1;
                }
                // bottom
                if state[(if y == HEIGHT - 1 { 0 } else { y + 1 }) * WIDTH + x] {
                    count += 1
                }
                // bottom left
                if state[(if y == HEIGHT - 1 { 0 } else { y + 1 }) * WIDTH + if x == 0 { WIDTH - 1 } else { x - 1 }] {
                    count += 1
                }
                // left
                if state[y * WIDTH + if x == 0 { WIDTH - 1 } else { x - 1 }] {
                    count += 1
                }

                // calculating next value of the cell in coordinates x,y
                // if cell is alive and has 2 or 3 neighbours, it survives
                // if no cell is alive in this square but has 3 alive neighbours, it will be born
                // any other scenario, the cell in this space dies
                let alive: bool = if *cell { count == 2 || count == 3 } else { count == 3 };

                // drawing the cell when it's alive and in window range
                if alive && x >= wx && x < wx + WIN_WIDTH && y >= wy && y < wy + WIN_HEIGHT {
                    // drawing a rectangle of size ZOOM in the x,y coordinates
                    window.fill_rects(&[[
                        (ZOOM * (x - wx)) as c_int,
                        (ZOOM * (y - wy)) as c_int,
                        ZOOM as c_int,
                        ZOOM as c_int]]).unwrap()
                }
                // updating new state for next iteration
                new_state.push(alive);

            }
            // new state fully calculated
            state = new_state;
        }
        // drawing frame
        window.present();
    }
}
