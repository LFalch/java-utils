#![warn(clippy::all, missing_docs, trivial_casts, trivial_numeric_casts)]
//! Implementations of Java classes in pure Rust
//!
//! For interop with your old Java applications or the like

/// Implentations from `java.lang.Object`
///
/// Only consists of the `HashCode` trait, since the
/// other methods don't make sense to port to Rust
pub mod object;
/// Implements of classes from `java.util`
pub mod util;

pub use crate::object::HashCode;
pub use crate::util::Random;

#[cfg(test)]
mod tests {
    use crate::Random;
    use crate::HashCode;
    const RANDOMS: [i32; 64] = [
        2992, 3717, 3763, 3320, 3762, 892, 2783, 1165, 321, 2041, 101, 3492, 2864, 3273, 3297, 1097, 619, 2353, 3787, 1722, 3128, 2937, 13, 2184,
        3016, 1476, 3916, 1858, 3373, 529, 772, 2640, 1335, 1681, 3078, 774, 1148, 1847, 942, 2404, 3308, 3015, 3109, 1705, 3200, 1909, 3658, 1571,
        2146, 3201, 210, 3536, 1420, 508, 1966, 2000, 3713, 742, 2336, 2204, 2284, 3441, 2341, 4063,
    ];
    const RANDOMS2: [i32; 64] = [
        1130, 3485, 662, 3602, 558, 2973, 2899, 3534, 3023, 2378, 1110, 1529, 3209, 1193, 3207, 610, 3376, 2053, 1746, 3646, 4088, 2404, 138, 712,
        2448, 1359, 1469, 744, 3838, 1962, 282, 3748, 3875, 3080, 2638, 311, 2934, 1084, 2032, 413, 0, 3776, 3639, 2840, 1359, 1152, 763, 2894, 1316,
        3727, 800, 2731, 2211, 2522, 400, 1092, 3237, 2462, 34, 871, 3906, 3476, 802, 2946,
    ];
    #[allow(clippy::unreadable_literal)]
    const STRINGS_TO_TEST: [(&str, i32); 4] = [
        ("", 0),
        ("hello", 99162322),
        ("Þɪs ɪn jʉ͡u tiː ɛf eɪt", 1666277289),
        ("Здразтвуйте", 1364145635),
    ];

    #[test]
    fn random_ints_with_seed_4() {
        let mut r = Random::new(4);
        for (i, java_r) in RANDOMS.iter().enumerate() {
            let k = r.next_int(4096);
            println!("{}", k);
            assert_eq!(k as i32, *java_r, "{}th iteration", i);
        }
    }
    #[test]
    fn random_ints_with_seed_4_non_power_of_two() {
        let mut r = Random::new(4);
        for (i, java_r) in RANDOMS2.iter().enumerate() {
            let k = r.next_int(4097);
            println!("{}", k);
            assert_eq!(k as i32, *java_r, "{}th iteration", i);
        }
    }
    #[test]
    fn test_strings() {
        for (s, n) in STRINGS_TO_TEST.iter() {
            assert_eq!(s.hash_code(), *n, "string {:?}", s);
        }
    }
    #[test]
    fn test_ints() {
        // java.lang.Byte
        assert_eq!((127u8).hash_code(), 127);
        assert_eq!((-1i8).hash_code(), -1);
        assert_eq!((255u8).hash_code(), -1, "{}", (255u8).hash_code());
        // java.lang.Short
        assert_eq!((32767i16).hash_code(), 32767);
        assert_eq!((-1i16).hash_code(), -1);
        assert_eq!((65535u16).hash_code(), -1);
        // java.lang.Integer
        assert_eq!((2_147_483_647i32).hash_code(), 2_147_483_647);
        assert_eq!((-1i32).hash_code(), -1);
        assert_eq!((4_294_967_295u32).hash_code(), -1);
        // java.lang.Long
        assert_eq!((9_223_372_036_854_775_807i64).hash_code(), -2_147_483_648);
        assert_eq!((-1i64).hash_code(), 0);
        assert_eq!((18_446_744_073_709_551_615u64).hash_code(), 0);
    }
    #[test]
    fn test_bools() {
        assert_eq!(true.hash_code(), 1231);
        assert_eq!(false.hash_code(), 1237);
    }
    use std::{f32, f64};
    #[test]
    fn test_floats() {
        assert_eq!(4124.012f32.hash_code(), 1_166_073_881);
        assert_eq!(4_124.041_241_235_123f64.hash_code(), -830_930_928);
        assert_eq!(f32::NAN.hash_code(), 2_143_289_344);
        assert_eq!(f64::NAN.hash_code(), 2_146_959_360);
        assert_eq!(f32::NEG_INFINITY.hash_code(), -8_388_608);
        assert_eq!(f64::NEG_INFINITY.hash_code(), -1_048_576);
    }
}
