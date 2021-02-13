use criterion::{ black_box, criterion_group, criterion_main, Criterion };
use gameoflife as game;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("game::step_generation", |b| b.iter(|| {
        let mut state = game::init((1280, 720), 2);
        game::step_generation(black_box(&mut state), 1)
    }));

    c.bench_function("game::draw_generation", |b| b.iter(|| {
        let mut pixels = vec![0; 1280 * 720 * 3];
        let mut buffer = game::ImageBuffer {
            width:  1280,
            height: 720,
            pitch:  1280 * 3,
            pixels: pixels.as_mut_slice()
        };
        let state = game::init((1280, 720), 2);
        game::draw_generation(&mut buffer, 1, &state);
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
