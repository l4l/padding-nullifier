pub trait Padder {
    fn fill_padding(buf: &mut [u8]);
}

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
