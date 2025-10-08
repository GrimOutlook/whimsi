pub(crate) trait MsiDao {
    fn to_row(&self) -> Vec<msi::Value>;
    fn conflicts_with(&self, other: &Self) -> bool;
}
