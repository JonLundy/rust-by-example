#![feature(macro_rules)]

use std::simd::f32x4;

macro_rules! assert_len {
    () => {
        assert!(xs.len() == ys.len(),
                "add_assign: dimension mismatch: {} += {}",
                (xs.len(),),
                (ys.len(),));
    }
}

// element-wise addition
fn add_assign(xs: &mut Vec<f32>, ys: &Vec<f32>) {
    assert_len!();

    for (x, y) in xs.mut_iter().zip(ys.iter()) {
        *x += *y;
    }
}

// simd accelerated addition
fn simd_add_assign(xs: &mut Vec<f32>, ys: &Vec<f32>) {
    assert_len!();

    let size = xs.len() as int;
    let chunks = size / 4;

    // pointer to the start of the vector data
    let p_x: *mut f32 = xs.as_mut_ptr();
    let p_y: *const f32 = ys.as_ptr();

    // sum excess elements that don't fit in the simd vector
    for i in range(4 * chunks, size) {
        // dereferencing a raw pointer requires an unsafe block
        unsafe {
            // offset by i elements
            *p_x.offset(i) += *p_y.offset(i);
        }
    }

    // treat f32 vector as an simd f32x4 vector
    let simd_p_x = p_x as *mut f32x4;
    let simd_p_y = p_y as *const f32x4;

    // sum "simd vector"
    for i in range(0, chunks) {
        unsafe {
            *simd_p_x.offset(i) += *simd_p_y.offset(i);
        }
    }
}

mod bench {
    extern crate test;
    use self::test::Bencher;
    static BENCH_SIZE: uint = 10_000;

    macro_rules! bench {
        ($name:ident, $func:ident) => {
            #[bench]
            fn $name(b: &mut Bencher) {
                let mut x = Vec::from_elem(BENCH_SIZE, 1.0f32);
                let y = Vec::from_elem(BENCH_SIZE, 0.1f32);

                b.iter(|| {
                    super::$func(&mut x, &y);
                })
            }
        }
    }

    bench!(vanilla, add_assign)
    bench!(simd, simd_add_assign)
}
