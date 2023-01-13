/*****************************************************************************
*                               COMMON
******************************************************************************/

// Source: https://stackoverflow.com/questions/38811387/how-to-implement-idiomatic-operator-overloading-for-values-and-references-in-rus/38815035#38815035
macro_rules! forward_ref_binop {
    (impl $imp:ident, $method:ident for $t:ident, $u:ident) => {
        impl<'a, N: $imp<N, Output = N> + Copy> $imp<$u<N>> for &'a $t<N> {
            type Output = <$t<N> as $imp<$u<N>>>::Output;

            #[inline]
            fn $method(self, other: $u<N>) -> <$t<N> as $imp<$u<N>>>::Output {
                $imp::$method(*self, other)
            }
        }

        impl<'a, N: $imp<N, Output = N> + Copy> $imp<&'a $u<N>> for $t<N> {
            type Output = <$t<N> as $imp<$u<N>>>::Output;

            #[inline]
            fn $method(self, other: &'a $u<N>) -> <$t<N> as $imp<$u<N>>>::Output {
                $imp::$method(self, *other)
            }
        }

        impl<'a, 'b, N: $imp<N, Output = N> + Copy> $imp<&'a $u<N>> for &'b $t<N> {
            type Output = <$t<N> as $imp<$u<N>>>::Output;

            #[inline]
            fn $method(self, other: &'a $u<N>) -> <$t<N> as $imp<$u<N>>>::Output {
                $imp::$method(*self, *other)
            }
        }
    }
}
pub(crate) use forward_ref_binop;

macro_rules! assign_ref_binop {
    (impl $imp:ident, $method:ident for $t:ident, $u:ident) => {
        impl<'a, N: $imp<N> + Copy> $imp<$u<N>> for &'a mut $t<N> {
            #[inline]
            fn $method(&mut self, other: $u<N>) {
                $imp::$method(*self, other)
            }
        }

        impl<'a, N: $imp<N> + Copy> $imp<&'a $u<N>> for $t<N> {
            #[inline]
            fn $method(&mut self, other: &'a $u<N>) {
                $imp::$method(self, *other)
            }
        }

        impl<'a, 'b, N: $imp<N> + Copy> $imp<&'a $u<N>> for &'b mut $t<N> {
            #[inline]
            fn $method(&mut self, other: &'a $u<N>) {
                $imp::$method(*self, *other)
            }
        }
    }
}
pub(crate) use assign_ref_binop;

macro_rules! scalar_forward_ref_binop {
    (impl $imp:ident, $method:ident for $t:ident, $u:ident) => {
        impl<'a, N: $imp<N, Output = N> + Copy> $imp<N> for &'a $t<N> {
            type Output = <$t<N> as $imp<$u<N>>>::Output;

            #[inline]
            fn $method(self, other: N) -> <$t<N> as $imp<$u<N>>>::Output {
                $imp::$method(*self, other)
            }
        }

        impl<'a, N: $imp<N, Output = N> + Copy> $imp<&'a N> for $t<N> {
            type Output = <$t<N> as $imp<$u<N>>>::Output;

            #[inline]
            fn $method(self, other: &'a N) -> <$t<N> as $imp<$u<N>>>::Output {
                $imp::$method(self, *other)
            }
        }

        impl<'a, 'b, N: $imp<N, Output = N> + Copy> $imp<&'a N> for &'b $t<N> {
            type Output = <$t<N> as $imp<$u<N>>>::Output;

            #[inline]
            fn $method(self, other: &'a N) -> <$t<N> as $imp<$u<N>>>::Output {
                $imp::$method(*self, *other)
            }
        }
    }
}
pub(crate) use scalar_forward_ref_binop;

