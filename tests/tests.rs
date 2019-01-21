use padding_nullifier::{Alignof, Padder};

#[repr(C)]
#[derive(Alignof, Padder, Clone)]
struct A {
    i: i8,
    u: u32,
}

#[test]
fn test_sample_usage() {
    let structs = vec![
        A {
            i: 0x7f,
            u: 0x12345678,
        };
        4
    ];
    let mut network_buf = [0u8; 32];

    // Copy array to buffer
    unsafe {
        std::ptr::copy(
            structs.as_ptr(),
            network_buf.as_mut_ptr() as *mut A,
            structs.len(),
        )
    };

    // fill paddings
    for i in 0..structs.len() {
        let offset = i * std::mem::size_of::<A>();
        let mut sub_buf = &mut network_buf[dbg!(offset)..];
        A::fill_padding(&mut sub_buf);
    }

    #[cfg(target_endian = "little")]
    fn get_buf() -> [u8; 32] {
        [
            0x7f, 0x00, 0x00, 0x00, 0x78, 0x56, 0x34, 0x12, // first
            0x7f, 0x00, 0x00, 0x00, 0x78, 0x56, 0x34, 0x12, // second
            0x7f, 0x00, 0x00, 0x00, 0x78, 0x56, 0x34, 0x12, // third
            0x7f, 0x00, 0x00, 0x00, 0x78, 0x56, 0x34, 0x12, // forth
        ]
    }

    #[cfg(target_endian = "big")]
    fn get_buf() -> [u8; 32] {
        [
            0x7f, 0x00, 0x00, 0x00, 0x12, 0x34, 0x56, 0x78, // first
            0x7f, 0x00, 0x00, 0x00, 0x12, 0x34, 0x56, 0x78, // second
            0x7f, 0x00, 0x00, 0x00, 0x12, 0x34, 0x56, 0x78, // third
            0x7f, 0x00, 0x00, 0x00, 0x12, 0x34, 0x56, 0x78, // forth
        ]
    }

    assert_eq!(network_buf, get_buf());
}
