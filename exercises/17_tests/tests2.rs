// Calculates the power of 2 using a bit shift.
// `1 << n` is equivalent to "2 to the power of n".
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // use super::*;
    use crate::power_of_2 ;

    #[test]
    fn you_can_assert_eq() {
        // TODO: Test the function `power_of_2` with some values.
        assert_eq!(16 , power_of_2(4));
        assert_eq!(1 , power_of_2(0));
        assert_eq!(65536 , power_of_2(16));
        assert_eq!(8 , power_of_2(3));
    }
}
