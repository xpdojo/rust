use std::mem;

// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
#[test]
fn test_size() {
    let a: u8 = 123; // u = unsigned, 8 bits (0..255) = 0 to 2^N-1
    let b: i8 = -123; // i = signed, 8 bits (-128..127) = -2^(N-1) to 2^(N-1)-1
    // b = 42; // error: cannot assign twice to immutable variable
    assert_eq!(mem::size_of_val(&a), 1);
    assert_eq!(mem::size_of_val(&b), 1);
}

#[test]
fn test_mutable() {
    // mut = mutable
    let mut c: i8 = 0;
    assert_eq!(c, 0);

    c = 42;
    assert_eq!(c, 42);
}

#[test]
#[should_panic]
fn test_overflow() {
    let mut d: u8 = 255;
    assert_eq!(mem::size_of_val(&d), 1);

    d = d + 1;
    assert_eq!(d, 0);
}

#[test]
fn test_i32() {
    let d = 123456789; // 타입을 지정하지 않으면 자동 i32 = 32 bits = 4 bytes
    let size = mem::size_of_val(&d);
    assert_eq!(size, 4); // i32 takes up 4 bytes
}

#[test]
fn test_isize_default_arch() {
    // u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
    // isize, usize
    let z: isize = 123;
    let size = mem::size_of_val(&z);
    assert_eq!(size, 8); // isize takes up 8 bytes, 64-bit OS
}

#[test]
fn test_f32() {
    let f: f32 =2.5;
    let size = mem::size_of_val(&f);
    assert_eq!(size, 4); // f32 takes up 4 bytes
}

#[test]
fn test_char() {
    let e: char = 'x';
    let size = mem::size_of_val(&e);
    assert_eq!(size, 4); // char takes up 4 bytes
}

#[test]
fn test_bool() {
    let b: bool = true;
    let size = mem::size_of_val(&b);
    assert_eq!(size, 1); // bool takes up 1 byte
}

#[test]
fn test_array() {
    let a = [1, 2, 3]; // [T; N] = T = type, N = number
    let size = mem::size_of_val(&a);
    assert_eq!(size, 4 * 3); // [i32; 3] takes up 12 bytes
}
