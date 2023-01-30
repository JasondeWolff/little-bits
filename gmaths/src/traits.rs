pub trait Clear {
    fn clear(&mut self);
}

pub trait Magnitude<R> {
    fn magnitude(&self) -> R;
    fn magnitude_sqr(&self) -> R;
}

pub trait Dot<V, R> {
    fn dot(&self, other: V) -> R;
}

pub trait Cross<V, R> {
    fn cross(&self, other: V) -> R;
}

pub trait Normalize<R> {
    fn normalize(&mut self);
    fn normalized(&self) -> R;
}

pub trait Distance<V, R> {
    fn distance(&self, b: V) -> R;
}

pub trait Lerp<V, R, N> {
    fn lerp(&self, b: V, t: N) -> R;
}

pub trait Min<V, R> {
    fn min(&self, b: V) -> R;
}

pub trait Max<V, R> {
    fn max(&self, b: V) -> R;
}