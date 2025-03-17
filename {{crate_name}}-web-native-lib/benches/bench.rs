//! Comment out the "cdylib" crate type and any #[wasm_bindgen] usages
//! prior to running the benchmark.

#![feature(test)]
extern crate test;

#[cfg(test)]
mod benches {

    #[bench]
    fn universe_creation(b: &mut Bencher) {
        b.iter(|| black_box(1 + 1));
    }
}
