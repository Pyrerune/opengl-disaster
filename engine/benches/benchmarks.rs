use criterion::{black_box, criterion_group, criterion_main, Criterion};
use engine::*;

fn bench_world_gen(c: &mut Criterion) {
    c.bench_function("World Generation", |b| b.iter( || {
        black_box(World::new(10, 10, [0.0; 3]));
    }));
}
fn bench_chunk_gen(c: &mut Criterion) {
    c.bench_function("Chunk Generation", |b| b.iter(|| {
        let mut chunk = Chunk::new([0.0;3]);
        black_box(chunk.construct());
    }));
}
fn bench_block_gen(c: &mut Criterion) {
    c.bench_function("Block Generation", |b| b.iter(|| {
        let vertices = all_faces([0.0; 3]);
        black_box(Block {
            center: [0.0; 3],
            faces: vertices,
        });
    }));
}

criterion_group!(benches, bench_chunk_gen, bench_block_gen);
criterion_main!(benches);

