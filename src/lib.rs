#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

/// A trait to get or set a single bit.
///
/// This trait is implemented for all type that implement `BitRange<T>`.
pub trait Bit {
    /// Get a single bit.
    fn bit(&self, bit: usize) -> bool;

    /// Set a single bit.
    fn set_bit(&mut self, bit: usize, value: bool);
}

/// A trait to get or set ranges of bits.
pub trait BitRange<T> {
    /// Get a range of bits.
    fn bit_range(&self, msb: usize, lsb: usize) -> T;
    /// Set a range of bits.
    fn set_bit_range(&mut self, msb: usize, lsb: usize, value: T);
}

/// A struct to support bits operations.
pub struct Bits<T>(pub T);

#[macro_export(local_inner_macros)]
macro_rules! impl_bits {
    () => {};
    (@inner $T:tt => bool) => {
        impl From<bool> for Bits<$T> {
            fn from(value: bool) -> Self {
                match value {
                    true => Self(1 as $T),
                    false => Self(0 as $T),
                }
            }
        }
        impl Into<bool> for Bits<$T> {
            fn into(self) -> bool {
                match self.0 {
                    0 => false,
                    _ => true,
                }
            }
        }
        impl Bit for Bits<$T> {
            fn bit(&self, bit: usize) -> bool {
                (self.0 & (1 << bit)) != 0
            }

            fn set_bit(&mut self, bit: usize, value: bool) {
                match value {
                    true => self.0 |= 1 << bit,
                    false => self.0 &= !(1 << bit),
                }
            }
        }
    };
    (@inner $T:tt => $U:tt) => {
        impl From<$U> for Bits<$T> {
            fn from(value: $U) -> Self {
                Self(value as $T)
            }
        }
        impl Into<$U> for Bits<$T> {
            fn into(self) -> $U {
                self.0 as $U
            }
        }
        impl BitRange<$U> for Bits<$T> {
            #[inline]
            #[allow(unknown_lints)]
            #[allow(cast_lossless)]
            fn bit_range(&self, msb: usize, lsb: usize) -> $U {
                let mask = ((1 << (msb - lsb + 1)) - 1) << lsb;
                ((self.0 & mask) >> lsb) as $U
            }

            #[inline]
            #[allow(unknown_lints)]
            #[allow(cast_lossless)]
            fn set_bit_range(&mut self, msb: usize, lsb: usize, value: $U) {
                let mask = ((1 << (msb - lsb + 1)) - 1) << lsb;
                self.0 = (self.0 & !mask) | ((value as $T) << lsb);
            }
        }
    };
    ($T:tt => [$($U:tt),*]; $($rest:tt)*) => {
        $(
            impl_bits!{ @inner $T => $U }
        )*
        impl_bits!{ $($rest)* }
    }
}

#[macro_export(local_inner_macros)]
macro_rules! impl_bitrange {
    () => {};
    (@inner $T:ty => $U:ty) => {
        impl From<$U> for Bits<$T> {
            fn from(value: $U) -> Self {
                Self(value as $T)
            }
        }
        impl Into<$U> for Bits<$T> {
            fn into(self) -> $U {
                self.0 as $U
            }
        }
        impl BitRange<$U> for Bits<$T> {
            #[inline]
            #[allow(unknown_lints)]
            #[allow(cast_lossless)]
            fn bit_range(&self, msb: usize, lsb: usize) -> $U {
                let mask = ((1 << (msb - lsb + 1)) - 1) << lsb;
                ((self.0 & mask) >> lsb) as $U
            }

            #[inline]
            #[allow(unknown_lints)]
            #[allow(cast_lossless)]
            fn set_bit_range(&mut self, msb: usize, lsb: usize, value: $U) {
                let mask = ((1 << (msb - lsb + 1)) - 1) << lsb;
                self.0 = (self.0 & !mask) | ((value as $T) << lsb);
            }
        }
    };
    ($T:ty => [$($U:ty),*]; $($rest:tt)*) => {
        $(
            impl_bitrange!{ @inner $T => $U }
        )*
        impl_bitrange!{ $($rest)* }
    }
}

