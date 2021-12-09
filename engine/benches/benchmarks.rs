use criterion::{black_box, criterion_group, criterion_main, Criterion};
use engine::*;

fn bench_world_gen(c: &mut Criterion) {
    c.bench_function("World Generation", |b| b.iter( || {
        black_box(World::new(1, 1, [0; 3]));
    }));
}
fn bench_chunk_gen(c: &mut Criterion) {
    c.bench_function("Chunk Generation", |b| b.iter(|| {
        let mut chunk = Chunk::new([0;3]);
        black_box(chunk.construct());
    }));
}
fn bench_block_gen(c: &mut Criterion) {
    c.bench_function("Block Generation", |b| b.iter(|| {
        let normals = Shape::construct_normals([0; 3]);
        black_box(Shape::cube([0; 3], &normals, vec![Faces::Top, Faces::Front, Faces::Left, Faces::Back, Faces::Bottom, Faces::Right]));
    }));
}
fn bench_world_updates(c: &mut Criterion) {
    let mut world = World::new(1, 1, [0; 3]);

    c.bench_function("World Updates", |b| b.iter(|| {
        black_box(world.update([1000.0, 500.0, 2500.0]));
    }));
}
criterion_group!(benches, bench_world_gen, bench_chunk_gen, bench_block_gen, bench_world_updates);
criterion_main!(benches);

