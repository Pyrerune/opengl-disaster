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
fn bench_chunk_vertices(c: &mut Criterion) {
    c.bench_function("Chunk Vertex Collection", |b| b.iter(|| {

        let mut chunk = Chunk::new([0.0; 3]);
        chunk.construct();
        black_box(chunk.vertices());
    }));
}

fn bench_block_into(c: &mut Criterion) {
    let vertices = all_faces([0.0; 3]);
    let block = Block {
            center: [0.0; 3],
            faces: vertices,
    };
    c.bench_function("Block Vertex Collection", |b| b.iter(|| {
        black_box({
            let block_vertices: Vec<Vertex> = block.into();
            block_vertices
        });
    }));
}
fn bench_world_vertices(c: &mut Criterion) {
//DON'T RUN THIS IT TAKES  SO LONG
    let world = World::new(10, 10, [0.0; 3]);
    
    c.bench_function("World Vertex Collection", |b| b.iter(|| {
        //let worldclone = world.clone();
        black_box({
            for chunk in &world.chunks {
                chunk.clone().vertices();
            }
        });
    }));
}
criterion_group!(benches, bench_world_gen, bench_world_vertices, bench_chunk_gen, bench_chunk_vertices, bench_block_gen, bench_block_into);
criterion_main!(benches);

