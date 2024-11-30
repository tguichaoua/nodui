/// A push-only vec.
pub(crate) struct Collector<T>(Vec<T>);

impl<T> Collector<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn into_vec(self) -> Vec<T> {
        self.0
    }

    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    pub fn watch<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> (&[T], R) {
        let start = self.0.len();
        let inner = f(self);
        let end = self.0.len();

        assert!(
            start <= end,
            "some items has been removed from the Collector"
        );

        #[allow(clippy::indexing_slicing)]
        let new_items = &self.0[start..end];

        (new_items, inner)
    }
}