macro_rules! scalar_assign_ref_binop {
    (impl $imp:ident, $method:ident for $t:ident, $u:ident) => {
        impl<'a, N: $imp<N> + Copy> $imp<N> for &'a mut $t<N> {
            #[inline]
            fn $method(&mut self, other: N) {
                $imp::$method(*self, other)
            }
        }

        impl<'a, N: $imp<N> + Copy> $imp<&'a N> for $t<N> {
            #[inline]
            fn $method(&mut self, other: &'a N) {
                $imp::$method(self, *other)
            }
        }

        impl<'a, 'b, N: $imp<N> + Copy> $imp<&'a N> for &'b mut $t<N> {
            #[inline]
            fn $method(&mut self, other: &'a N) {
                $imp::$method(self, *other)
            }
        }
    }
}
pub(crate) use scalar_assign_ref_binop;

/*****************************************************************************
*                               CONSTRUCTION
******************************************************************************/

macro_rules! default_impl(
    ($t: ident, $($compN: ident),+) => (
        impl<N> Default for $t<N> where N : Default {
            #[inline]
            fn default() -> $t<N> {
                $t {
                    $($compN: N::default() ),+
                }
            }
        }
    );
);
pub(crate) use default_impl;

#[macro_export]
macro_rules! new_impl(
    ($t: ident, $($compN: ident),+) => (
        impl<N> $t<N> {
            #[inline]
            pub fn new($($compN: N ),+) -> $t<N> {
                $t {
                    $($compN: $compN ),+
                }
            }
        }
    );
);
pub(crate) use new_impl;

/*****************************************************************************
*                               MODIFIERS
******************************************************************************/

macro_rules! clear_impl(
    ($t: ident, $($compN: ident),+) => (
        impl<N: Default> Clear for $t<N> {
            #[inline]
            fn clear(&mut self) {
                $( self.$compN = N::default(); )+
            }
        }
    )
);
pub(crate) use clear_impl;

macro_rules! rand_impl(
    ($t: ident, $($compN: ident),*) => (
        impl<N: Rand> Rand for $t<N> {
            #[inline]
            fn rand<R: Rng>(rng: &mut R) -> $t<N> {
                $t { $($compN: Rand::rand(rng), )* }
            }
        }

        impl<N: Rand> $t<N> {
            #[inline]
            fn randomize<R: Rng>(&mut self, rng: &mut R) {
                $( self.$compN = Rand::rand(rng); )+
            }
        }
    )
);
pub(crate) use rand_impl;

/*****************************************************************************
*                               CONVERSION
******************************************************************************/

macro_rules! conversion_impl(
    ($t: ident, $dimension: expr) => (
        impl<N> AsRef<[N; $dimension]> for $t<N> {
            #[inline]
            fn as_ref(&self) -> &[N; $dimension] {
                unsafe {
                    mem::transmute(self)
                }
            }
        }

        impl<N> AsMut<[N; $dimension]> for $t<N> {
            #[inline]
            fn as_mut(&mut self) -> &mut [N; $dimension] {
                unsafe {
                    mem::transmute(self)
                }
            }
        }

        impl<'a, N> From<&'a [N; $dimension]> for &'a $t<N> {
            #[inline]
            fn from(arr: &'a [N; $dimension]) -> &'a $t<N> {
                unsafe {
                    mem::transmute(arr)
                }
            }
        }

        impl<'a, N> From<&'a mut [N; $dimension]> for &'a mut $t<N> {
            #[inline]
            fn from(arr: &'a mut [N; $dimension]) -> &'a mut $t<N> {
                unsafe {
                    mem::transmute(arr)
                }
            }
        }

        impl<'a, N: Clone> From<&'a [N; $dimension]> for $t<N> {
            #[inline]
            fn from(arr: &'a [N; $dimension]) -> $t<N> {
                let vref: &$t<N> = From::from(arr);
                vref.clone()
            }
        }
    )
);
pub(crate) use conversion_impl;

/*****************************************************************************
*                               INDEXING
******************************************************************************/

