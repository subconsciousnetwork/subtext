use tendril::{SubtendrilError, Tendril};

pub fn cut<T>(tendril: &Tendril<T>, at: usize) -> Result<Tendril<T>, SubtendrilError>
where
    T: tendril::Format,
{
    tendril.try_subtendril(at as u32, tendril.len32() - at as u32)
}
