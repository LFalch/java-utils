use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::ffi::OsStr;
use std::hash::{BuildHasher, Hash};
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;
use std::path::Path;

/// The `hashCode` function on all Java `Objects`
pub trait HashCode {
    /// Returns a Java hash code value for the object.
    ///
    /// This follows the principles of the `java.lang.Object.hashCode` method.
    fn hash_code(&self) -> i32;
}

impl<T: HashCode> HashCode for &T {
    fn hash_code(&self) -> i32 {
        <T>::hash_code(self)
    }
}
impl<T: HashCode> HashCode for &mut T {
    fn hash_code(&self) -> i32 {
        <T>::hash_code(self)
    }
}
impl<T: HashCode> HashCode for Box<T> {
    fn hash_code(&self) -> i32 {
        <T>::hash_code(self)
    }
}
impl<T> HashCode for *const T {
    fn hash_code(&self) -> i32 {
        (*self as usize).hash_code()
    }
}
impl<T> HashCode for *mut T {
    fn hash_code(&self) -> i32 {
        (*self as usize).hash_code()
    }
}
impl<Ret> HashCode for fn() -> Ret {
    fn hash_code(&self) -> i32 {
        (*self as usize).hash_code()
    }
}
impl<Ret, A> HashCode for fn(A) -> Ret {
    fn hash_code(&self) -> i32 {
        (*self as usize).hash_code()
    }
}
impl<Ret, A, B> HashCode for fn(A, B) -> Ret {
    fn hash_code(&self) -> i32 {
        (*self as usize).hash_code()
    }
}
impl<Ret, A, B, C> HashCode for fn(A, B, C) -> Ret {
    fn hash_code(&self) -> i32 {
        (*self as usize).hash_code()
    }
}
impl<Ret, A, B, C, D> HashCode for fn(A, B, C, D) -> Ret {
    fn hash_code(&self) -> i32 {
        (*self as usize).hash_code()
    }
}
impl<Ret, A, B, C, D, E> HashCode for fn(A, B, C, D, E) -> Ret {
    fn hash_code(&self) -> i32 {
        (*self as usize).hash_code()
    }
}
impl<Ret, A, B, C, D, E, F> HashCode for fn(A, B, C, D, E, F) -> Ret {
    fn hash_code(&self) -> i32 {
        (*self as usize).hash_code()
    }
}
impl HashCode for () {
    fn hash_code(&self) -> i32 {
        0
    }
}
impl<T: HashCode> HashCode for Option<&T> {
    fn hash_code(&self) -> i32 {
        self.map(|e| e.hash_code()).unwrap_or(0)
    }
}
impl<T: HashCode, U: HashCode> HashCode for (T, U) {
    fn hash_code(&self) -> i32 {
        self.0.hash_code() ^ self.1.hash_code()
    }
}

macro_rules! impl_for_iterable {
    ($($type:ty:[ $( $i:tt: $($t:ident),*);+ ] )*) => (
        $(
            impl< $( $i: $($t+)* ),* > HashCode for $type {
                fn hash_code(&self) -> i32 {
                    self.iter()
                        .fold(1, |hash_code, e| 31*hash_code + e.hash_code())
                }
            }
        )*
    );
}

impl_for_iterable!(&[T]:[T: HashCode]);

// COLLECTIONS

impl_for_iterable! {
    Vec<T>:[T: HashCode]
    VecDeque<T>:[T: HashCode]
    LinkedList<T>:[T: HashCode]
    HashMap<K, V, H>:[K: HashCode, Eq, Hash; V: HashCode; H: BuildHasher]
    BTreeMap<K, V>:[K: HashCode, Eq; V: HashCode]
    HashSet<T, H>:[T: HashCode, Eq, Hash; H: BuildHasher]
    BTreeSet<T>:[T: HashCode]
    BinaryHeap<T>:[T: HashCode, Ord]
}
// COLLECTIONS END

impl HashCode for str {
    fn hash_code(&self) -> i32 {
        let n = self.chars().count() as u32;
        self.chars()
            .enumerate()
            .map(|(i, c)| (i as u32, c as i32))
            .map(|(i, c)| c.wrapping_mul(31i32.pow(n - i - 1)))
            .sum()
    }
}

#[cfg(windows)]
impl HashCode for OsStr {
    fn hash_code(&self) -> i32 {
        let n = self.encode_wide().count() as u32;
        self.encode_wide()
            .enumerate()
            .map(|(i, c)| (i as u32, i32::from(c)))
            .map(|(i, c)| c.wrapping_mul(31i32.pow(n - i - 1)))
            .sum()
    }
}
#[cfg(unix)]
impl HashCode for OsStr {
    fn hash_code(&self) -> i32 {
        self.as_bytes().hash_code()
    }
}

#[cfg(any(unix, windows))]
impl HashCode for Path {
    fn hash_code(&self) -> i32 {
        self.as_os_str().hash_code()
    }
}

macro_rules! impl_for_prim {
    ($($type:ty)*) => (
        $(
            impl HashCode for $type {
                fn hash_code(&self) -> i32 {
                    i32::from(*self)
                }
            }
        )*
    );
}

impl_for_prim! {
    i8 u8
    i16 u16
    i32
}

impl HashCode for bool {
    fn hash_code(&self) -> i32 {
        if *self {
            1231
        } else {
            1237
        }
    }
}

impl HashCode for char {
    fn hash_code(&self) -> i32 {
        *self as i32
    }
}

impl HashCode for u32 {
    fn hash_code(&self) -> i32 {
        *self as i32
    }
}

impl HashCode for f32 {
    fn hash_code(&self) -> i32 {
        self.to_bits() as i32
    }
}

impl HashCode for f64 {
    fn hash_code(&self) -> i32 {
        let v = self.to_bits();
        (v.pow((v >> 32) as u32)) as i32
    }
}

impl HashCode for i64 {
    fn hash_code(&self) -> i32 {
        self.pow((*self >> 32) as u32) as i32
    }
}
impl HashCode for u64 {
    fn hash_code(&self) -> i32 {
        self.pow((*self >> 32) as u32) as i32
    }
}

#[cfg(target_pointer_width = "64")]
impl HashCode for isize {
    fn hash_code(&self) -> i32 {
        (*self as u64).hash_code()
    }
}
#[cfg(target_pointer_width = "64")]
impl HashCode for usize {
    fn hash_code(&self) -> i32 {
        (*self as u64).hash_code()
    }
}
#[cfg(not(target_pointer_width = "64"))]
impl HashCode for isize {
    fn hash_code(&self) -> i32 {
        *self as i32
    }
}
#[cfg(not(target_pointer_width = "64"))]
impl HashCode for usize {
    fn hash_code(&self) -> i32 {
        *self as i32
    }
}