macro_rules! index_impl(
    ($t: ident) => (
        impl<N, T> Index<T> for $t<N> where [N]: Index<T> {
            type Output = <[N] as Index<T>>::Output;

            fn index(&self, i: T) -> &<[N] as Index<T>>::Output {
                &self.as_ref()[i]
            }
        }

        impl<N, T> IndexMut<T> for $t<N> where [N]: IndexMut<T> {
            fn index_mut(&mut self, i: T) -> &mut <[N] as Index<T>>::Output {
                &mut self.as_mut()[i]
            }
        }
    )
);
pub(crate) use index_impl;

/*****************************************************************************
*                               OPERATORS
******************************************************************************/

macro_rules! add_impl(
    ($t: ident, $($compN: ident),+) => (
        impl<N: Add<N, Output = N>> Add<$t<N>> for $t<N> {
            type Output = $t<N>;

            #[inline]
            fn add(self, right: $t<N>) -> $t<N> {
                $t::new($(self.$compN + right.$compN),+)
            }
        }

        forward_ref_binop!(impl Add, add for $t, $t);

        impl<N: AddAssign<N>> AddAssign<$t<N>> for $t<N> {
            #[inline]
            fn add_assign(&mut self, right: $t<N>) {
                $( self.$compN += right.$compN; )+
            }
        }

        assign_ref_binop!(impl AddAssign, add_assign for $t, $t);
    )
);
pub(crate) use add_impl;

macro_rules! scalar_add_impl(
    ($t: ident, $($compN: ident),+) => (
        impl<N: Copy + Add<N, Output = N>> Add<N> for $t<N> {
            type Output = $t<N>;

            #[inline]
            fn add(self, right: N) -> $t<N> {
                $t::new($(self.$compN + right),+)
            }
        }

        scalar_forward_ref_binop!(impl Add, add for $t, $t);

        impl<N: Copy + AddAssign<N>> AddAssign<N> for $t<N> {
            #[inline]
            fn add_assign(&mut self, right: N) {
                $( self.$compN += right; )+
            }
        }

        scalar_assign_ref_binop!(impl AddAssign, add_assign for $t, $t);
    )
);
pub(crate) use scalar_add_impl;

macro_rules! sub_impl(
    ($t: ident, $($compN: ident),+) => (
        impl<N: Sub<N, Output = N>> Sub<$t<N>> for $t<N> {
            type Output = $t<N>;

            #[inline]
            fn sub(self, right: $t<N>) -> $t<N> {
                $t::new($(self.$compN - right.$compN),+)
            }
        }

        forward_ref_binop!(impl Sub, sub for $t, $t);

        impl<N: SubAssign<N>> SubAssign<$t<N>> for $t<N> {
            #[inline]
            fn sub_assign(&mut self, right: $t<N>) {
                $( self.$compN -= right.$compN; )+
            }
        }

        assign_ref_binop!(impl SubAssign, sub_assign for $t, $t);
    )
);
pub(crate) use sub_impl;

macro_rules! scalar_sub_impl(
    ($t: ident, $($compN: ident),+) => (
        impl<N: Copy + Sub<N, Output = N>> Sub<N> for $t<N> {
            type Output = $t<N>;

            #[inline]
            fn sub(self, right: N) -> $t<N> {
                $t::new($(self.$compN - right),+)
            }
        }

        scalar_forward_ref_binop!(impl Sub, sub for $t, $t);

        impl<N: Copy + SubAssign<N>> SubAssign<N> for $t<N> {
            #[inline]
            fn sub_assign(&mut self, right: N) {
                $( self.$compN -= right; )+
            }
        }

        scalar_assign_ref_binop!(impl SubAssign, sub_assign for $t, $t);

    )
);
pub(crate) use scalar_sub_impl;

macro_rules! mul_impl(
    ($t: ident, $($compN: ident),+) => (
        impl<N: Mul<N, Output = N>> Mul<$t<N>> for $t<N> {
            type Output = $t<N>;

            #[inline]
            fn mul(self, right: $t<N>) -> $t<N> {
                $t::new($(self.$compN * right.$compN),+)
            }
        }

        forward_ref_binop!(impl Mul, mul for $t, $t);

        impl<N: MulAssign<N>> MulAssign<$t<N>> for $t<N> {
            #[inline]
            fn mul_assign(&mut self, right: $t<N>) {
                $( self.$compN *= right.$compN; )+
            }
        }

        assign_ref_binop!(impl MulAssign, mul_assign for $t, $t);
    )
);
pub(crate) use mul_impl;

