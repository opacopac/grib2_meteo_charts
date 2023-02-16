pub trait GridValueType: Copy + PartialEq + Send + Sync {
}

impl<T> GridValueType for T
    where T: Copy + PartialEq + Send + Sync {
}
