#![allow(dead_code)]
#![allow(non_snake_case)]

pub struct BitField(u8, u8);

trait BitFieldOps<T> {
    fn get(&self, target: T) -> T;
    fn set(&self, target: &mut T, value: T);
}

trait BitFields {
    fn field(&self, index: usize) -> &'static BitField;
    fn fields(&self) -> &'static [BitField];
}

impl BitField {
    #[inline]
    fn new(width: u8, offset: u8) -> Self {
        Self {
            0: width,
            1: offset,
        }
    }

    #[inline]
    fn mask_u8(&self) -> u8 {
        match self.0 {
            0..=7 => (1u8.wrapping_shl(self.0 as u32) - 1).wrapping_shl(self.1 as u32),
            _ => 0xffu8,
        }
    }

    #[inline]
    fn mask_u16(&self) -> u16 {
        match self.0 {
            0..=15 => (1u16.wrapping_shl(self.0 as u32) - 1).wrapping_shl(self.1 as u32),
            _ => 0xffffu16,
        }
    }

    #[inline]
    fn mask_u32(&self) -> u32 {
        match self.0 {
            0..=31 => (1u32.wrapping_shl(self.0 as u32) - 1).wrapping_shl(self.1 as u32),
            _ => 0xffff_ffffu32,
        }
    }

    #[inline]
    fn mask_u64(&self) -> u64 {
        match self.0 {
            0..=63 => (1u64.wrapping_shl(self.0 as u32) - 1).wrapping_shl(self.1 as u32),
            _ => 0xffff_ffff_ffff_ffffu64,
        }
    }
}

impl BitFieldOps<u8> for BitField {
    #[inline]
    fn get(&self, target: u8) -> u8 {
        (target & self.mask_u8()) >> self.1
    }

    #[inline]
    fn set(&self, target: &mut u8, value: u8) {
        *target = (*target & !self.mask_u8()) | ((value << self.1) & self.mask_u8())
    }
}

impl BitFieldOps<u16> for BitField {
    #[inline]
    fn get(&self, target: u16) -> u16 {
        (target & self.mask_u16()) >> self.1
    }

    #[inline]
    fn set(&self, target: &mut u16, value: u16) {
        *target = (*target & !self.mask_u16()) | ((value << self.1) & self.mask_u16())
    }
}

impl BitFieldOps<u32> for BitField {
    #[inline]
    fn get(&self, target: u32) -> u32 {
        (target & self.mask_u32()) >> self.1
    }

    #[inline]
    fn set(&self, target: &mut u32, value: u32) {
        *target = (*target & !self.mask_u32()) | ((value << self.1) & self.mask_u32())
    }
}

impl BitFieldOps<u64> for BitField {
    #[inline]
    fn get(&self, target: u64) -> u64 {
        (target & self.mask_u64()) >> self.1
    }

    #[inline]
    fn set(&self, target: &mut u64, value: u64) {
        *target = (*target & !self.mask_u64()) | ((value << self.1) & self.mask_u64())
    }
}

#[allow(unused_macros)]
macro_rules! all_fn {
    ($DataType:ty) => {
        #[inline]
        pub fn all(&self) -> $DataType { self.0 }
        pub fn clear(&mut self) -> &mut Self { self.0 = 0; self }
        pub fn fill(&mut self, value: $DataType) -> &mut Self { self.0 = value; self }
    }
}

