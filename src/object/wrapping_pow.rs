pub(super) trait WrapPow {
    fn wrap_pow(self, exp: u32) -> Self;
}

macro_rules! impl_for_prim {
    ($($type:ty)*) => (
        $(
            impl WrapPow for $type {
                /// Stolen from std since it's currenlty unstable
                fn wrap_pow(self, mut exp: u32) -> Self {
                    let mut base = self;
                    let mut acc: Self = 1;

                    while exp > 1 {
                        if (exp & 1) == 1 {
                            acc = acc.wrapping_mul(base);
                        }
                        exp /= 2;
                        base = base.wrapping_mul(base);
                    }

                    // Deal with the final bit of the exponent separately, since
                    // squaring the base afterwards is not necessary and may cause a
                    // needless overflow.
                    if exp == 1 {
                        acc = acc.wrapping_mul(base);
                    }

                    acc
                }
            }
        )*
    );
}

impl_for_prim!{
    i32 u32 i64 u64
}