macro_rules! scalar_mul_impl(
    ($t: ident, $($compN: ident),+) => (
        impl<N: Copy + Mul<N, Output = N>> Mul<N> for $t<N> {
            type Output = $t<N>;

            #[inline]
            fn mul(self, right: N) -> $t<N> {
                $t::new($(self.$compN * right),+)
            }
        }

        scalar_forward_ref_binop!(impl Mul, mul for $t, $t);

        impl<N: Copy + MulAssign<N>> MulAssign<N> for $t<N> {
            #[inline]
            fn mul_assign(&mut self, right: N) {
                $( self.$compN *= right; )+
            }
        }

        scalar_assign_ref_binop!(impl MulAssign, mul_assign for $t, $t);
    )
);
pub(crate) use scalar_mul_impl;

macro_rules! div_impl(
    ($t: ident, $($compN: ident),+) => (
        impl<N: Div<N, Output = N>> Div<$t<N>> for $t<N> {
            type Output = $t<N>;

            #[inline]
            fn div(self, right: $t<N>) -> $t<N> {
                $t::new($(self.$compN / right.$compN),+)
            }
        }

        forward_ref_binop!(impl Div, div for $t, $t);

        impl<N: DivAssign<N>> DivAssign<$t<N>> for $t<N> {
            #[inline]
            fn div_assign(&mut self, right: $t<N>) {
                $( self.$compN /= right.$compN; )+
            }
        }

        assign_ref_binop!(impl DivAssign, div_assign for $t, $t);
    )
);
pub(crate) use div_impl;

macro_rules! scalar_div_impl(
    ($t: ident, $($compN: ident),+) => (
        impl<N: Copy + Div<N, Output = N>> Div<N> for $t<N> {
            type Output = $t<N>;

            #[inline]
            fn div(self, right: N) -> $t<N> {
                $t::new($(self.$compN / right),+)
            }
        }

        scalar_forward_ref_binop!(impl Div, div for $t, $t);

        impl<N: Copy + DivAssign<N>> DivAssign<N> for $t<N> {
            #[inline]
            fn div_assign(&mut self, right: N) {
                $( self.$compN /= right; )+
            }
        }

        scalar_assign_ref_binop!(impl DivAssign, div_assign for $t, $t);
    )
);
pub(crate) use scalar_div_impl;

macro_rules! neg_impl(
    ($t: ident, $($compN: ident),+) => (
        impl<N: Neg<Output = N> + Copy> Neg for $t<N> {
            type Output = $t<N>;

            #[inline]
            fn neg(self) -> $t<N> {
                $t::new($(-self.$compN ),+)
            }
        }

        impl<N: Neg<Output = N> + Copy> Neg for &$t<N> {
            type Output = $t<N>;

            #[inline]
            fn neg(self) -> $t<N> {
                $t::new($(-self.$compN ),+)
            }
        }

        impl<N: Neg<Output = N> + Copy> Neg for &mut $t<N> {
            type Output = $t<N>;

            #[inline]
            fn neg(self) -> $t<N> {
                $t::new($(-self.$compN ),+)
            }
        }
    )
);
pub(crate) use neg_impl;

/*****************************************************************************
*                               FUNCS
******************************************************************************/

macro_rules! magnitude_impl(
    ($t: ident, $($compN: ident),+) => (
        impl<N: Float + Default + AddAssign<N>> Magnitude<N> for $t<N> {
            #[inline]
            fn magnitude(&self) -> N {
                let mut r: N = N::default();
                $( r += self.$compN * self.$compN; )+
                r.sqrt()
            }

            #[inline]
            fn magnitude_sqr(&self) -> N {
                let mut r: N = N::default();
                $( r += self.$compN * self.$compN; )+
                r
            }
        }
    )
);
pub(crate) use magnitude_impl;

