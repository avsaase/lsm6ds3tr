use core::fmt::{Display, Result};

#[allow(clippy::upper_case_acronyms)]
#[derive(Default)]
pub struct XYZ<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Display for XYZ<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result {
        write!(f, "X:{} Y:{} Z:{}", self.x, self.y, self.z)
    }
}

#[cfg(feature = "defmt")]
impl<T> defmt::Format for XYZ<T>
where
    T: defmt::Format,
{
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "X:{} Y:{} Z:{}", self.x, self.y, self.z)
    }
}
