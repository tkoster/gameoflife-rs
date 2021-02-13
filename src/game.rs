use sdl2::event::Event;
use sdl2::event::EventPollIterator;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;

pub struct ImageBuffer<'a> {
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
    pub pixels: &'a mut [u8]
}

pub struct GameState {
    pause: bool,
    mouse_position: (u32, u32),
    mouse_was_pressed: bool,
    frame: u32,
    width: u32,
    height: u32,
    cell_size: u32,
    gen0: Vec<u32>,
    gen1: Vec<u32>,
    selected_pattern_number: usize,
    selected_rotation_number: usize
}

pub fn init(window_size: (u32, u32), cell_size: u32) -> GameState {
    let (window_width, window_height) = window_size;
    let width = window_width / cell_size;
    let height = window_height / cell_size;
    let num_cells = (window_width * window_height) as usize;
    let gen0 = vec![0; num_cells];
    let gen1 = vec![0; num_cells];

    GameState {
        mouse_position: (window_width / 2, window_height / 2),
        mouse_was_pressed: false,
        pause: false,
        frame: 1,
        width,
        height,
        cell_size,
        gen0,
        gen1,
        selected_pattern_number: 2,
        selected_rotation_number: 0
    }
}

pub fn update(frame: u32, buffer: &mut ImageBuffer, state: &mut GameState, events: EventPollIterator) -> bool {
    state.frame = frame;

    let mut should_exit = false;
    let mut mouse_is_pressed = false;
    for event in events {
        match event {
            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                should_exit = true;
            },
            Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                state.selected_pattern_number = 0;
            },
            Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
                state.selected_pattern_number = 1;
            },
            Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {
                state.selected_pattern_number = 2;
            },
            Event::KeyDown { keycode: Some(Keycode::Backspace), .. } => {
                clear_generation(state);
            },
            Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                state.pause = !state.pause;
            },
            Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                state.selected_rotation_number = (state.selected_rotation_number + 1) % 4;
            },
            Event::MouseMotion { x, y, .. } => {
                state.mouse_position = (x as u32, y as u32);
            },
            Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                state.mouse_position = (x as u32, y as u32);
                mouse_is_pressed = mouse_btn == MouseButton::Left;
                state.mouse_was_pressed = mouse_btn == MouseButton::Right;
            },
            Event::MouseButtonUp { x, y, mouse_btn, .. } => {
                state.mouse_position = (x as u32, y as u32);
                state.mouse_was_pressed ^= mouse_btn == MouseButton::Right;
            },
            _ => {}
        }
    }

    if should_exit {
        return true;
    }

    let (mouse_x, mouse_y) = state.mouse_position;
    let mouse_cell_x = mouse_x / state.cell_size;
    let mouse_cell_y = mouse_y / state.cell_size;
    let selected_pattern = PATTERNS[state.selected_pattern_number];
    let selected_rotation = ROTATIONS[state.selected_rotation_number];
    if mouse_is_pressed || state.mouse_was_pressed {
        write_pattern(state, mouse_cell_x, mouse_cell_y, selected_pattern, selected_rotation, frame);
    }

    if !state.pause {
        std::mem::swap(&mut state.gen0, &mut state.gen1);
        step_generation(state, frame);
    }

    draw_generation(buffer, frame, state);
    draw_pattern(buffer, mouse_cell_x, mouse_cell_y, state.cell_size, &CROSSHAIR, ROTATIONS[0], Color::RGB(0x38, 0x38, 0x38));
    draw_pattern(buffer, mouse_cell_x, mouse_cell_y, state.cell_size, selected_pattern, selected_rotation, Color::RGB(0xf7, 0xca, 0x88));

    false
}

fn clear_generation(state: &mut GameState) {
    for y in 0 .. state.height {
        for x in 0 .. state.width {
            write_cell_unchecked(state, x, y, 0);
        }
    }
}

pub fn step_generation(state: &mut GameState, frame: u32) {
    for y in 1 .. state.height - 1 {
        for x in 1 .. state.width - 1 {
            let neighbours = count_neighbours(state, x, y);
            let offset = (y * state.width + x) as usize;
            let value = state.gen0[offset];
            let new_value =
                if value > 0 && (neighbours == 2 || neighbours == 3) {
                    value
                } else if value > 0 {
                    0
                } else if neighbours == 3 {
                    frame
                } else {
                    0
                };
            write_cell_unchecked(state, x, y, new_value);
        }
    }
}

fn count_neighbours(state: &GameState, x: u32, y: u32) -> u32 {
    let n0 = if read_cell(state, x - 1, y - 1) > 0 { 1 } else { 0 };
    let n1 = if read_cell(state, x    , y - 1) > 0 { 1 } else { 0 };
    let n2 = if read_cell(state, x + 1, y - 1) > 0 { 1 } else { 0 };
    let n3 = if read_cell(state, x - 1, y    ) > 0 { 1 } else { 0 };
    let n4 = if read_cell(state, x + 1, y    ) > 0 { 1 } else { 0 };
    let n5 = if read_cell(state, x - 1, y + 1) > 0 { 1 } else { 0 };
    let n6 = if read_cell(state, x    , y + 1) > 0 { 1 } else { 0 };
    let n7 = if read_cell(state, x + 1, y + 1) > 0 { 1 } else { 0 };
    n0 + n1 + n2 + n3 + n4 + n5 + n6 + n7
}

