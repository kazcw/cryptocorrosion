//#![no_std]

//use crypto_simd::*;

// Design:
// - safety: safe creation of any machine type is done only by instance methods of a
//   Machine (which is a ZST + Copy type), which can only by created unsafely or safely
//   through feature detection (e.g. fn AVX2::try_get() -> Option<Machine>).

use std::arch::x86_64::{__m128i, __m256i};

mod avx;
mod sse2;

// crate minimums: sse2, x86_64

pub mod crypto_simd_new {
    use core::ops::{Add, AddAssign, BitAnd, BitOr, BitXor, BitXorAssign, Not};

    pub trait AndNot {
        type Output;
        fn andnot(self, rhs: Self) -> Self::Output;
    }
    pub trait BSwap {
        fn bswap(self) -> Self;
    }
    /// Ops that depend on word size
    pub trait ArithOps: Add<Output = Self> + AddAssign + Sized + Copy + Clone + BSwap {}
    /// Ops that are independent of word size and endian
    pub trait BitOps0:
        BitAnd<Output = Self>
        + BitOr<Output = Self>
        + BitXor<Output = Self>
        + BitXorAssign
        + Not<Output = Self>
        + AndNot<Output = Self>
        + Sized
        + Copy
        + Clone
    {
}

    pub trait BitOps32: BitOps0 + RotateEachWord32 {}
    pub trait BitOps64: BitOps32 + RotateEachWord64 {}
    pub trait BitOps128: BitOps64 + RotateEachWord128 {}

    pub trait RotateEachWord32 {
        fn rotate_each_word_right7(self) -> Self;
        fn rotate_each_word_right8(self) -> Self;
        fn rotate_each_word_right11(self) -> Self;
        fn rotate_each_word_right12(self) -> Self;
        fn rotate_each_word_right16(self) -> Self;
        fn rotate_each_word_right20(self) -> Self;
        fn rotate_each_word_right24(self) -> Self;
        fn rotate_each_word_right25(self) -> Self;
    }

    pub trait RotateEachWord64 {
        fn rotate_each_word_right32(self) -> Self;
    }

    pub trait RotateEachWord128 {}
}
pub use crate::crypto_simd_new::{ArithOps, BSwap, BitOps128, BitOps32, BitOps64};

#[allow(non_camel_case_types)]
pub mod crypto_simd_new_types {
    //! Vector type naming scheme:
    //! uN[xP]xL
    //! Unsigned; N-bit words * P bits per lane * L lanes
    //!
    //! A lane is always 128-bits, chosen because common SIMD architectures treat 128-bit units of
    //! wide vectors specially (supporting e.g. intra-lane shuffles), and tend to have limited and
    //! slow inter-lane operations.

    use crate::{
        vec128_storage, vec256_storage, vec512_storage, ArithOps, BitOps128, BitOps32, BitOps64,
        Store, StoreBytes,
    };

    pub trait UnsafeFrom<T> {
        unsafe fn unsafe_from(t: T) -> Self;
    }

    /// A vector composed of two elements, which may be words or themselves vectors.
    pub trait Vec2<W> {
        fn extract(self, i: u32) -> W;
        fn insert(self, w: W, i: u32) -> Self;
    }

    /// A vector composed of four elements, which may be words or themselves vectors.
    pub trait Vec4<W> {
        fn extract(self, i: u32) -> W;
        fn insert(self, w: W, i: u32) -> Self;
    }

    // TODO: multiples of 4 should inherit this
    /// A vector composed of four words; depending on their size, operations may cross lanes.
    pub trait Words4 {
        fn shuffle1230(self) -> Self;
        fn shuffle2301(self) -> Self;
        fn shuffle3012(self) -> Self;
    }

    /// A vector composed one or more lanes each composed of four words.
    pub trait LaneWords4 {
        fn shuffle_lane_words1230(self) -> Self;
        fn shuffle_lane_words2301(self) -> Self;
        fn shuffle_lane_words3012(self) -> Self;
    }

    // TODO: make this a part of BitOps
    /// Exchange neigboring ranges of bits of the specified size
    pub trait Swap64 {
        fn swap1(self) -> Self;
        fn swap2(self) -> Self;
        fn swap4(self) -> Self;
        fn swap8(self) -> Self;
        fn swap16(self) -> Self;
        fn swap32(self) -> Self;
        fn swap64(self) -> Self;
    }

    pub trait u32x4:
        BitOps32 + Store<vec128_storage> + ArithOps + Vec4<u32> + Words4 + LaneWords4 + StoreBytes
    {
}
    pub trait u64x2: BitOps64 + Store<vec128_storage> + ArithOps + Vec2<u64> {}
    pub trait u128x1: BitOps128 + Store<vec128_storage> + Swap64 {}

