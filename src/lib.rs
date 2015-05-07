// #![feature(std_misc)]

use std::default::Default;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

// use std::collections::HashMap;
// use std::hash::Hash;

/// A value that has some default type that can recycle it.
pub trait Recyclable {
    type DefaultRecycler: Recycler<Item=Self>;
}

pub fn make_recycler<T: Recyclable>() -> T::DefaultRecycler {
    Default::default()
}

// These really want default associated types. (rust-lang/rust#19476)
impl Recyclable for u8 { type DefaultRecycler = TrashRecycler<Self>; }
impl Recyclable for i8 { type DefaultRecycler = TrashRecycler<Self>; }
impl Recyclable for u16 { type DefaultRecycler = TrashRecycler<Self>; }
impl Recyclable for i16 { type DefaultRecycler = TrashRecycler<Self>; }
impl Recyclable for u32 { type DefaultRecycler = TrashRecycler<Self>; }
impl Recyclable for i32 { type DefaultRecycler = TrashRecycler<Self>; }
impl Recyclable for u64 { type DefaultRecycler = TrashRecycler<Self>; }
impl Recyclable for i64 { type DefaultRecycler = TrashRecycler<Self>; }
impl Recyclable for usize { type DefaultRecycler = TrashRecycler<Self>; }
impl Recyclable for isize { type DefaultRecycler = TrashRecycler<Self>; }
impl Recyclable for () { type DefaultRecycler = TrashRecycler<Self>; }

impl Recyclable for String {
    type DefaultRecycler = StringRecycler;
}

impl<T: Recyclable> Recyclable for Vec<T> {
    type DefaultRecycler = VecRecycler<T::DefaultRecycler>;
}

impl<T: Recyclable> Recyclable for Option<T> {
    type DefaultRecycler = OptionRecycler<T::DefaultRecycler>;
}

impl<A: Recyclable, B: Recyclable> Recyclable for (A, B) {
    type DefaultRecycler = (A::DefaultRecycler, B::DefaultRecycler);
}


// allows recycling of items
pub trait Recycler : Default {
    type Item;
    fn recycle(&mut self, _item: Self::Item) { }
}

/// A "recycler" that doesn't recycle anything, instead just dropping anything
/// it is given. This is particularly useful for primitive types such as `i32`
/// that do not have `Drop` implementations.
pub struct TrashRecycler<Item> {
    marker: PhantomData<Item>
}

impl<Item> Default for TrashRecycler<Item> {
    fn default() -> Self {
        TrashRecycler {
            marker: PhantomData
        }
    }
}

impl<Item> Recycler for TrashRecycler<Item> {
    type Item = Item;
}

// demonstrating how tuples might be recycled
impl<R1: Recycler, R2: Recycler> Recycler for (R1, R2) {
    type Item = (R1::Item, R2::Item);
    fn recycle(&mut self, (part1, part2): (R1::Item, R2::Item)) {
        self.0.recycle(part1);
        self.1.recycle(part2);
    }
}

#[derive(Default)]
pub struct StringRecycler {
    stash: Vec<String>
}

impl Recycler for StringRecycler {
    type Item = String;
    fn recycle(&mut self, mut string: String) {
        string.clear();
        self.stash.push(string);
    }
}

impl StringRecycler {
    pub fn new(&mut self) -> String {
        self.stash.pop().unwrap_or(String::new())
    }
    pub fn new_from(&mut self, s: &str) -> String {
        let mut string = self.new();
        string.push_str(s);
        string
    }
}

// A recycler for vectors and their contents
pub struct VecRecycler<R: Recycler> {
    pub recycler: R,
    stash: Vec<Vec<R::Item>>,
}

// recycles vec contents, then stashes the vec
impl<R: Recycler> Recycler for VecRecycler<R> {
    type Item = Vec<R::Item>;
    fn recycle(&mut self, mut vec: Vec<R::Item>) {
        while let Some(x) = vec.pop() {
            self.recycler.recycle(x)
        }
        self.stash.push(vec);
    }
}

impl<R: Recycler> VecRecycler<R> {
    pub fn new(&mut self) -> (Vec<R::Item>, &mut R) {
        (self.stash.pop().unwrap_or(Vec::new()), &mut self.recycler)
    }
}

impl<R: Recycler> Default for VecRecycler<R> {
    fn default() -> Self {
        VecRecycler {
            recycler: Default::default(),
            stash: Vec::new(),
        }
    }
}

// option recycler
#[derive(Default)]
pub struct OptionRecycler<R: Recycler> {
    pub recycler: R,
}

impl<R: Recycler> Recycler for OptionRecycler<R> {
    type Item = Option<R::Item>;
    fn recycle(&mut self, option: Option<R::Item>) {
        if let Some(thing) = option {
            self.recycler.recycle(thing);
        }
    }
}

// derefs to contained recycler
impl<R: Recycler> Deref for OptionRecycler<R> {
    type Target = R;
    fn deref(&self) -> &Self::Target { &self.recycler }
}

// derefs to contained recycler, permits .new()
impl<R: Recycler> DerefMut for OptionRecycler<R> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.recycler }
}

// // commented out due to beta-instability of .drain()
// // recycles keys and values, then stashed the hashmap
// pub struct HashMapRecycler<KR: Recycler, VR: Recycler> {
//     pub key_recycler: KR,
//     pub val_recycler: VR,
//     pub stash: Vec<HashMap<KR::Item, VR::Item>>,
// }
//
// impl<KR: Recycler, VR: Recycler> Recycler for HashMapRecycler<KR, VR> where KR::Item: Eq+Hash {
//     type Item = HashMap<KR::Item, VR::Item>;
//     fn recycle(&mut self, mut map: HashMap<KR::Item, VR::Item>) {
//         for (key, val) in map.drain() {
//             self.key_recycler.recycle(key);
//             self.val_recycler.recycle(val);
//         }
//         self.stash.push(map);
//     }
// }
//
// impl<KR: Recycler, VR: Recycler> HashMapRecycler<KR, VR> where KR::Item: Eq+Hash {
//     pub fn new(&mut self) -> (HashMap<KR::Item, VR::Item>, (&mut KR, &mut VR)) {
//         (self.stash.pop().unwrap_or(HashMap::new()), (&mut self.key_recycler, &mut self.val_recycler))
//     }
// }
//
// impl<KR: Recycler, VR: Recycler> Default for HashMapRecycler<KR, VR> {
//     fn default() -> Self {
//         HashMapRecycler {
//             key_recycler: Default::default(),
//             val_recycler: Default::default(),
//             stash: Vec::new(),
//         }
//     }
// }
