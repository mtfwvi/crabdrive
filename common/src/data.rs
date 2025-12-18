use std::fmt;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub const KB: u64 = 1_000;
pub const MB: u64 = KB * 1_000;
pub const GB: u64 = MB * 1_000;
pub const TB: u64 = GB * 1_000;

pub const KIB: u64 = 1_024;
pub const MIB: u64 = KIB * 1_024;
pub const GIB: u64 = MIB * 1_024;
pub const TIB: u64 = GIB * 1_024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataUnit {
    Byte,
    Kilobyte,
    Megabyte,
    Gigabyte,
    Terabyte,
    Kibibyte,
    Mebibyte,
    Gibibyte,
    Tebibyte,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct DataAmount(u64);

impl DataAmount {
    pub fn new(amount: f64, unit: DataUnit) -> Self {
        let unit_factor = match unit {
            DataUnit::Byte => 1,
            DataUnit::Kilobyte => KB,
            DataUnit::Megabyte => MB,
            DataUnit::Gigabyte => GB,
            DataUnit::Terabyte => TB,
            DataUnit::Kibibyte => KIB,
            DataUnit::Mebibyte => MIB,
            DataUnit::Gibibyte => GIB,
            DataUnit::Tebibyte => TIB,
        } as f64;

        DataAmount((amount * unit_factor) as u64)
    }

    pub fn as_bytes(&self) -> u64 {
        self.0
    }

    pub fn as_kb(&self) -> f64 {
        self.0 as f64 / KB as f64
    }
    pub fn as_mb(&self) -> f64 {
        self.0 as f64 / MB as f64
    }
    pub fn as_gb(&self) -> f64 {
        self.0 as f64 / GB as f64
    }
    pub fn as_tb(&self) -> f64 {
        self.0 as f64 / TB as f64
    }

    pub fn as_kib(&self) -> f64 {
        self.0 as f64 / KIB as f64
    }
    pub fn as_mib(&self) -> f64 {
        self.0 as f64 / MIB as f64
    }
    pub fn as_gib(&self) -> f64 {
        self.0 as f64 / GIB as f64
    }
    pub fn as_tib(&self) -> f64 {
        self.0 as f64 / TIB as f64
    }

    pub fn unit_floor(&self) -> DataUnit {
        if self.0 >= TB {
            DataUnit::Terabyte
        } else if self.0 >= GB {
            DataUnit::Gigabyte
        } else if self.0 >= MB {
            DataUnit::Megabyte
        } else if self.0 >= KB {
            DataUnit::Kilobyte
        } else {
            DataUnit::Byte
        }
    }
}

impl Add for DataAmount {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        DataAmount(self.0.saturating_add(other.0))
    }
}

impl AddAssign for DataAmount {
    fn add_assign(&mut self, other: Self) {
        self.0 = self.0.saturating_add(other.0);
    }
}

impl Sub for DataAmount {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        DataAmount(self.0.saturating_sub(other.0))
    }
}

impl SubAssign for DataAmount {
    fn sub_assign(&mut self, other: Self) {
        self.0 = self.0.saturating_sub(other.0);
    }
}

impl Sum for DataAmount {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(DataAmount(0), |a, b| a + b)
    }
}

impl fmt::Display for DataAmount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bytes = self.0 as f64;
        if self.0 >= TB {
            write!(f, "{:.2} TB ({bytes} Bytes)", bytes / TB as f64)
        } else if self.0 >= GB {
            write!(f, "{:.2} GB ({bytes} Bytes)", bytes / GB as f64)
        } else if self.0 >= MB {
            write!(f, "{:.2} MB ({bytes} Bytes)", bytes / MB as f64)
        } else if self.0 >= KB {
            write!(f, "{:.2} KB ({bytes} Bytes)", bytes / KB as f64)
        } else if self.0 == 1 {
            write!(f, "1 Byte")
        } else {
            write!(f, "{} Bytes", self.0)
        }
    }
}

