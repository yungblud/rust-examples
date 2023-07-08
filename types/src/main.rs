#![allow(overflowing_literals)]

type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    let decimal = 65.4321_f32;

    // let integer: u8 = decimal;

    let integer = decimal as u8;
    let character = integer as char;

    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as u16 is: {}", 1000 as u16);

    println!("1000 as u16 is: {}", 1000 as u8);

    println!(" -1 as u8 is : {}", 1000 as u8);

    println!("1000 mod 256 is : {}", 1000 % 256);

    println!(" 128 as i16 is: {}", 128 as i16);

    // 128 as u8 -> 128, whose value in 8-bit two's complement representation is:
    println!(" 128 as a i8 is : {}", 128 as i8);

    // repeating the example above
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // and the value of 232 in 8-bit two's complement representation is -24
    println!(" 232 as a i8 is : {}", 232 as i8);

    // Since Rust 1.45, the `as` keyword performs a *saturating cast*
    // when casting from float to int. If the floating point value exceeds
    // the upper bound or is less than the lower bound, the returned value
    // will be equal to the bound crossed.

    // 300.0 as u8 is 255
    println!(" 300.0 as u8 is : {}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is : {}", -100.0_f32 as u8);
    // nan as u8 is 0
    println!("   nan as u8 is : {}", f32::NAN as u8);

    unsafe {
        println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
        println!("-100.0 as u8 is : {}", (-100.0_f32).to_int_unchecked::<u8>());
        println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
    }

    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    let i = 1;
    let f = 1.0;

    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

    let elem = 5u8;
    let mut vec = Vec::new();

    vec.push(elem);

    println!("{:?}", vec);

    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;

    println!("{} nanoseconds + {} inches = {} unit?", nanoseconds, inches, nanoseconds + inches);
}
