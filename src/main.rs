use benchmarking::*;
fn main() {
    let mut rng = rand::thread_rng();
    let mut v = vec_with_n_times(10000, &mut rng);
    let mut h = hashmap_with_n_times(10000, &mut rng);
    let mut l = linked_list_with_n_times(10000, &mut rng);
    let mut b = btree_map_with_n_times(10000, &mut rng);

    insert_vec_with_n_items(1000, &v, &mut rng);
    insert_hashmap_with_n_items(1000, &h, &mut rng);
    insert_linked_list_with_n_items(1000, &l, &mut rng);
    insert_btree_map_with_n_items(1000, &b, &mut rng);

    _ = getting_random_item_from_vec(&v, &mut rng);
    _ = getting_random_item_from_hashmap(&h, &mut rng);
    _ = getting_random_item_from_btreemap(&b, &mut rng);
    _ = getting_random_item_from_linked_list(&l, &mut rng);
}