/// Create a data amount according to IEC 60027-2
///
/// Usage:
/// ```
/// use crabdrive_common::data::{DataUnit, DataAmount};
/// use crabdrive_common::da;
///
/// let one_kilobyte = da!(1000);
/// one_kilobyte.as_kb(); // 1
/// println!("{}", one_kilobyte); // 1.00 KB (1000 Bytes)
///
/// let five_hundred_megabytes = da!(500 MB);
/// five_hundred_megabytes.as_bytes(); // 500_000_000
/// println!("{}", five_hundred_megabytes) // 500.00 MB (500000000 Bytes)
/// ```
///
/// Supported units:
/// - `KB` (1000) and `KiB` (1024)
/// - `MB` (1,000,000) and `MiB` (1,048,576)
/// - `GB` (1,000,000,000) and `GiB` (1,073,741,824)
/// - `TB` (1,000,000,000,000) and `TiB` (1,099,511,627,776)
///
/// Units are **case-sensitive**.
#[macro_export]
macro_rules! da {
    ($val:expr) => {
        DataAmount::new($val as f64, DataUnit::Byte)
    };

    ($val:literal B) => {
        DataAmount::new($val as f64, DataUnit::Byte)
    };
    ($val:literal KB) => {
        DataAmount::new($val as f64, DataUnit::Kilobyte)
    };
    ($val:literal MB) => {
        DataAmount::new($val as f64, DataUnit::Megabyte)
    };
    ($val:literal GB) => {
        DataAmount::new($val as f64, DataUnit::Gigabyte)
    };
    ($val:literal TB) => {
        DataAmount::new($val as f64, DataUnit::Terabyte)
    };

    ($val:literal KiB) => {
        DataAmount::new($val as f64, DataUnit::Kibibyte)
    };
    ($val:literal MiB) => {
        DataAmount::new($val as f64, DataUnit::Mebibyte)
    };
    ($val:literal GiB) => {
        DataAmount::new($val as f64, DataUnit::Gibibyte)
    };
    ($val:literal TiB) => {
        DataAmount::new($val as f64, DataUnit::Tebibyte)
    };
}

#[cfg(test)]
mod tests {
    use crate::data::{DataAmount, DataUnit};
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    #[test]
    fn test_macro() {
        assert_eq!(da!(1000).as_bytes(), 1000);
        assert_eq!(da!(1000).as_kb(), 1.0);
        assert_eq!(da!(1000).as_mb(), 0.001);
        assert_eq!(da!(1000).as_gb(), 1e-6);

        assert_eq!(da!(1.024 KB).as_bytes(), 1024);
        assert_eq!(da!(1.024 KB).as_kb(), 1.024);
        assert_eq!(da!(1.024 KB).as_kib(), 1.0);

        assert_eq!(da!(1.024 KB), da!(1 KiB));
    }

    #[test_case("0 Bytes", 0 ; "for zero")]
    #[test_case("1 Byte", 1 ; "for single Byte")]
    #[test_case("100.00 KB (100000 Bytes)", 100_000 ; "for one hundred kilobytes")]
    #[test_case("1.00 MB (1000000 Bytes)", 1_000_000 ; "for one megabyte")]
    #[test_case("1.02 KB (1024 Bytes)", 1_024 ; "for one kibibyte")]
    fn test_display(expected: &str, for_bytes: u64) {
        let actual = format!("{}", da!(for_bytes));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_arithmetic() {
        let mut da1 = da!(10 KB);
        let mut da2 = da!(20 KB);
        assert_eq!(da1 + da2, da!(30 KB));
        assert_eq!(da1 - da2, da!(0));
        assert_eq!(da!(10 GB) - da!(5 GB), da!(5 GB));
        da1 -= da!(1 GB);
        da2 -= da!(5 KB);
        assert_eq!(da1, da!(0));
        assert_eq!(da2, da!(15 KB));
    }
}
