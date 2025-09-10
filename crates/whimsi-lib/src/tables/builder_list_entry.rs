pub trait MsiBuilderListEntry {
    fn conflicts(&self, other: &Self) -> bool;
}
