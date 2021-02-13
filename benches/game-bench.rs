use criterion::{ Criterion, BatchSize, black_box, criterion_group, criterion_main };
use gameoflife as game;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const CELL_SIZE: u32 = 2;
const BYTES_PER_PIXEL: usize = 3;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("game::step_generation", |bencher| bencher.iter_batched_ref(
        || {
            game::init((WIDTH as u32, HEIGHT as u32), CELL_SIZE)
        },
        |state| {
            game::step_generation(black_box(state), 1)
        },
        BatchSize::PerIteration
    ));

    c.bench_function("game::draw_generation", |bencher| bencher.iter_batched_ref(
        || {
            let pixels = vec![0; WIDTH * HEIGHT * BYTES_PER_PIXEL];
            let state = game::init((WIDTH as u32, HEIGHT as u32), CELL_SIZE);
            (pixels, state)
        },
        |(pixels, state)| {
            let mut buffer = game::ImageBuffer {
                width:  WIDTH,
                height: HEIGHT,
                pitch:  WIDTH * BYTES_PER_PIXEL,
                pixels: pixels.as_mut_slice()
            };
            game::draw_generation(&mut buffer, 1, state)
        },
        BatchSize::PerIteration
    ));

    c.bench_function("game::update", |bencher| bencher.iter_batched_ref(
        || {
            let pixels = vec![0; WIDTH * HEIGHT * BYTES_PER_PIXEL];
            let state = game::init((WIDTH as u32, HEIGHT as u32), CELL_SIZE);
            (pixels, state)
        },
        |(pixels, state)| {
            let mut buffer = game::ImageBuffer {
                width:  WIDTH,
                height: HEIGHT,
                pitch:  WIDTH * BYTES_PER_PIXEL,
                pixels: pixels.as_mut_slice()
            };
            let events = NoEventsIterator {};
            game::update(1, &mut buffer, state, events)
        },
        BatchSize::PerIteration
    ));
}

struct NoEventsIterator {
}

impl Iterator for NoEventsIterator {
    type Item = sdl2::event::Event;
    fn next(&mut self) -> Option<sdl2::event::Event> { None }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
