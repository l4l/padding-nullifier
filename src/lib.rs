pub use padding_derive::*;
pub use padding_traits::{Alignof, Padder};

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[repr(C)]
    #[derive(Padder, Alignof)]
    struct Flat {
        a: i8,
        b: u32,
        c: u16,
    }

    #[test]
    fn test_flat() {
        let mut buf = [0xffu8; size_of::<Flat>()];
        Flat::fill_padding(&mut buf);
        assert_eq!(
            buf,
            [0xff, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00]
        );
    }

    #[repr(C)]
    #[derive(Padder, Alignof)]
    struct NewType([u8; 7]);

    #[repr(C)]
    #[derive(Padder, Alignof)]
    struct NewType2([u16; 3]);

    #[test]
    fn test_new_type() {
        let mut buf = [0xffu8; size_of::<NewType>()];
        NewType::fill_padding(&mut buf);
        assert_eq!(buf, [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
        let mut buf = [0xffu8; size_of::<NewType2>()];
        NewType2::fill_padding(&mut buf);
        assert_eq!(buf, [0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
    }

    #[repr(C)]
    #[derive(Padder, Alignof)]
    struct NewtypeNested {
        nt1: NewType,
        i: i32,
        nt2: NewType,
        u: u32,
    }

    #[repr(C)]
    #[derive(Padder, Alignof)]
    struct NewtypeNested2 {
        nt1: NewType2,
        i: u16,
        nt2: NewType2,
        u: u32,
    }

    #[test]
    fn test_newtype_nested() {
        let mut buf = [0xffu8; size_of::<NewtypeNested>()];
        NewtypeNested::fill_padding(&mut buf);
        assert_eq!(
            buf,
            [
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0xff, 0xff, 0xff, 0xff
            ]
        );
        let mut buf = [0xffu8; size_of::<NewtypeNested2>()];
        NewtypeNested2::fill_padding(&mut buf);
        assert_eq!(
            buf,
            [
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0x00, 0x00, 0xff, 0xff, 0xff, 0xff
            ]
        );
    }
}