    pub trait u32x4x2<W: u32x4>:
        BitOps32 + Store<vec256_storage> + Vec2<W> + MultiLane<[W; 2]> + ArithOps
    {
}
    pub trait u64x2x2<W: u64x2>:
        BitOps64 + Store<vec256_storage> + Vec2<W> + MultiLane<[W; 2]> + ArithOps + Words4 + StoreBytes
    {
}
    pub trait u128x2<W: u128x1>:
        BitOps128 + Store<vec256_storage> + Vec2<W> + MultiLane<[W; 2]> + Swap64
    {
}

    pub trait u32x4x4<W: u32x4>:
        BitOps32 + Store<vec512_storage> + Vec4<W> + MultiLane<[W; 4]> + ArithOps + LaneWords4
    {
}
    pub trait u64x2x4<W: u64x2>:
        BitOps64 + Store<vec512_storage> + Vec4<W> + MultiLane<[W; 4]> + ArithOps
    {
}
    // TODO: Words4
    pub trait u128x4<W: u128x1>:
        BitOps128 + Store<vec512_storage> + Vec4<W> + MultiLane<[W; 4]> + Swap64
    {
}

    /// Build a vector from words
    pub trait IntoVec<v> {
        fn into_vec(self) -> v;
    }

    macro_rules! impl_into_vec {
        ($storage:ident, $array:ty, $output:ident) => {
            impl IntoVec<$storage> for $array {
                fn into_vec(self) -> $storage {
                    $storage { $output: self }
                }
            }
        };
    }

    impl_into_vec!(vec128_storage, [u32; 4], u32x4);
    impl_into_vec!(vec128_storage, [u64; 2], u64x2);
    impl_into_vec!(vec128_storage, [u128; 1], u128x1);

    impl_into_vec!(vec256_storage, [u32; 8], u32x8);
    impl_into_vec!(vec256_storage, [u64; 4], u64x4);
    impl_into_vec!(vec256_storage, [u128; 2], u128x2);

    impl_into_vec!(vec512_storage, [u32; 16], u32x16);
    impl_into_vec!(vec512_storage, [u64; 8], u64x8);
    impl_into_vec!(vec512_storage, [u128; 4], u128x4);

    /// A vector composed of multiple 128-bit lanes.
    pub trait MultiLane<Lanes> {
        /// Split a multi-lane vector into single-lane vectors.
        fn to_lanes(self) -> Lanes;
        /// Build a multi-lane vector from individual lanes.
        fn from_lanes(lanes: Lanes) -> Self;
    }

    /// Combine single vectors into a multi-lane vector.
    pub trait VZip<V> {
        fn vzip(self) -> V;
    }

    impl<V, T> VZip<V> for T
    where
        V: MultiLane<T>,
    {
        fn vzip(self) -> V {
            V::from_lanes(self)
        }
    }
}
pub use crate::crypto_simd_new_types::*;

pub(crate) mod features {
    #[derive(Copy, Clone)]
    pub struct YesS3;
    #[derive(Copy, Clone)]
    pub struct NoS3;

    #[derive(Copy, Clone)]
    pub struct YesS4;
    #[derive(Copy, Clone)]
    pub struct NoS4;

    #[derive(Copy, Clone)]
    pub struct YesA2;
    #[derive(Copy, Clone)]
    pub struct NoA2;

    #[derive(Copy, Clone)]
    pub struct YesNI;
    #[derive(Copy, Clone)]
    pub struct NoNI;
}
pub(crate) use crate::features::*;

pub trait Machine: Sized + Copy {
    type S3; // SSSE3
    type S4; // SSE4.1
    type A2; // AVX2
    type NI; // AES

    type u32x4: u32x4;
    type u64x2: u64x2;
    type u128x1: u128x1;

    type u32x4x2: u32x4x2<Self::u32x4>;
    type u64x2x2: u64x2x2<Self::u64x2>;
    type u128x2: u128x2<Self::u128x1>;

    type u32x4x4: u32x4x4<Self::u32x4>;
    type u64x2x4: u64x2x4<Self::u64x2>;
    type u128x4: u128x4<Self::u128x1>;

    fn unpack<V, S>(self, s: S) -> V
    where
        V: Store<S>,
    {
        unsafe { V::unpack(s) }
    }

    fn vec<V, T, S>(self, t: T) -> V
    where
        T: IntoVec<S>,
        V: Store<S>,
    {
        self.unpack(t.into_vec())
    }