#[macro_export(local_inner_macros)]
macro_rules! bitfield_fields {
    // Empty.
    () => {};
    // Dummy.
    (@field ($(#[$attribute:meta])*) ($($vis:tt)*) _, _ : $sty:tt [] in $slot:tt) => {
    };
    (@field ($(#[$attribute:meta])*) ($($vis:tt)*) _, _ : $sty:tt [$msb:tt..$lsb:tt] in $slot:tt as $vty:tt) => {
    };
    // Return all bits.
    (@field ($(#[$attribute:meta])*) ($($vis:tt)*) $getter:tt, _ : $sty:tt [] in $slot:tt) => {
        $(#[$attribute])*
        $($vis)* fn $getter(&self) -> $sty {
            self.$slot
        }
    };
    (@getter ($(#[$attribute:meta])*) ($($vis:tt)*) $getter:tt : $sty:tt [] in $slot:tt) => {
        $(#[$attribute])*
        $($vis)* fn $getter(&self) -> $sty {
            self.$slot
        }
    };
    // Return msb bit as bool.
    (@field ($(#[$attribute:meta])*) ($($vis:tt)*) $getter:tt, _ : $sty:tt [$msb:tt..$lsb:tt] in $slot:tt as bool) => {
        $(#[$attribute])*
        $($vis)* fn $getter(&self) -> bool {
            Bits::<$sty>(self.$slot).bit($msb)
        }
    };
    // Return bit range of [msb..lsb] as U.
    (@field ($(#[$attribute:meta])*) ($($vis:tt)*) $getter:tt, _ : $sty:tt [$msb:tt..$lsb:tt] in $slot:tt as $vty:tt) => {
        $(#[$attribute])*
        $($vis)* fn $getter(&self) -> $vty {
            Bits::<$sty>(self.$slot).bit_range($msb, $lsb)
        }
    };
    // Set all bits with T.
    (@field ($(#[$attribute:meta])*) ($($vis:tt)*) _, $setter:tt : $sty:tt [] in $slot:tt) => {
        $(#[$attribute])*
        $($vis)* fn $setter(&mut self, value: $sty) -> &mut Self {
            self.$slot = value;
            self
        }
    };
    (@setter ($(#[$attribute:meta])*) ($($vis:tt)*) $setter:tt : $sty:tt [] in $slot:tt) => {
        $(#[$attribute])*
        $($vis)* fn $setter(&mut self, value: $sty) -> &mut Self {
            self.$slot = value;
            self
        }
    };
    // Set msb bit with bool.
    (@field ($(#[$attribute:meta])*) ($($vis:tt)*) _, $setter:tt : $sty:tt [$msb:tt..$lsb:tt] in $slot:tt as bool) => {
        $(#[$attribute])*
        $($vis)* fn $setter(&mut self, value: bool) -> &mut Self {
            let mut bits = Bits::<$sty>(self.$slot);
            bits.set_bit($msb, value);
            self.$slot = bits.into();
            self
        }
    };
    // Set bit range of [msb..lsb] with U.
    (@field ($(#[$attribute:meta])*) ($($vis:tt)*) _, $setter:tt : $sty:tt [$msb:tt..$lsb:tt] in $slot:tt as $vty:tt) => {
        $(#[$attribute])*
        $($vis)* fn $setter(&mut self, value: $vty) -> &mut Self {
            let mut bits = Bits::<$sty>(self.$slot);
            bits.set_bit_range($msb, $lsb, value);
            self.$slot = bits.into();
            self
        }
    };
    // Match: pub? <getter>,<setter> : <T> []
    /*(($(#[$attribute:meta])*) $getter:tt, $setter:tt : $sty:tt []; $($rest:tt)*) => {
        bitfield_fields!{ @field ($(#[$attribute])*) () $getter, _ : $sty [] in 0 }
        bitfield_fields!{ @field ($(#[$attribute])*) () _, $setter : $sty [] in 0 }
        bitfield_fields!{ $($rest)* }
    };*/
    ($(#[$attribute:meta])* $vis:vis $getter:ident, $setter:ident : $sty:tt []; $($rest:tt)*) => {
        bitfield_fields!{ @field ($(#[$attribute])*) ($vis) $getter, _ : $sty [] in 0 }
        bitfield_fields!{ @field ($(#[$attribute])*) ($vis) _, $setter : $sty [] in 0 }
        bitfield_fields!{ $($rest)* }
    };
    // Match: pub? <getter>,<setter> : <T> [] in <slot>
    ($(#[$attribute:meta])* $vis:vis $getter:ident, _ : $sty:tt [] in $slot:tt; $($rest:tt)*) => {
        bitfield_fields!{ @getter ($(#[$attribute])*) ($vis) $getter : $sty [] in $slot }
        //bitfield_fields!{ @field ($(#[$attribute])*) () _, $setter : $sty [] in $slot }
        bitfield_fields!{ $($rest)* }
    };
    ($(#[$attribute:meta])* $vis:vis _, $setter:ident : $sty:tt [] in $slot:tt; $($rest:tt)*) => {
        //bitfield_fields!{ @field ($(#[$attribute])*) ($vis) $getter, _ : $sty [] in $slot }
        bitfield_fields!{ @setter ($(#[$attribute])*) ($vis) $setter : $sty [] in $slot }
        bitfield_fields!{ $($rest)* }
    };
    // Match: pub? <getter>,<setter> : <T> [<msb>]
    /*($(#[$attribute:meta])* $getter:tt, $setter:tt : $sty:tt [$msb:tt]; $($rest:tt)*) => {
        bitfield_fields!{ @field ($(#[$attribute])*) () $getter, _ : $sty [$msb..$msb] in 0 as $sty }
        bitfield_fields!{ @field ($(#[$attribute])*) () _, $setter : $sty [$msb..$msb] in 0 as $sty }
        bitfield_fields!{ $($rest)* }
    };*/
    ($(#[$attribute:meta])* $vis:vis $getter:ident, $setter:ident : $sty:tt [$msb:tt]; $($rest:tt)*) => {
        bitfield_fields!{ @field ($(#[$attribute])*) ($vis) $getter, _ : $sty [$msb..$msb] in 0 as $sty }
        bitfield_fields!{ @field ($(#[$attribute])*) ($vis) _, $setter : $sty [$msb..$msb] in 0 as $sty }
        bitfield_fields!{ $($rest)* }
    };
    // Match: pub? <getter>,<setter> : <T> [<msb>] in <slot>
    /*($(#[$attribute:meta])* $getter:tt, $setter:tt : $sty:tt [$msb:tt] in $slot:tt; $($rest:tt)*) => {
        bitfield_fields!{ @field ($(#[$attribute])*) () $getter, _ : $sty [$msb..$msb] in $slot as $sty }
        bitfield_fields!{ @field ($(#[$attribute])*) () _, $setter : $sty [$msb..$msb] in $slot as $sty }
        bitfield_fields!{ $($rest)* }
    };*/
    ($(#[$attribute:meta])* $vis:vis $getter:ident, $setter:ident : $sty:tt [$msb:tt] in $slot:tt; $($rest:tt)*) => {
        bitfield_fields!{ @field ($(#[$attribute])*) ($vis) $getter, _ : $sty [$msb..$msb] in $slot as $sty }
        bitfield_fields!{ @field ($(#[$attribute])*) ($vis) _, $setter : $sty [$msb..$msb] in $slot as $sty }
        bitfield_fields!{ $($rest)* }
    };
    // Match: pub? <getter>,<setter> : <T> [<msb>] as <U>
    /*($(#[$attribute:meta])* $getter:tt, $setter:tt : $sty:tt [$msb:tt] as $vty:tt; $($rest:tt)*) => {
        bitfield_fields!{ @field ($(#[$attribute])*) () $getter, _ : $sty [$msb..$msb] in 0 as $vty }
        bitfield_fields!{ @field ($(#[$attribute])*) () _, $setter : $sty [$msb..$msb] in 0 as $vty }
        bitfield_fields!{ $($rest)* }
    };*/
    ($(#[$attribute:meta])* $vis:vis $getter:ident, $setter:ident : $sty:tt [$msb:tt] as $vty:tt; $($rest:tt)*) => {
        bitfield_fields!{ @field ($(#[$attribute])*) ($vis) $getter, _ : $sty [$msb..$msb] in 0 as $vty }
        bitfield_fields!{ @field ($(#[$attribute])*) ($vis) _, $setter : $sty [$msb..$msb] in 0 as $vty }
        bitfield_fields!{ $($rest)* }
    };
    // Match: pub? <getter>,<setter> : <T> [<msb>] in <slot> as <U>
    /*($(#[$attribute:meta])* $getter:tt, $setter:tt : $sty:tt [$msb:tt] in $slot:tt as $vty:tt; $($rest:tt)*) => {
        bitfield_fields!{ @field ($(#[$attribute])*) () $getter, _ : $sty [$msb..$msb] in $slot as $vty }
        bitfield_fields!{ @field ($(#[$attribute])*) () _, $setter : $sty [$msb..$msb] in $slot as $vty }
        bitfield_fields!{ $($rest)* }
    };*/
    ($(#[$attribute:meta])* $vis:vis $getter:ident, $setter:ident : $sty:tt [$msb:tt] in $slot:tt as $vty:tt; $($rest:tt)*) => {
        bitfield_fields!{ @field ($(#[$attribute])*) ($vis) $getter, _ : $sty [$msb..$msb] in $slot as $vty }
        bitfield_fields!{ @field ($(#[$attribute])*) ($vis) _, $setter : $sty [$msb..$msb] in $slot as $vty }
        bitfield_fields!{ $($rest)* }
    };
    // Match: pub? <getter>,<setter> : <T> [<msb>..<lsb>]
    /*($(#[$attribute:meta])* $getter:tt, $setter:tt : $sty:tt [$msb:tt..$lsb:tt]; $($rest:tt)*) => {
        bitfield_fields!{ @field ($(#[$attribute])*) () $getter, _ : $sty [$msb..$lsb] in 0 as $sty }
        bitfield_fields!{ @field ($(#[$attribute])*) () _, $setter : $sty [$msb..$lsb] in 0 as $sty }
        bitfield_fields!{ $($rest)* }
    };*/
    ($(#[$attribute:meta])* $vis:vis $getter:ident, $setter:ident : $sty:tt [$msb:tt..$lsb:tt]; $($rest:tt)*) => {
        bitfield_fields!{ @field ($(#[$attribute])*) ($vis) $getter, _ : $sty [$msb..$lsb] in 0 as $sty }
        bitfield_fields!{ @field ($(#[$attribute])*) ($vis) _, ($setter) : $sty [$msb..$lsb] in 0 as $sty }
        bitfield_fields!{ $($rest)* }
    };
    // Match: pub? <getter>,<setter> : <T> [<msb>..<lsb>] in <slot>
    /*($(#[$attribute:meta])* $getter:tt, $setter:tt : $sty:tt [$msb:tt..$lsb:tt] in $slot:tt; $($rest:tt)*) => {
        bitfield_fields!{ @field ($(#[$attribute])*) () $getter, _ : $sty [$msb..$lsb] in $slot as $sty }
        bitfield_fields!{ @field ($(#[$attribute])*) () _, $setter : $sty [$msb..$lsb] in $slot as $sty }
        bitfield_fields!{ $($rest)* }
    };*/
    ($(#[$attribute:meta])* $vis:vis $getter:ident, $setter:ident : $sty:tt [$msb:tt..$lsb:tt] in $slot:tt; $($rest:tt)*) => {
        bitfield_fields!{ @field ($(#[$attribute])*) ($vis) $getter, _ : $sty [$msb..$lsb] in $slot as $sty }
        bitfield_fields!{ @field ($(#[$attribute])*) ($vis) _, $setter : $sty [$msb..$lsb] in $slot as $sty }
        bitfield_fields!{ $($rest)* }
    };
    // Match: pub? <getter>,<setter> : <T> [<msb>..<lsb>] as <U>
    /*($(#[$attribute:meta])* $getter:tt, $setter:tt : $sty:tt [$msb:tt..$lsb:tt] as $vty:tt; $($rest:tt)*) => {
        bitfield_fields!{ @field ($(#[$attribute])*) () $getter, _ : $sty [$msb..$lsb] in 0 as $vty }
        bitfield_fields!{ @field ($(#[$attribute])*) () _, $setter : $sty [$msb..$lsb] in 0 as $vty }
        bitfield_fields!{ $($rest)* }
    };*/
    ($(#[$attribute:meta])* $vis:vis $getter:ident, $setter:ident : $sty:tt [$msb:tt..$lsb:tt] as $vty:tt; $($rest:tt)*) => {
        bitfield_fields!{ @field ($(#[$attribute])*) ($vis) $getter, _ : $sty [$msb..$lsb] in 0 as $vty }
        bitfield_fields!{ @field ($(#[$attribute])*) ($vis) _, $setter : $sty [$msb..$lsb] in 0 as $vty }
        bitfield_fields!{ $($rest)* }
    };
    // Match: pub? <getter>,<setter> : <T> [<msb>..<lsb>] in <slot> as <U>
    /*($(#[$attribute:meta])* $getter:tt, $setter:tt : $sty:tt [$msb:tt..$lsb:tt] in $slot:tt as $vty:tt; $($rest:tt)*) => {
        bitfield_fields!{ @field ($(#[$attribute])*) () $getter, _ : $sty [$msb..$lsb] in $slot as $vty }
        bitfield_fields!{ @field ($(#[$attribute])*) () _, $setter : $sty [$msb..$lsb] in $slot as $vty }
        bitfield_fields!{ $($rest)* }
    };*/
    ($(#[$attribute:meta])* $vis:vis $getter:ident, $setter:ident : $sty:tt [$msb:tt..$lsb:tt] in $slot:tt as $vty:tt; $($rest:tt)*) => {
        bitfield_fields!{ @field ($(#[$attribute])*) ($vis) $getter, _ : $sty [$msb..$lsb] in $slot as $vty }
        bitfield_fields!{ @field ($(#[$attribute])*) ($vis) _, $setter : $sty [$msb..$lsb] in $slot as $vty }
        bitfield_fields!{ $($rest)* }
    };
}

impl_bits! {
    i8 => [bool, i8, i16, i32, i64, u8, u16, u32, u64];
    i16 => [bool, i8, i16, i32, i64, u8, u16, u32, u64];
    i32 => [bool, i8, i16, i32, i64, u8, u16, u32, u64];
    i64 => [bool, i8, i16, i32, i64, u8, u16, u32, u64];
    u8 => [bool, i8, i16, i32, i64, u8, u16, u32, u64];
    u16 => [bool, i8, i16, i32, i64, u8, u16, u32, u64];
    u32 => [bool, i8, i16, i32, i64, u8, u16, u32, u64];
    u64 => [bool, i8, i16, i32, i64, u8, u16, u32, u64];
}

macro_rules! at {
    () => {};
    (@inner ($(#[$attribute:meta])*) $getter:tt, _) => {
        println!("|{:?}|", stringify!($getter));
    };
    /*($(#[$attribute:meta])* $getter:ident, $setter:ident; $($rest:tt)*) => {
        println!("a |{:?}|", stringify!($(#[$attribute])*));
        println!("a |{:?},{:?}|", stringify!($getter), stringify!($setter));
        at! { @inner ($(#[$attribute])*) $getter, _ }
        at! { $($rest)* }
    };*/
    ($(#[$attribute:meta])* $v:vis $getter:ident, $setter:ident; $($rest:tt)*) => {
        println!("b |{:?}|", stringify!($(#[$attribute])*));
        println!("b |{:?},{:?}|", stringify!($getter), stringify!($setter));
        at! { @inner ($(#[$attribute])*) $getter, _ }
        at! { $($rest)* }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Copy, Clone, Default, Debug)]
    struct FooBar(u8,u16,u32,u64);

    impl FooBar {
        bitfield_fields! {
            // u8
            f1a, set_f1a : u8 [0];
            f1b, set_f1b : u8 [1] in 0;
            f1c, set_f1c : u8 [2] as bool;
            f1d, set_f1d : u8 [4..3] in 0 as u8;
            f1e, set_f1e : u8 [7..5] in 0 as u16;
            pub all1, _ : u8 [] in 0;
            // u16
            f2a, set_f2a : u16 [15] in 1 as bool;
            f2b, set_f2b : u16 [14..0] in 1;
            pub all2, _ : u16 [] in 1;
            // u32
            f3a, set_f3a : u32 [7..0] in 2 as u8;
            f3b, set_f3b : u32 [15..8] in 2 as u8;
            f3c, set_f3c : u32 [15..8] in 2 as u8;
            pub all3, _ : u32 [] in 2;
            // u64
            rsv, _ : u64 [] in 3;
            _, set_rsv: u64 [] in 3;
        }
    }

    #[test]
    fn test_foobar() {
        let mut a = FooBar(0x77, 0x55AA, 0xFF77_AA55, 0x0000_FFFF_FF77_AA55);
        assert_eq!(1, a.f1a());
        assert_eq!(1, a.f1b());
        assert_eq!(true, a.f1c());
        assert_eq!(false, a.set_f1c(false).f1c());
        assert_eq!(0x73, a.all1());
        assert_eq!(2, a.f1d());
        assert_eq!(0, a.set_f1d(0).f1d());
        assert_eq!(0x63, a.all1());
        assert_eq!(3, a.f1e());
        assert_eq!(7, a.set_f1e(7).f1e());
        assert_eq!(0xE3, a.all1());
        assert_eq!(0x0000_FFFF_FF77_AA55, a.rsv());
    }

    #[test]
    fn test_at() {
        at!{
            #[inline]
            #[hello]
            pub aka, set_aka;

            #[inline]
            foo, bar;
        }
    }
}
