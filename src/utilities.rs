pub fn bool_to_float<T>(boolean: bool) -> T
where
    T: num::Float,
{
    if boolean {
        return T::one();
    } else {
        return T::from(-1.0).unwrap();
    }
}
