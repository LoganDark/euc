pub trait Lerp<F = f32> {
    fn lerp_unchecked(a: &Self, b: &Self, factor: &F) -> Self;
}

impl Lerp<f32> for f32 {
    #[inline(always)]
    fn lerp_unchecked(a: &Self, b: &Self, factor: &f32) -> Self { factor.mul_add(*b - *a, *a) }
}
impl Lerp<f64> for f64 {
    #[inline(always)]
    fn lerp_unchecked(a: &Self, b: &Self, factor: &f64) -> Self { factor.mul_add(*b - *a, *a) }
}

impl<T, F, const N: usize> Lerp<F> for [T; N]
    where T: Lerp<F> + Copy
{
    #[inline(always)]
    fn lerp_unchecked(a: &Self, b: &Self, factor: &F) -> Self {
        let mut out = *a;
        (0..N).for_each(|i| out[i] = Lerp::lerp_unchecked(&out[i], &b[i], factor));
        out
    }
}

pub trait Clamp {
    fn clamp(&self, min: &Self, max: &Self) -> Self;
}

impl Clamp for f32 {
    #[inline(always)]
    fn clamp(&self, min: &f32, max: &f32) -> Self { self.max(*min).min(*max) }
}

impl Clamp for f64 {
    #[inline(always)]
    fn clamp(&self, min: &f64, max: &f64) -> Self { self.max(*min).min(*max) }
}

impl<T, const N: usize> Clamp for [T; N]
    where T: Clamp + Copy
{
    #[inline(always)]
    fn clamp(&self, min: &Self, max: &Self) -> Self {
        let mut out = *self;
        (0..N).for_each(|i| out[i] = out[i].clamp(&min[i], &max[i]));
        out
    }
}

/// Truncation of positive values to integers
pub trait Truncate<T> {
    fn truncate(self) -> T;
    fn detruncate(x: T) -> Self;
}

impl Truncate<u16> for f32 { fn truncate(self) -> u16 { self as u16 } fn detruncate(x: u16) -> Self { x as f32 } }
impl Truncate<u32> for f32 { fn truncate(self) -> u32 { self as u32 } fn detruncate(x: u32) -> Self { x as f32 } }
impl Truncate<u64> for f32 { fn truncate(self) -> u64 { self as u64 } fn detruncate(x: u64) -> Self { x as f32 } }
impl Truncate<usize> for f32 { fn truncate(self) -> usize { self as usize } fn detruncate(x: usize) -> Self { x as f32 } }

impl Truncate<u16> for f64 { fn truncate(self) -> u16 { self as u16 } fn detruncate(x: u16) -> Self { x as f64 } }
impl Truncate<u32> for f64 { fn truncate(self) -> u32 { self as u32 } fn detruncate(x: u32) -> Self { x as f64 } }
impl Truncate<u64> for f64 { fn truncate(self) -> u64 { self as u64 } fn detruncate(x: u64) -> Self { x as f64 } }
impl Truncate<usize> for f64 { fn truncate(self) -> usize { self as usize } fn detruncate(x: usize) -> Self { x as f64 } }
