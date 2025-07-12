use raylib::prelude::*;
use std::thread;
use std::time::Duration;

mod framebuffer;
use framebuffer::Framebuffer;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

// Estado inicial
fn load_glider(state: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    let pattern = vec![
        (0, 1),
        (1, 2),
        (2, 0),
        (2, 1),
        (2, 2),
    ];
    for (dx, dy) in pattern {
        if y + dy < HEIGHT && x + dx < WIDTH {
            state[y + dy][x + dx] = true;
        }
    }
}

// Contar vecinos vivos (bordes toroidales)
fn count_alive_neighbors(state: &Vec<Vec<bool>>, x: usize, y: usize) -> u8 {
    let mut count = 0;
    for dy in [-1isize, 0, 1].iter() {
        for dx in [-1isize, 0, 1].iter() {
            if *dx == 0 && *dy == 0 {
                continue;
            }
            let nx = ((x as isize + dx + WIDTH as isize) % WIDTH as isize) as usize;
            let ny = ((y as isize + dy + HEIGHT as isize) % HEIGHT as isize) as usize;
            if state[ny][nx] {
                count += 1;
            }
        }
    }
    count
}

// Renderizar y calcular nuevo estado (con colores por índice)
fn render(
    framebuffer: &mut Framebuffer,
    state: &mut Vec<Vec<bool>>,
    state_color: &mut Vec<Vec<Option<usize>>>,
) {
    let mut new_state = state.clone();
    let mut new_state_color = state_color.clone();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let alive = state[y][x];
            let neighbors = count_alive_neighbors(state, x, y);

            new_state[y][x] = match (alive, neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };

            // Si la célula revive, hereda el color más común de sus vecinos
            if !alive && new_state[y][x] {
                let mut color_count = std::collections::HashMap::new();
                for dy in [-1isize, 0, 1].iter() {
                    for dx in [-1isize, 0, 1].iter() {
                        if *dx == 0 && *dy == 0 { continue; }
                        let nx = ((x as isize + dx + WIDTH as isize) % WIDTH as isize) as usize;
                        let ny = ((y as isize + dy + HEIGHT as isize) % HEIGHT as isize) as usize;
                        if let Some(color_idx) = state_color[ny][nx] {
                            *color_count.entry(color_idx).or_insert(0) += 1;
                        }
                    }
                }
                // El color más frecuente
                let mut max_color = None;
                let mut max_count = 0;
                for (color_idx, count) in color_count {
                    if count > max_count {
                        max_count = count;
                        max_color = Some(color_idx);
                    }
                }
                new_state_color[y][x] = max_color;
            } else if new_state[y][x] {
                // Si sigue viva, mantiene su color
                new_state_color[y][x] = state_color[y][x];
            } else {
                // Si muere, no tiene color
                new_state_color[y][x] = None;
            }
        }
    }

    // Dibujar nueva generación
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if new_state[y][x] {
                let color = new_state_color[y][x]
                    .map(|idx| PATTERN_COLORS[idx % PATTERN_COLORS.len()])
                    .unwrap_or(Color::WHITE);
                framebuffer.point(x as i32, y as i32, color);
            } else {
                framebuffer.point(x as i32, y as i32, Color::BLACK);
            }
        }
    }

    *state = new_state;
    *state_color = new_state_color;
}

// Patrones clásicos
fn pattern_block() -> Vec<(usize, usize)> {
    vec![(0,0), (0,1), (1,0), (1,1)]
}

fn pattern_beehive() -> Vec<(usize, usize)> {
    vec![(1,0), (2,0), (0,1), (3,1), (1,2), (2,2)]
}

fn pattern_loaf() -> Vec<(usize, usize)> {
    vec![(1,0), (2,0), (0,1), (3,1), (1,2), (3,2), (2,3)]
}

fn pattern_boat() -> Vec<(usize, usize)> {
    vec![(0,0), (1,0), (0,1), (2,1), (1,2)]
}

fn pattern_tub() -> Vec<(usize, usize)> {
    vec![(1,0), (0,1), (2,1), (1,2)]
}

fn pattern_blinker() -> Vec<(usize, usize)> {
    vec![(0,1), (1,1), (2,1)]
}

fn pattern_toad() -> Vec<(usize, usize)> {
    vec![(1,0), (2,0), (3,0), (0,1), (1,1), (2,1)]
}

