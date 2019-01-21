use padding_nullifier::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use std::mem::size_of;

fn padding_simple(c: &mut Criterion) {
    c.bench_function("padding_simple", {
        |b| {
            #[repr(C)]
            #[derive(Padder, Alignof)]
            struct S {
                a: i8,
                b: u32,
                c: u16,
            }
            let mut buf = [0xffu8; size_of::<S>()];
            b.iter(|| {
                S::fill_padding(&mut buf);
                black_box(&buf);
            })
        }
    });
}

fn padding_nop(c: &mut Criterion) {
    c.bench_function("padding_nop", {
        |b| {
            #[repr(C)]
            #[derive(Padder, Alignof)]
            struct S {
                a: i8,
                b: i8,
                c: i8,
            }
            let mut buf = [0xffu8; size_of::<S>()];
            b.iter(|| {
                S::fill_padding(&mut buf);
                black_box(&buf);
            })
        }
    });
}

criterion_group!(benches, padding_simple, padding_nop);
criterion_main!(benches);
