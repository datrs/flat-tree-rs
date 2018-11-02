#[macro_use]
extern crate criterion;
extern crate flat_tree;

use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("children_9", |b| b.iter(|| flat_tree::children(9)));
  c.bench_function("children_256", |b| b.iter(|| flat_tree::children(256)));
  c.bench_function("count_128", |b| b.iter(|| flat_tree::count(128)));
  c.bench_function("depth_128", |b| b.iter(|| flat_tree::depth(128)));
  // c.bench_function("full_roots_16", |b| {
  //   let mut nodes: Vec<usize> = Vec::with_capacity(16);
  //   b.iter(|| {
  //     flat_tree::full_roots(16, &mut nodes);
  //   })
  // });
  // c.bench_function("full_roots_256", |b| {
  //   let mut nodes: Vec<usize> = Vec::with_capacity(16);
  //   b.iter(|| {
  //     flat_tree::full_roots(256, &mut nodes);
  //   })
  // });
  c.bench_function("index_3_1", |b| b.iter(|| flat_tree::index(3, 1)));
  c.bench_function("left_child_128", |b| b.iter(|| flat_tree::left_child(128)));
  c.bench_function("left_span_128", |b| b.iter(|| flat_tree::left_span(128)));
  c.bench_function("offset_128", |b| b.iter(|| flat_tree::offset(128)));
  c.bench_function("parent_128", |b| b.iter(|| flat_tree::parent(128)));
  c.bench_function("right_child_128", |b| {
    b.iter(|| flat_tree::right_child(128))
  });
  c.bench_function("right_span_128", |b| b.iter(|| flat_tree::right_span(128)));
  c.bench_function("sibling_128", |b| b.iter(|| flat_tree::sibling(128)));
  c.bench_function("spans_128", |b| b.iter(|| flat_tree::spans(128)));
  c.bench_function("uncle_128", |b| b.iter(|| flat_tree::uncle(128)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