#[allow(unused_macros)]
macro_rules! decl_fields {
    ($w0:expr, $w1:expr) => {
        static MYFIELDS: &'static [BitField] = &[BitField($w0, 0), BitField($w1, $w0)];
    };

    ($w0:expr, $w1:expr, $w2:expr) => {
        static MYFIELDS: &'static [BitField] = &[
            BitField($w0, 0),
            BitField($w1, $w0),
            BitField($w2, $w1 + $w0),
        ];
    };

    ($w0:expr, $w1:expr, $w2:expr, $w3:expr) => {
        static MYFIELDS: &'static [BitField] = &[
            BitField($w0, 0),
            BitField($w1, $w0),
            BitField($w2, $w1 + $w0),
            BitField($w3, $w2 + $w1 + $w0),
        ];
    };

    ($w0:expr, $w1:expr, $w2:expr, $w3:expr, $w4:expr) => {
        static MYFIELDS: &'static [BitField] = &[
            BitField($w0, 0),
            BitField($w1, $w0),
            BitField($w2, $w1 + $w0),
            BitField($w3, $w2 + $w1 + $w0),
            BitField($w4, $w3 + $w2 + $w1 + $w0),
        ];
    };

    ($w0:expr, $w1:expr, $w2:expr, $w3:expr, $w4:expr, $w5:expr) => {
        static MYFIELDS: &'static [BitField] = &[
            BitField($w0, 0),
            BitField($w1, $w0),
            BitField($w2, $w1 + $w0),
            BitField($w3, $w2 + $w1 + $w0),
            BitField($w4, $w3 + $w2 + $w1 + $w0),
            BitField($w5, $w4 + $w3 + $w2 + $w1 + $w0),
        ];
    };

    ($w0:expr, $w1:expr, $w2:expr, $w3:expr, $w4:expr, $w5:expr, $w6:expr) => {
        static MYFIELDS: &'static [BitField] = &[
            BitField($w0, 0),
            BitField($w1, $w0),
            BitField($w2, $w1 + $w0),
            BitField($w3, $w2 + $w1 + $w0),
            BitField($w4, $w3 + $w2 + $w1 + $w0),
            BitField($w5, $w4 + $w3 + $w2 + $w1 + $w0),
            BitField($w6, $w5 + $w4 + $w3 + $w2 + $w1 + $w0),
        ];
    };

    ($w0:expr, $w1:expr, $w2:expr, $w3:expr, $w4:expr, $w5:expr, $w6:expr, $w7:expr) => {
        static MYFIELDS: &'static [BitField] = &[
            BitField($w0, 0),
            BitField($w1, $w0),
            BitField($w2, $w1 + $w0),
            BitField($w3, $w2 + +w1 + $w0),
            BitField($w4, $w3 + $w2 + $w1 + $w0),
            BitField($w5, $w4 + $w3 + $w2 + $w1 + $w0),
            BitField($w6, $w5 + $w4 + $w3 + $w2 + $w1 + $w0),
            BitField($w7, $w6 + $w5 + $w4 + $w3 + $w2 + $w1 + $w0),
        ];
    };
}

#[allow(unused_macros)]
macro_rules! decl_type {
    ($TypeName:ident, $DataType:ty) => {
        #[repr(C)]
        pub struct $TypeName($DataType);
    };
}

