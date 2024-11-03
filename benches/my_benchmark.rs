use benchmarking::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::{BTreeMap, HashMap, LinkedList};

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    let mut testing_datastructure = c.benchmark_group("testing datastructures");

    let mut v: Vec<isize> = Vec::new();
    let mut hm: HashMap<isize, isize> = HashMap::new();
    let mut bm: BTreeMap<isize, isize> = BTreeMap::new();
    let mut ll: LinkedList<isize> = LinkedList::new();

    let size = 10usize;
    testing_datastructure.bench_with_input(format!("Vec/{}", size), &size, |b, &size| {
        b.iter(|| v = vec_with_n_times(black_box(size), &mut rng))
    });
    testing_datastructure.bench_with_input(format!("Hashmap/{}", size), &size, |b, &size| {
        b.iter(|| hm = hashmap_with_n_times(black_box(size), &mut rng))
    });
    testing_datastructure.bench_with_input(format!("BTree/{}", size), &size, |b, &size| {
        b.iter(|| bm = btree_map_with_n_times(black_box(size), &mut rng))
    });
    testing_datastructure.bench_with_input(format!("LinkedList/{}", size), &size, |b, &size| {
        b.iter(|| ll = linked_list_with_n_times(black_box(size), &mut rng))
    });

    testing_datastructure.bench_with_input(
        format!("Insert into Vec/{}", size),
        &size,
        |b, &size| b.iter(|| v = insert_vec_with_n_items(black_box(size), &v, &mut rng)),
    );
    testing_datastructure.bench_with_input(
        format!("Insert into Hashmap/{}", size),
        &size,
        |b, &size| b.iter(|| hm = insert_hashmap_with_n_items(black_box(size), &hm, &mut rng)),
    );
    testing_datastructure.bench_with_input(
        format!("Insert into BTree/{}", size),
        &size,
        |b, &size| b.iter(|| bm = insert_btree_map_with_n_items(black_box(size), &bm, &mut rng)),
    );
    testing_datastructure.bench_with_input(
        format!("Insert into LinkedList/{}", size),
        &size,
        |b, &size| b.iter(|| ll = insert_linked_list_with_n_items(black_box(size), &ll, &mut rng)),
    );

    testing_datastructure.bench_with_input(
        format!("Looking For Item in  Vec/{}", size),
        &size,
        |b, &size| b.iter(|| _ = getting_random_item_from_vec(&v, &mut rng)),
    );

    testing_datastructure.bench_with_input(
        format!("Looking For Item in  Hashmap/{}", size),
        &size,
        |b, &size| b.iter(|| _ = getting_random_item_from_hashmap(&hm, &mut rng)),
    );

    testing_datastructure.bench_with_input(
        format!("Looking For Item in  BTree/{}", size),
        &size,
        |b, &size| b.iter(|| _ = getting_random_item_from_btreemap(&bm, &mut rng)),
    );

    testing_datastructure.bench_with_input(
        format!("Looking For Item in  LinkedList/{}", size),
        &size,
        |b, &size| b.iter(|| _ = getting_random_item_from_linked_list(&ll, &mut rng)),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
