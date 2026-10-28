use crate::metrics::traits::Clear;
use compact_str::CompactString;

/// The component information primarily consists of data regarding the temperatures of
/// individual components (circuit boards, processor cores, etc.).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct ComponentListInfo {
    /// Components count
    pub count: usize,
    /// Checks whether the component field is empty.
    pub is_empty: bool,
    /// Components info, see [`ComponentInfo`]
    pub components: Vec<ComponentInfo>,
}

/// Processor thread information, used in [`ComponentListInfo`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct ComponentInfo {
    /// Component identifier recognized by the system kernel
    pub id: CompactString,
    /// Component name
    pub name: CompactString,
    /// Component temp
    pub temp: f32,
    /// Critical temp
    pub critical_temp: f32,
    /// Max temp of component
    pub max_temp: f32,
}

impl Clear for ComponentListInfo {
    fn clear_dynamic(&mut self) {
        self.components.clear();

        // self.is_empty = true;
        // self.count = 0;
    }
}