macro_rules! dot_impl(
    ($t: ident, $($compN: ident),+) => (
        impl<N: Float + Default + AddAssign<N>> Dot<$t<N>, N> for $t<N> {
            #[inline]
            fn dot(&self, other: $t<N>) -> N {
                let mut r: N = N::default();
                $( r += self.$compN * other.$compN; )+
                r
            }
        }

        impl<N: Float + Default + AddAssign<N>> Dot<&mut $t<N>, N> for $t<N> {
            #[inline]
            fn dot(&self, other: &mut $t<N>) -> N {
                let mut r: N = N::default();
                $( r += self.$compN * other.$compN; )+
                r
            }
        }
    )
);
pub(crate) use dot_impl;

macro_rules! cross_impl(
    ($t: ident, $($compN: ident),+) => (
        impl<N: Float + Default + AddAssign<N>> Cross<$t<N>, $t<N>> for $t<N> {
            #[inline]
            fn cross(&self, other: $t<N>) -> $t<N> {
                $t::<N>::new(self.y * other.z - self.z * other.y, self.z * other.x - self.x * other.z, self.x * other.y - self.y * other.x)
            }
        }

        impl<N: Float + Default + AddAssign<N>> Cross<&mut $t<N>, $t<N>> for $t<N> {
            #[inline]
            fn cross(&self, other: &mut $t<N>) -> $t<N> {
                $t::<N>::new(self.y * other.z - self.z * other.y, self.z * other.x - self.x * other.z, self.x * other.y - self.y * other.x)
            }
        }
    )
);
pub(crate) use cross_impl;

macro_rules! normalize_impl(
    ($t: ident, $($compN: ident),+) => (
        impl<N: Float + Default + AddAssign<N> + DivAssign<N>> Normalize<$t<N>> for $t<N> {
            #[inline]
            fn normalize(&mut self) {
                let m = self.magnitude();
                if m != N::default() {
                    *self /= m;
                }
            }

            #[inline]
            fn normalized(&self) -> $t<N> {
                let clone = self.clone();

                let m = clone.magnitude();
                if m != N::default() {
                    clone / m
                } else {
                    clone
                }
            }
        }
    )
);
pub(crate) use normalize_impl;

macro_rules! distance_impl(
    ($t: ident, $($compN: ident),+) => (
        impl<N: Float + Default + AddAssign<N>> Distance<$t<N>, N> for $t<N> {
            #[inline]
            fn distance(&self, other: $t<N>) -> N {
                let mut d: N = N::default();
                $( d += (self.$compN - other.$compN) * (self.$compN - other.$compN); )+
                d.sqrt()
            }
        }

        impl<N: Float + Default + AddAssign<N>> Distance<&mut $t<N>, N> for $t<N> {
            #[inline]
            fn distance(&self, other: &mut $t<N>) -> N {
                let mut d: N = N::default();
                $( d += (self.$compN - other.$compN) * (self.$compN - other.$compN); )+
                d.sqrt()
            }
        }
    )
);
pub(crate) use distance_impl;

macro_rules! lerp_impl(
    ($t: ident, $($compN: ident),+) => (
        impl<N: Float + Default + AddAssign<N>> Lerp<$t<N>, $t<N>, N> for $t<N> {
            #[inline]
            fn lerp(&self, other: $t<N>, t: N) -> $t<N> {
                $t {
                    $($compN: self.$compN + (other.$compN - self.$compN) * t ),+
                }
            }
        }

        impl<N: Float + Default + AddAssign<N>> Lerp<&mut $t<N>, $t<N>, N> for $t<N> {
            #[inline]
            fn lerp(&self, other: &mut $t<N>, t: N) -> $t<N> {
                $t {
                    $($compN: self.$compN + (other.$compN - self.$compN) * t ),+
                }
            }
        }
    )
);
pub(crate) use lerp_impl;