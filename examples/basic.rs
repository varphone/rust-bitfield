#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use rust_bitfield::{bitfield_fields, BitRange, Bits};

// Define a BitField Type with 2 fields.
#[derive(Copy, Clone, Debug, Default)]
struct tV56aIEy_Bi1(u16);

impl tV56aIEy_Bi1 {
    bitfield_fields! {
        IEDZ, set_IEDZ : u16 [9..0];
        _rb_, set__rb_ : u16 [15..10];
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
struct tV56aIEy {
    IES0: u8,
    IES1: u8,
    IES2: u8,
    IES3: u8,
    bi1: tV56aIEy_Bi1,
}

fn main() {
    let mut a = tV56aIEy::default();
    assert_eq!(6, std::mem::size_of::<tV56aIEy>());
    println!("{:?}", a);
    println!("{:?}", a);
    println!("IEDZ = {}, _rb_ = {}", a.bi1.IEDZ(), a.bi1._rb_());
    println!("{:?}", a);
    println!(
        "IEDZ = {}, _rb_ = {}",
        a.bi1.set_IEDZ(512).IEDZ(),
        a.bi1._rb_()
    );
    println!("{:?}", a);
    println!("IEDZ = {}, _rb_ = {}", a.bi1.IEDZ(), a.bi1._rb_());
    println!("{:?}", a);
    println!("IEDZ = {}, _rb_ = {}", a.bi1.IEDZ(), a.bi1._rb_());
    println!("{:?}", a);
}