    // TODO: require the type to be from this machine!
    fn read_le<V>(self, input: &[u8]) -> V
    where
        V: StoreBytes,
    {
        unsafe { V::unsafe_read_le(input) }
    }

    // TODO: require the type to be from this machine!
    fn read_be<V>(self, input: &[u8]) -> V
    where
        V: StoreBytes,
    {
        unsafe { V::unsafe_read_be(input) }
    }
}

pub mod machine {
    pub mod x86 {
        use crate::*;
        macro_rules! sse2_vectypes {
            () => {
                type u32x4 = sse2::u32x4_sse2<Self::S3, Self::S4, Self::NI>;
                type u64x2 = sse2::u64x2_sse2<Self::S3, Self::S4, Self::NI>;
                type u128x1 = sse2::u128x1_sse2<Self::S3, Self::S4, Self::NI>;

                type u32x4x2 = sse2::u32x4x2_sse2<Self::S3, Self::S4, Self::NI>;
                type u64x2x2 = sse2::u64x2x2_sse2<Self::S3, Self::S4, Self::NI>;
                type u128x2 = sse2::u128x2_sse2<Self::S3, Self::S4, Self::NI>;

                type u32x4x4 = sse2::u32x4x4_sse2<Self::S3, Self::S4, Self::NI>;
                type u64x2x4 = sse2::u64x2x4_sse2<Self::S3, Self::S4, Self::NI>;
                type u128x4 = sse2::u128x4_sse2<Self::S3, Self::S4, Self::NI>;
            };
        }
        #[derive(Copy, Clone)]
        pub struct SSE2;
        impl Machine for SSE2 {
            type S3 = NoS3;
            type S4 = NoS4;
            type A2 = NoA2;
            type NI = NoNI;
            sse2_vectypes!();
        }

        #[derive(Copy, Clone)]
        pub struct SSSE3;
        impl Machine for SSSE3 {
            type S3 = YesS3;
            type S4 = NoS4;
            type A2 = NoA2;
            type NI = NoNI;
            sse2_vectypes!();
        }

        #[derive(Copy, Clone)]
        pub struct SSE41;
        impl Machine for SSE41 {
            type S3 = YesS3;
            type S4 = YesS4;
            type A2 = NoA2;
            type NI = NoNI;
            sse2_vectypes!();
        }
    }
}

/// Generic wrapper for unparameterized storage of any of the possible impls.
/// Converting into and out of this type should be essentially free, although it may be more
/// aligned than a particular impl requires.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub union vec128_storage {
    u32x4: [u32; 4],
    u64x2: [u64; 2],
    u128x1: [u128; 1],
    sse2: __m128i,
}
macro_rules! impl_into {
    ($storage:ident, $array:ty, $name:ident) => {
        impl Into<$array> for $storage {
            #[inline(always)]
            fn into(self) -> $array {
                unsafe { self.$name }
            }
        }
    };
}
impl_into!(vec128_storage, [u32; 4], u32x4);
impl_into!(vec128_storage, [u64; 2], u64x2);
impl_into!(vec128_storage, [u128; 1], u128x1);
impl Store<vec128_storage> for vec128_storage {
    #[inline(always)]
    unsafe fn unpack(p: vec128_storage) -> Self {
        p
    }
    #[inline(always)]
    fn pack(self) -> vec128_storage {
        self
    }
}
impl<'a> Into<&'a [u32; 4]> for &'a vec128_storage {
    fn into(self) -> &'a [u32; 4] {
        unsafe { &self.u32x4 }
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub union vec256_storage {
    u32x8: [u32; 8],
    u64x4: [u64; 4],
    u128x2: [u128; 2],
    sse2: [vec128_storage; 2],
    avx: __m256i,
}
impl_into!(vec256_storage, [u32; 8], u32x8);
impl_into!(vec256_storage, [u64; 4], u64x4);
impl_into!(vec256_storage, [u128; 2], u128x2);

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub union vec512_storage {
    u32x16: [u32; 16],
    u64x8: [u64; 8],
    u128x4: [u128; 4],
    sse2: [vec128_storage; 4],
    avx: [vec256_storage; 2],
}
impl_into!(vec512_storage, [u32; 16], u32x16);
impl_into!(vec512_storage, [u64; 8], u64x8);
impl_into!(vec512_storage, [u128; 4], u128x4);

pub trait Store<S> {
    unsafe fn unpack(p: S) -> Self;
    fn pack(self) -> S;
}

pub trait StoreBytes {
    unsafe fn unsafe_read_le(input: &[u8]) -> Self;
    unsafe fn unsafe_read_be(input: &[u8]) -> Self;
    fn write_le(self, out: &mut [u8]);
    fn write_be(self, out: &mut [u8]);
}