fn pattern_beacon() -> Vec<(usize, usize)> {
    vec![(0,0), (1,0), (0,1), (1,1), (2,2), (3,2), (2,3), (3,3)]
}

fn pattern_pulsar() -> Vec<(usize, usize)> {
    let base = [
        (2,0),(3,0),(4,0),(8,0),(9,0),(10,0),
        (0,2),(5,2),(7,2),(12,2),
        (0,3),(5,3),(7,3),(12,3),
        (0,4),(5,4),(7,4),(12,4),
        (2,5),(3,5),(4,5),(8,5),(9,5),(10,5),
        (2,7),(3,7),(4,7),(8,7),(9,7),(10,7),
        (0,8),(5,8),(7,8),(12,8),
        (0,9),(5,9),(7,9),(12,9),
        (0,10),(5,10),(7,10),(12,10),
        (2,12),(3,12),(4,12),(8,12),(9,12),(10,12)
    ];
    base.to_vec()
}

fn pattern_glider() -> Vec<(usize, usize)> {
    vec![(0,1), (1,2), (2,0), (2,1), (2,2)]
}

fn pattern_lwss() -> Vec<(usize, usize)> {
    vec![(1,0),(4,0),(0,1),(0,2),(4,2),(0,3),(1,3),(2,3),(3,3)]
}

fn pattern_mwss() -> Vec<(usize, usize)> {
    vec![(1,0),(2,0),(3,0),(4,0),(5,0),(0,1),(0,2),(5,2),(0,3),(1,3),(2,3),(3,3),(4,3)]
}

fn pattern_hwss() -> Vec<(usize, usize)> {
    vec![(1,0),(2,0),(3,0),(4,0),(5,0),(6,0),(0,1),(0,2),(6,2),(0,3),(1,3),(2,3),(3,3),(4,3),(5,3)]
}

// Paleta de colores para patrones
const PATTERN_COLORS: [Color; 13] = [
    Color::RED,
    Color::ORANGE,
    Color::YELLOW,
    Color::GREEN,
    Color::BLUE,
    Color::PURPLE,
    Color::PINK,
    Color::LIME,
    Color::SKYBLUE,
    Color::VIOLET,
    Color::BROWN,
    Color::GOLD,
    Color::MAROON,
];

// Coloca un patrón en el estado en la posición (x, y) y colorea las células
fn load_pattern(
    state: &mut Vec<Vec<bool>>,
    state_color: &mut Vec<Vec<Option<usize>>>,
    pattern: &[(usize, usize)],
    x: usize,
    y: usize,
    color_idx: usize,
) {
    for &(dx, dy) in pattern {
        let nx = x + dx;
        let ny = y + dy;
        if nx < WIDTH && ny < HEIGHT {
            state[ny][nx] = true;
            state_color[ny][nx] = Some(color_idx);
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 800;

    let (mut window, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Conway's Game of Life")
        .build();

    let mut framebuffer = Framebuffer::new(WIDTH as u32, HEIGHT as u32);

    // Estado inicial
    let mut state = vec![vec![false; WIDTH]; HEIGHT];
    let mut state_color = vec![vec![None; WIDTH]; HEIGHT];

    // Selecciona 10 patrones distintos de los implementados
    let patterns: Vec<fn() -> Vec<(usize, usize)>> = vec![
        pattern_block,
        pattern_beehive,
        pattern_loaf,
        pattern_boat,
        pattern_tub,
        pattern_blinker,
        pattern_toad,
        pattern_beacon,
        pattern_glider,
        pattern_lwss,
    ];

    // Tamaño máximo de patrón (el más grande de los 10)
    let pattern_size = 6; // El lwss es 5x4, beacon 4x4, toad 4x2, etc.
    let step = pattern_size + 2; // Espacio pequeño para mayor densidad
    let mut pattern_idx = 0;

    for y in (0..HEIGHT-step).step_by(step) {
        for x in (0..WIDTH-step).step_by(step) {
            let pattern = patterns[pattern_idx % patterns.len()]();
            let color_idx = pattern_idx % PATTERN_COLORS.len();
            load_pattern(&mut state, &mut state_color, &pattern, x, y, color_idx);
            pattern_idx += 1;
        }
    }

    while !window.window_should_close() {
        render(&mut framebuffer, &mut state, &mut state_color);
        framebuffer.swap_buffers(&mut window, &thread);
        thread::sleep(Duration::from_millis(100));
    }
}