#[inline]
fn read_cell(state: &GameState, x: u32, y: u32) -> u32 {
    if x >= state.width || y >= state.height {
        0
    } else {
        let offset = (y * state.width + x) as usize;
        state.gen0[offset]
    }
}

fn write_pattern(state: &mut GameState, cell_x: u32, cell_y: u32, pattern: &Pattern, rotation: (i32, i32), value: u32) {
    let (rx, ry) = rotation;
    for (x, y) in pattern.points {
        let point_cell_x = (rx * (x - pattern.origin_x) + cell_x as i32) as u32;
        let point_cell_y = (ry * (y - pattern.origin_y) + cell_y as i32) as u32;
        write_cell_checked(state, point_cell_x, point_cell_y, value);
    }
}

#[inline]
fn write_cell_unchecked(state: &mut GameState, x: u32, y: u32, value: u32) {
    let offset = (y * state.width + x) as usize;
    unsafe {
        let cell = state.gen1.get_unchecked_mut(offset);
        *cell = value;
    }
}

#[inline]
fn write_cell_checked(state: &mut GameState, x: u32, y: u32, value: u32) {
    if x >= state.width || y >= state.height {
        return;
    }
    write_cell_unchecked(state, x, y, value);
}

struct Pattern<'a> {
    origin_x: i32,
    origin_y: i32,
    points: &'a[(i32, i32)]
}

const CROSSHAIR: Pattern = Pattern {
    origin_x: 0,
    origin_y: 0,
    points: &[
      (-20, 0), (-21, 0), (-22, 0), (-23, 0),
      (0, -20), (0, -21), (0, -22), (0, -23),
      (20, 0), (21, 0), (22, 0), (23, 0),
      (0, 20), (0, 21), (0, 22), (0, 23)
    ]
};

const GLIDER: Pattern = Pattern {
    origin_x: 1,
    origin_y: 1,
    points: &[(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)]
};

const GLIDER_MISTAKE: Pattern = Pattern {
    origin_x: 1,
    origin_y: 1,
    points: &[(1, 0), (2, 0), (2, 1), (0, 2), (1, 2), (2, 2)]
};

const GLIDER_GUN: Pattern = Pattern {
    origin_x: 18,
    origin_y: 5,
    points: &[
        (0,  5), (0,  6),
        (1,  5), (1,  6),
        (10, 5), (10, 6), (10, 7),
        (11, 4), (11, 8),
        (12, 3), (12, 9),
        (13, 3), (13, 9),
        (14, 6),
        (15, 4), (15, 8),
        (16, 5), (16, 6), (16, 7),
        (17, 6),
        (20, 3), (20, 4), (20, 5),
        (21, 3), (21, 4), (21, 5),
        (22, 2), (22, 6),
        (24, 1), (24, 2), (24, 6), (24, 7),
        (34, 3), (34, 4),
        (35, 3), (35, 4)
    ]
};

const PATTERNS: &[&Pattern] = &[&GLIDER, &GLIDER_MISTAKE, &GLIDER_GUN];

const ROTATIONS: &[(i32, i32)] = &[(1, 1), (1, -1), (-1, -1), (-1, 1)];

pub fn draw_generation(buffer: &mut ImageBuffer, frame: u32, state: &GameState) {
    if state.width == 0 || state.height == 0 {
        return;
    }

    // TODO: Draw in scanlines for better cache utilisation.
    for y in 0 .. state.height {
        for x in 0 .. state.width {
            let offset = (y * state.width + x) as usize;
            let value = state.gen1[offset];
            let shade: u8;
            if value > 0 {
                let age = frame - value;
                shade = 0xff - std::cmp::min(0xd0, age) as u8;
            } else {
                shade = 0x18;
            }
            let color = Color::RGB(shade, shade, shade);
            draw_cell(buffer, x, y, state.cell_size, color);
        }
    }
}

fn draw_pattern(buffer: &mut ImageBuffer, cell_x: u32, cell_y: u32, cell_size: u32, pattern: &Pattern, rotation: (i32, i32), color: Color) {
    let (rx, ry) = rotation;
    for (x, y) in pattern.points {
        let point_cell_x = rx * (x - pattern.origin_x) + cell_x as i32;
        let point_cell_y = ry * (y - pattern.origin_y) + cell_y as i32;
        if point_cell_x < 0 || point_cell_y < 0 {
            continue;
        }
        draw_cell(buffer, point_cell_x as u32, point_cell_y as u32, cell_size, color);
    }
}

fn draw_cell(buffer: &mut ImageBuffer, cell_x: u32, cell_y: u32, cell_size: u32, color: Color) {
    draw_point(buffer, cell_x * cell_size,     cell_y * cell_size,     color);
    draw_point(buffer, cell_x * cell_size + 1, cell_y * cell_size,     color);
    draw_point(buffer, cell_x * cell_size,     cell_y * cell_size + 1, color);
    draw_point(buffer, cell_x * cell_size + 1, cell_y * cell_size + 1, color);
}

fn draw_point(buffer: &mut ImageBuffer, x: u32, y: u32, color: Color) {
    if x as usize >= buffer.width || y as usize >= buffer.height {
        return;
    }
    let offset = y as usize * buffer.pitch + x as usize * 3;
    buffer.pixels[offset]     = color.r;
    buffer.pixels[offset + 1] = color.g;
    buffer.pixels[offset + 2] = color.b;
}

