# recycler
A small Rust library for recycling types with owned memory

Recycler provides the `Recycler` trait and several implementations. Each `Recycler` object is capable of "recycling" items of its associated type `Item`, and using recycled items to "recreate" owned copies of referenced items.

```rust
pub trait Recycler : Default {
    type Item;
    fn recycle(&mut self, item: Self::Item);
    fn recreate(&mut self, other: &Self::Item) -> Self::Item;
}
```
The default `TrashRecycler` just drops arguments to `recycle` and clones arguments to `recreate`. However, smarter recyclers for types with owned memory can deconstruct the item and stash any of its owned memory, and then use the stashed memory to recreate items. For example, the implementation for `VecRecycler<R>` does just this:

 ```rust
 impl<R: Recycler> Recycler for VecRecycler<R> {
     type Item = Vec<R::Item>;
     // recycles vec contents and then stashes the vec
     fn recycle(&mut self, mut vec: Vec<R::Item>) {
         while let Some(x) = vec.pop() {
             self.recycler.recycle(x)
         }
         self.stash.push(vec);
     }
     // pops a stashed vector and then recreates each element
     fn recreate(&mut self, other: &Vec<R::Item>) -> Vec<R::Item> {
         let mut vec = self.stash.pop().unwrap_or(Vec::new());
         for elem in other.iter() {
             vec.push(self.recycler.recreate(elem));
         }
         vec
     }
 }
 ```

While recycling might sound great just because of civic duty, the real purpose is that these recyclers are able to return the owned memory to you, using a pattern not unlike standard allocation. Where you might write something like

```rust
#[bench]
fn allocate_vec_vec_str(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut v1 = Vec::with_capacity(10);
        for _ in 0..10 {
            let mut v2 = Vec::with_capacity(10);
            for _ in 0..10 {
                v2.push(("test!").to_owned());
            }
            v1.push(v2);
        }
        v1
    });
}
```

you can now instead write something pretty similar (no, not the same):

```rust
#[bench]
fn recycler_vec_vec_str(bencher: &mut Bencher) {
    let mut r1 = make_recycler::<Vec<Vec<String>>>();
    bencher.iter(|| {
        let v = { // scope the borrow of r1
            let (mut v1, r2) = r1.new();
            for _ in 0..10 {
                let (mut v2, r3) = r2.new();
                for _ in 0..10 {
                    v2.push(r3.new_from("test!"));
                }
                v1.push(v2);
            }
            v1
        };
        r1.recycle(v);
    });
}
```

The reason you do this is because if you run those benchmarks up there, you see numbers like:

    test allocate_vec_vec_str ... bench:       3,494 ns/iter (+/- 1,128)
    test recycler_vec_vec_str ... bench:       1,709 ns/iter (+/- 643)

If you do less formatting stuff and just put some `u64` data in the vectors, you see similar distinction:

    test allocate_vec_vec_u64 ... bench:         267 ns/iter (+/- 49)
    test recycler_vec_vec_u64 ... bench:         145 ns/iter (+/- 26)

note: a previous version of these numbers looked much better, because I used `Vec::new()` rather than `Vec::with_capacity(10)`.

The main down side is that you may get vectors that may have more memory than you need, and memory may also live for quite a while in the recycler. I almost added a `clear` method, but if you want to do that just make a new recycler and clobber the old one.

## recreate

If for some reason you find you are often given references to objects and need a quick clone (for example, using `decode` in [Abomonation](https://github.com/frankmcsherry/abomonation)), the `recreate` method is meant to be painless. The above benchmark becomes:

```rust
#[bench]
fn recreate_vec_vec_str(bencher: &mut Bencher) {
    let mut recycler = make_recycler::<Vec<Vec<String>>>();
    let data = vec![vec!["test!".to_owned(); 10]; 10];
    bencher.iter(|| {
        let record = recycler.recreate(&data);
        recycler.recycle(record);
    });
}
```

If you compare using `recreate` with just using `clone`, you see numbers like:

    test clone_vec_vec_str    ... bench:       2,906 ns/iter (+/- 774)
    test recreate_vec_vec_str ... bench:       1,773 ns/iter (+/- 625)

    test clone_vec_vec_u64    ... bench:         344 ns/iter (+/- 134)
    test recreate_vec_vec_u64 ... bench:         157 ns/iter (+/- 42)

## thanks!

If anyone has any hot tips or recommendations, especially about a macro or syntax extension that would let structs and such automatically derive recyclers, I'd be all ears. Any other friendly comments or contributions are also welcome.
