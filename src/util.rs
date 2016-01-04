use num::traits::{PrimInt};

pub fn count_bit_errors<T>(x: T, y: T) -> u32
    where T: PrimInt
{
    T::count_ones(x ^ y)
}
