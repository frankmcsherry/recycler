# recycler
A small Rust library for recycling types with owned memory

Recycler provides the `Recycler` trait and several implementations. Each `Recycler` object is capable of "recycling" items of its associated type `Item`.

```rust
// allows recycling of items
pub trait Recycler : Default {
    type Item;
    fn recycle(&mut self, _item: Self::Item) { }
}
```

 The intended behavior is that types with owned memory will be deconstructed and have the owned memory enqueued. For example, the implementation for `VecRecycler<R>` is just

 ```rust
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
 ```

While recycling might sound great just because of civic duty, the real purpose is that these recyclers are able to return the owned memory to you, using a pattern not unlike standard allocation. Where you might write something like

```rust
#[bench]
fn allocate_vec_vec_str(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut v1 = Vec::new();
        for _ in 0..10 {
            let mut v2 = Vec::new();
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
    let mut r1: VecRecycler<VecRecycler<StringRecycler>> = Default::default();
    bencher.iter(|| {
        let v = {
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

There is a bit of nonsense because of lexical borrows, and because type inference doesn't seem to do a great job finding an implementation with an associated type as a constraint. Perhaps I'll learn that this was a bad pattern.

The reason you do this is because if you run those benchmarks up there, you see numbers like:

    test allocate_vec_vec_str   ... bench:      4556 ns/iter (+/- 2166)
    test recycler_vec_vec_str   ... bench:      2033 ns/iter (+/- 945)

If you do less formatting stuff and just put some `u64` data in the vectors, you see an even bigger distinction:

    test allocate_vec_vec_u64   ... bench:      1281 ns/iter (+/- 303)
    test recycler_vec_vec_u64   ... bench:       154 ns/iter (+/- 27)

The main down side I can think of is that you are getting vectors that may have more memory than you need. They may also live for a while in the recycler. I almost added a `clear` method, but if you want to do that just make a new recycler and clobber the old one.

If anyone has any hot tips or recommendations, especially about a macro or syntax extension that would let structs and such automatically derive recyclers, I'd be all ears. Or any other friendly comments or contributions.
