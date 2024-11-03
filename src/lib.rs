use rand::prelude::ThreadRng;
use rand::Rng;
use std::collections::{BTreeMap, HashMap, LinkedList};

#[inline]
pub fn vec_with_n_times(n: usize, r: &mut ThreadRng) -> Vec<isize> {
    let mut v: Vec<isize> = Vec::new();
    while v.len() < n {
        v.push(r.gen::<isize>());
    }
    v
}

#[inline]
pub fn hashmap_with_n_times(n: usize, r: &mut ThreadRng) -> HashMap<isize, isize> {
    let mut hm: HashMap<isize, isize> = HashMap::new();
    while hm.len() < n {
        let i = r.gen::<isize>();
        hm.insert(i, i);
    }
    hm
}

#[inline]
pub fn linked_list_with_n_times(n: usize, r: &mut ThreadRng) -> LinkedList<isize> {
    let mut ll: LinkedList<isize> = LinkedList::new();
    while ll.len() < n {
        ll.push_back(r.gen::<isize>());
    }
    ll
}

#[inline]
pub fn btree_map_with_n_times(n: usize, r: &mut ThreadRng) -> BTreeMap<isize, isize> {
    let mut btm: BTreeMap<isize, isize> = BTreeMap::new();
    while btm.len() < n {
        let i = r.gen::<isize>();
        btm.insert(i, i);
    }
    btm
}

#[inline]
pub fn insert_vec_with_n_items(n: usize, v: &Vec<isize>, r: &mut ThreadRng) -> Vec<isize> {
    let mut vr = v.clone();
    let estimated_length = v.len() + n;
    while vr.len() < estimated_length {
        vr.push(r.gen::<isize>());
    }
    vr
}

pub fn insert_hashmap_with_n_items(
    n: usize,
    hm: &HashMap<isize, isize>,
    r: &mut ThreadRng,
) -> HashMap<isize, isize> {
    let mut hmr: HashMap<isize, isize> = hm.clone();
    let estimated_length = hm.len() + n;
    while hmr.len() < estimated_length {
        let i = r.gen::<isize>();
        hmr.insert(i, i);
    }

    hmr
}

pub fn insert_linked_list_with_n_items(
    n: usize,
    ll: &LinkedList<isize>,
    r: &mut ThreadRng,
) -> LinkedList<isize> {
    let mut l = ll.clone();
    let estimated_length = ll.len() + n;
    while l.len() < estimated_length {
        l.push_back(r.gen::<isize>());
    }
    l
}

pub fn insert_btree_map_with_n_items(
    n: usize,
    bm: &BTreeMap<isize, isize>,
    r: &mut ThreadRng,
) -> BTreeMap<isize, isize> {
    let mut btm: BTreeMap<isize, isize> = bm.clone();
    let estimated_length = bm.len() + n;
    while btm.len() < estimated_length {
        let i = r.gen::<isize>();
        btm.insert(i, i);
    }
    btm
}

pub fn getting_random_item_from_vec(v: &Vec<isize>, r: &mut ThreadRng) -> isize {
    let item = r.gen::<isize>();
    v.into_iter().find(|x| *x == &item).cloned().unwrap_or(item)
}

pub fn getting_random_item_from_hashmap(hm: &HashMap<isize, isize>, r: &mut ThreadRng) -> isize {
    let item = r.gen::<isize>();
    hm.get(&item).cloned().unwrap_or(item)
}

pub fn getting_random_item_from_linked_list(l: &LinkedList<isize>, r: &mut ThreadRng) -> isize {
    let item = r.gen::<isize>();
    l.iter().rfind(|x| *x == &item).cloned().unwrap_or(item)
}

pub fn getting_random_item_from_btreemap(bm: &BTreeMap<isize, isize>, r: &mut ThreadRng) -> isize {
    let item = r.gen::<isize>();
    bm.get(&item).cloned().unwrap_or(item)
}
