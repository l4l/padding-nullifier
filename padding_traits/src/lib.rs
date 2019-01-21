pub trait Padder {
    fn padding_buf() -> &'static [u8];
    fn fill_padding(buf: &mut [u8]) {
        buf.iter_mut().zip(Self::padding_buf()).for_each(|(b, pb)| {
            *b &= pb;
        })
    }
}

macro_rules! impl_padder_array {
    ($t:ty, $e:expr) => (
        impl Padder for [$t; $e] {
            fn padding_buf() -> &'static [u8] {
                static BUF: &'static [u8; std::mem::size_of::<[$t; $e]>()] =
                    &[0xff; std::mem::size_of::<[$t; $e]>()];
                &BUF[..]
            }
        }
    )
}

macro_rules! impl_padder {
    ([T; $e:expr]) => {
        impl_padder_array!(i8, $e);
        impl_padder_array!(u8, $e);
        impl_padder_array!(i16, $e);
        impl_padder_array!(u16, $e);
        impl_padder_array!(i32, $e);
        impl_padder_array!(u32, $e);
        impl_padder_array!(i64, $e);
        impl_padder_array!(u64, $e);
    };
    ($t:ty) => {
        impl Padder for $t {
            fn padding_buf() -> &'static [u8] {
                static BUF: &'static [u8; std::mem::size_of::<$t>()] =
                    &[0xff; std::mem::size_of::<$t>()];
                &BUF[..]
            }
        }
    };
}

impl_padder!(i8);
impl_padder!(u8);
impl_padder!(i16);
impl_padder!(u16);
impl_padder!(i32);
impl_padder!(u32);
impl_padder!(i64);
impl_padder!(u64);

impl_padder!([T; 1]);
impl_padder!([T; 2]);
impl_padder!([T; 3]);
impl_padder!([T; 4]);
impl_padder!([T; 5]);
impl_padder!([T; 6]);
impl_padder!([T; 7]);
impl_padder!([T; 8]);
impl_padder!([T; 9]);
impl_padder!([T; 10]);
impl_padder!([T; 11]);
impl_padder!([T; 12]);
impl_padder!([T; 13]);
impl_padder!([T; 14]);
impl_padder!([T; 15]);
impl_padder!([T; 16]);
impl_padder!([T; 17]);
impl_padder!([T; 18]);
impl_padder!([T; 19]);
impl_padder!([T; 20]);
impl_padder!([T; 21]);
impl_padder!([T; 22]);
impl_padder!([T; 23]);
impl_padder!([T; 24]);
impl_padder!([T; 25]);
impl_padder!([T; 26]);
impl_padder!([T; 27]);
impl_padder!([T; 28]);
impl_padder!([T; 29]);
impl_padder!([T; 30]);
impl_padder!([T; 31]);
impl_padder!([T; 32]);
impl_padder!([T; 64]);
impl_padder!([T; 128]);

pub trait Alignof {
    fn alignof() -> usize;
}

use std::mem::align_of;

macro_rules! impl_align {
    ([T; $e:expr]) => {
        impl<T> Alignof for [T; $e] {
            fn alignof() -> usize {
                align_of::<T>()
            }
        }
    };
    ($t:ty) => {
        impl Alignof for $t {
            fn alignof() -> usize {
                align_of::<$t>()
            }
        }
    };
}

impl_align!(i8);
impl_align!(u8);
impl_align!(i16);
impl_align!(u16);
impl_align!(i32);
impl_align!(u32);
impl_align!(i64);
impl_align!(u64);

impl_align!([T; 1]);
impl_align!([T; 2]);
impl_align!([T; 3]);
impl_align!([T; 4]);
impl_align!([T; 5]);
impl_align!([T; 6]);
impl_align!([T; 7]);
impl_align!([T; 8]);
impl_align!([T; 9]);
impl_align!([T; 10]);
impl_align!([T; 11]);
impl_align!([T; 12]);
impl_align!([T; 13]);
impl_align!([T; 14]);
impl_align!([T; 15]);
impl_align!([T; 16]);
impl_align!([T; 17]);
impl_align!([T; 18]);
impl_align!([T; 19]);
impl_align!([T; 20]);
impl_align!([T; 21]);
impl_align!([T; 22]);
impl_align!([T; 23]);
impl_align!([T; 24]);
impl_align!([T; 25]);
impl_align!([T; 26]);
impl_align!([T; 27]);
impl_align!([T; 28]);
impl_align!([T; 29]);
impl_align!([T; 30]);
impl_align!([T; 31]);
impl_align!([T; 32]);
impl_align!([T; 64]);
impl_align!([T; 128]);
impl_align!([T; 256]);