#[allow(unused_macros)]
macro_rules! impl_fields {
    ($TypeName:ident, [$($p:expr),*]) => {
        impl BitFields for $TypeName {
            #[inline]
            fn field(&self, index: usize) -> &'static BitField {
                &self.fields()[index]
            }
            #[inline]
            fn fields(&self) -> &'static [BitField] {
                decl_fields!($($p),*);
                MYFIELDS
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! get_set_fn {
    ($DataType:ty, $F:ident, $TypeName:expr) => {
        paste::item! {
            #[inline]
            pub fn $F(&self) -> $DataType {
                self.field($TypeName).get(self.0) as $DataType
            }
            #[inline]
            pub fn [<set_ $F>](&mut self, value: $DataType) -> &mut Self {
                self.field($TypeName).set(&mut self.0, value);
                self
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! impl_getter_setter {
    ($TypeName:ident, $DataType:ty, [$f0:ident, $f1:ident]) => {
        impl $TypeName {
            all_fn!($DataType);
            get_set_fn!($DataType, $f0, 0);
            get_set_fn!($DataType, $f1, 1);
        }
    };
    ($TypeName:ident, $DataType:ty, [$f0:ident, $f1:ident, $f2:ident]) => {
        impl $TypeName {
            all_fn!($DataType);
            get_set_fn!($DataType, $f0, 0);
            get_set_fn!($DataType, $f1, 1);
            get_set_fn!($DataType, $f2, 2);
        }
    };
    ($TypeName:ident, $DataType:ty, [$f0:ident, $f1:ident, $f2:ident, $f3:ident]) => {
        impl $TypeName {
            all_fn!($DataType);
            get_set_fn!($DataType, $f0, 0);
            get_set_fn!($DataType, $f1, 1);
            get_set_fn!($DataType, $f2, 2);
            get_set_fn!($DataType, $f3, 3);
        }
    };
    ($TypeName:ident, $DataType:ty, [$f0:ident, $f1:ident, $f2:ident, $f3:ident, $f4:ident]) => {
        impl $TypeName {
            all_fn!($DataType);
            get_set_fn!($DataType, $f0, 0);
            get_set_fn!($DataType, $f1, 1);
            get_set_fn!($DataType, $f2, 2);
            get_set_fn!($DataType, $f3, 3);
            get_set_fn!($DataType, $f4, 4);
        }
    };
    ($TypeName:ident, $DataType:ty, [$f0:ident, $f1:ident, $f2:ident, $f3:ident, $f4:ident, $f5:ident]) => {
        impl $TypeName {
            all_fn!($DataType);
            get_set_fn!($DataType, $f0, 0);
            get_set_fn!($DataType, $f1, 1);
            get_set_fn!($DataType, $f2, 2);
            get_set_fn!($DataType, $f3, 3);
            get_set_fn!($DataType, $f4, 4);
            get_set_fn!($DataType, $f5, 5);
        }
    };
    ($TypeName:ident, $DataType:ty, [$f0:ident, $f1:ident, $f2:ident, $f3:ident, $f4:ident, $f5:ident, $f6:ident]) => {
        impl $TypeName {
            all_fn!($DataType);
            get_set_fn!($DataType, $f0, 0);
            get_set_fn!($DataType, $f1, 1);
            get_set_fn!($DataType, $f2, 2);
            get_set_fn!($DataType, $f3, 3);
            get_set_fn!($DataType, $f4, 4);
            get_set_fn!($DataType, $f5, 5);
            get_set_fn!($DataType, $f6, 6);
        }
    };
    ($TypeName:ident, $DataType:ty, [$f0:ident, $f1:ident, $f2:ident, $f3:ident, $f4:ident, $f5:ident, $f6:ident, $f7:ident]) => {
        impl $TypeName {
            all_fn!($DataType);
            get_set_fn!($DataType, $f0, 0);
            get_set_fn!($DataType, $f1, 1);
            get_set_fn!($DataType, $f2, 2);
            get_set_fn!($DataType, $f3, 3);
            get_set_fn!($DataType, $f4, 4);
            get_set_fn!($DataType, $f5, 5);
            get_set_fn!($DataType, $f6, 6);
            get_set_fn!($DataType, $f7, 7);
        }
    };
}

#[macro_export]
macro_rules! bitfeilds {
    ($TypeName:ident($DataType:ty), {$f0:tt:$w0:expr, $f1:tt:$w1:expr}) => {
        decl_type!($TypeName, $DataType);
        impl_fields!($TypeName, [$w0, $w1]);
        impl_getter_setter!($TypeName, $DataType, [$f0, $f1]);
    };
    ($TypeName:ident($DataType:ty), {$f0:ident:$w0:expr, $f1:ident:$w1:expr, $f2:ident:$w2:expr}) => {
        decl_type!($TypeName, $DataType);
        impl_fields!($TypeName, [$w0, $w1, $w2]);
        impl_getter_setter!($TypeName, $DataType, [$f0, $f1, $f2]);
    };
    ($TypeName:ident($DataType:ty), {$f0:ident:$w0:expr, $f1:ident:$w1:expr, $f2:ident:$w2:expr, $f3:ident:$w3:expr}) => {
        decl_type!($TypeName, $DataType);
        impl_fields!($TypeName, [$w0, $w1, $w2, $w3]);
        impl_getter_setter!($TypeName, $DataType, [$f0, $f1, $f2, $f3]);
    };
    ($TypeName:ident($DataType:ty), {$f0:ident:$w0:expr, $f1:ident:$w1:expr, $f2:ident:$w2:expr, $f3:ident:$w3:expr, $f4:ident:$w4:expr}) => {
        decl_type!($TypeName, $DataType);
        impl_fields!($TypeName, [$w0, $w1, $w2, $w3, $w4]);
        impl_getter_setter!($TypeName, $DataType, [$f0, $f1, $f2, $f3, $f4]);
    };
    ($TypeName:ident($DataType:ty), {$f0:ident:$w0:expr, $f1:ident:$w1:expr, $f2:ident:$w2:expr, $f3:ident:$w3:expr, $f4:ident:$w4:expr, $f5:ident:$w5:expr}) => {
        decl_type!($TypeName, $DataType);
        impl_fields!($TypeName, [$w0, $w1, $w2, $w3, $w4, $w5]);
        impl_getter_setter!($TypeName, $DataType, [$f0, $f1, $f2, $f3, $f4, $f5]);
    };
    ($TypeName:ident($DataType:ty), {$f0:ident:$w0:expr, $f1:ident:$w1:expr, $f2:ident:$w2:expr, $f3:ident:$w3:expr, $f4:ident:$w4:expr, $f5:ident:$w5:expr, $f6:ident:$w6:expr}) => {
        decl_type!($TypeName, $DataType);
        impl_fields!($TypeName, [$w0, $w1, $w2, $w3, $w4, $w5, $f6]);
        impl_getter_setter!($TypeName, $DataType, [$f0, $f1, $f2, $f3, $f4, $f5, $f6]);
    };
    ($TypeName:ident($DataType:ty), {$f0:ident:$w0:expr, $f1:ident:$w1:expr, $f2:ident:$w2:expr, $f3:ident:$w3:expr, $f4:ident:$w4:expr, $f5:ident:$w5:expr, $f6:ident:$w6:expr, $f7:ident:$w7:expr}) => {
        decl_type!($TypeName, $DataType);
        impl_fields!($TypeName, [$w0, $w1, $w2, $w3, $w4, $w5, $f6]);
        impl_getter_setter!(
            $TypeName,
            $DataType,
            [$f0, $f1, $f2, $f3, $f4, $f5, $f6, $f7]
        );
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn test_bitfield_mask_u8() {
        let a = (1 << 1) - 1;
        assert_eq!(0x01 << 0, BitField(1, 0).mask_u8());
        assert_eq!(0x03 << 0, BitField(2, 0).mask_u8());
        assert_eq!(0x07 << 0, BitField(3, 0).mask_u8());
        assert_eq!(0x0f << 0, BitField(4, 0).mask_u8());
        assert_eq!(0x1f << 0, BitField(5, 0).mask_u8());
        assert_eq!(0x3f << 0, BitField(6, 0).mask_u8());
        assert_eq!(0x7f << 0, BitField(7, 0).mask_u8());
        assert_eq!(0xff << 0, BitField(8, 0).mask_u8());
        assert_eq!(0x03 << 1, BitField(2, 1).mask_u8());
    }

    // u8
    bitfeilds!(Bi8_1232(u8), { a:1, b: 2, c:3, d:2 });

    #[test]
    fn test_bitfield_u8() {
        assert_eq!(1, size_of::<Bi8_1232>());
        assert_eq!(4, Bi8_1232(0).fields().len());
        let mut a = Bi8_1232(0xff);
        assert_eq!(1, a.a());
        assert_eq!(3, a.b());
        assert_eq!(7, a.c());
        assert_eq!(3, a.d());
        assert_eq!(0xfe, a.set_a(0).all());
        assert_eq!(0, a.a());
        assert_eq!(0xf8, a.set_b(0).all());
        assert_eq!(0, a.b());
        assert_eq!(0xc0, a.set_c(0).all());
        assert_eq!(0, a.c());
        assert_eq!(0x00, a.set_d(0).all());
        assert_eq!(0, a.d());
        assert_eq!(0x55, a.fill(0x55).all());
        assert_eq!(0x00, a.clear().all());
        assert_eq!(0xff, a.fill(0xff).all());
        assert_eq!(1, a.a());
        assert_eq!(3, a.b());
        assert_eq!(7, a.c());
        assert_eq!(3, a.d());
    }

    // u16
    bitfeilds!(Bi16_123451(u16), { a:1, b:2, c:3, d:4, e:5, f:1 });

    #[test]
    fn test_bitfield_u16() {
        let mut a = Bi16_123451(0xffff);
        assert_eq!(2, size_of::<Bi16_123451>());
        assert_eq!(6, a.fields().len());
        assert_eq!(0xffff, a.all());
        assert_eq!(1, a.a());
        assert_eq!(3, a.b());
        assert_eq!(7, a.c());
        assert_eq!(15, a.d());
        assert_eq!(31, a.e());
        assert_eq!(1, a.f());
        assert_eq!(0, a.set_a(0).a());
        assert_eq!(0, a.set_b(0).b());
        assert_eq!(0, a.set_c(0).c());
        assert_eq!(0, a.set_d(0).d());
        assert_eq!(0, a.set_e(0).e());
        assert_eq!(0, a.set_f(0).f());
        assert_eq!(0, a.all());
        assert_eq!(0x55AA, a.fill(0x55AA).all());
        assert_eq!(0x00, a.clear().all());
        assert_eq!(0xffff, a.fill(0xffff).all());
        assert_eq!(1, a.a());
        assert_eq!(3, a.b());
        assert_eq!(7, a.c());
        assert_eq!(15, a.d());
        assert_eq!(31, a.e());
        assert_eq!(1, a.f());
    }
}
