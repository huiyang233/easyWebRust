///
///方便Vec ToVo
///

pub trait FromVo<V> {
    fn from_vo(arg: Vec<V>) -> Self;
}

// 由于trait已经约束了泛型的用法，impl中不需要重复约束T: From<V>。
impl<T, V> FromVo<V> for Vec<T>
    where
        T: From<V>,
{
    fn from_vo(arg: Vec<V>) -> Self {
        arg.into_iter().map(T::from).collect()
    }
}