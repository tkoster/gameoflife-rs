use criterion::{ black_box, criterion_group, criterion_main, Criterion };
use gameoflife as game;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const CELL_SIZE: u32 = 2;
const BYTES_PER_PIXEL: usize = 3;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("game::step_generation", |b| b.iter(|| {
        let mut state = game::init((WIDTH as u32, HEIGHT as u32), CELL_SIZE);
        game::step_generation(black_box(&mut state), 1)
    }));

    c.bench_function("game::draw_generation", |b| b.iter(|| {
        let mut pixels = vec![0; WIDTH * HEIGHT * BYTES_PER_PIXEL];
        let mut buffer = game::ImageBuffer {
            width:  WIDTH,
            height: HEIGHT,
            pitch:  WIDTH * BYTES_PER_PIXEL,
            pixels: pixels.as_mut_slice()
        };
        let state = game::init((WIDTH as u32, HEIGHT as u32), CELL_SIZE);
        game::draw_generation(&mut buffer, 1, &state);
    }));

    c.bench_function("game::update", |b| b.iter(|| {
        let mut pixels = vec![0; WIDTH * HEIGHT * BYTES_PER_PIXEL];
        let mut buffer = game::ImageBuffer {
            width:  WIDTH,
            height: HEIGHT,
            pitch:  WIDTH * BYTES_PER_PIXEL,
            pixels: pixels.as_mut_slice()
        };
        let mut state = game::init((WIDTH as u32, HEIGHT as u32), CELL_SIZE);
        let events = NoEventsIterator {};
        game::update(1, &mut buffer, &mut state, events);
    }));
}

struct NoEventsIterator {
}

impl Iterator for NoEventsIterator {
    type Item = sdl2::event::Event;
    fn next(&mut self) -> Option<sdl2::event::Event> { None }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
