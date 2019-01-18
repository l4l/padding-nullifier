pub use padding_derive::*;
pub use padding_traits::Padder;

#[cfg(test)]
mod tests {
    use super::Padder;
    use std::mem::size_of;

    #[repr(C)]
    #[derive(Padder)]
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
    #[derive(Padder)]
    struct NewType([u8; 7]);

    #[test]
    fn test_new_type() {
        let mut buf = [0xffu8; size_of::<NewType>()];
        NewType::fill_padding(&mut buf);
        assert_eq!(buf, [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
    }
}
