use crate::types::column::integer::Integer;

#[derive(Clone, Debug, PartialEq)]
pub struct Media {
    id: Option<Integer>,
    last_sequence: Integer,
}
