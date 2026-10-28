/// A list of various units of information measurement:
/// bytes, kilobytes, megabytes, gigabytes, terabytes, petabytes.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum DataUnit {
    Bytes,
    KiloBytes,
    MegaBytes,
    GigaBytes,
    TeraBytes,
    PetaBytes,
}

impl DataUnit {
    /// It is called on function arguments of type [`DataUnit`]
    /// and returns a value in bytes.
    ///
    /// # Example
    /// Let's look at an example of converting a 100-gigabyte swap
    /// space and convert those 100 gigabytes into megabytes.
    /// see also [`convert_from_to`] function
    /// ```rust
    /// use persona_exporter_types::types::DataUnit;
    ///
    /// fn main() {
    ///     let swap = 100;
    ///
    ///     let from = DataUnit::GigaBytes.as_bytes();
    ///     let to = DataUnit::MegaBytes.as_bytes();
    ///
    ///     let total_swap = (swap as f64 * from) / to;
    ///
    ///     assert_eq!(from, 1024.0 * 1024.0 * 1024.0);
    ///     assert_eq!(to, 1024.0 * 1024.0);
    ///     assert_eq!(total_swap, 102400.0)
    /// }
    /// ```
    #[must_use]
    pub fn as_bytes(&self) -> f64 {
        match self {
            Self::Bytes => 1.0,
            Self::KiloBytes => 1024.0,
            Self::MegaBytes => 1024.0 * 1024.0,
            Self::GigaBytes => 1024.0 * 1024.0 * 1024.0,
            Self::TeraBytes => 1024.0 * 1024.0 * 1024.0 * 1024.0,
            Self::PetaBytes => 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
        }
    }
}

fn convert_from_to<T>(data: T, from: DataUnit, to: DataUnit) -> f64
where
    f64: From<T>
{
    let bytes = f64::from(data) * from.as_bytes();
    bytes / to.as_bytes()
}
/// Data conversion between different units of measurement
/// # Arguments
/// * `from` - We specify the unit of our input data.
///   This allows us to pass bytes, kilobytes, megabytes, etc.,
///   to the function.
/// * `to` - Specify the unit to which you want to convert the data.
/// # Example
/// ```rust
/// use persona_exporter_types::types::{DataUnit, ConvertTo};
///
/// fn main() {
///     let total_RAM: u64 = 16543600640;
///     let gigabytes_RAM = total_RAM
///         .to_dataunit(
///             DataUnit::Bytes,
///             DataUnit::GigaBytes
///         );
///
///     assert_eq!(gigabytes_RAM, 15.407428741455078);
/// }
/// ```
pub trait ConvertTo {
    #[must_use]
    fn to_dataunit(self, from: DataUnit, to: DataUnit) -> Self;
}

macro_rules! insert_convert_to_impl {
    ( $( $t:ty ),* ) => {
        $(
            impl ConvertTo for $t {
                #[doc = concat!["Converting a value of type **", stringify!($t), "** from one unit to another" ]]
                /// For more information about conversition, see [`ConvertTo`]
                #[inline]
                #[allow(clippy::cast_possible_truncation)]
                #[allow(clippy::cast_sign_loss)]
                fn to_dataunit(self, from: DataUnit, to: DataUnit) -> Self {
                    convert_from_to(self, from, to) as $t
                }
            }
        )*
    };
}

insert_convert_to_impl!(u8, u16, u32, i8, i16, i32, f32, f64);
