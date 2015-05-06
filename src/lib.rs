use std::default::Default;

// allows recycling of items
pub trait Recycler : Default {
    type Item;
    fn recycle(&mut self, _item: Self::Item) { }
}

// a bunch of silly primitive type recyclers
impl Recycler for u8 { type Item = u8; }
impl Recycler for u16 { type Item = u16; }
impl Recycler for u32 { type Item = u32; }
impl Recycler for u64 { type Item = u64; }

impl Recycler for i8 { type Item = i8; }
impl Recycler for i16 { type Item = i16; }
impl Recycler for i32 { type Item = i32; }
impl Recycler for i64 { type Item = i64; }

impl Recycler for f32 { type Item = f32; }
impl Recycler for f64 { type Item = f64; }

impl Recycler for bool { type Item = bool; }
impl Recycler for () { type Item = (); }

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
    pub stash:    Vec<Vec<R::Item>>,
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
    pub fn new_from<F: FnMut(&mut Vec<R::Item>, &mut R)>(&mut self, mut func: F) -> Vec<R::Item> {
        let mut vec = self.stash.pop().unwrap_or(Vec::new());
        func(&mut vec, &mut self.recycler);
        vec
    }
}

impl<R: Recycler> Default for VecRecycler<R> {
    fn default() -> Self {
        VecRecycler {
            recycler: Default::default(),
            stash:    Vec::new(),
        }
    }
}
