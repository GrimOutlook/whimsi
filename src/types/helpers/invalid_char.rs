use derive_more::Constructor;
use derive_more::Display;
use getset::Getters;

#[derive(Clone, Constructor, Debug, Display, Getters, PartialEq)]
#[display("{}", char)]
pub struct InvalidChar {
    char: char,
    index: usize,
}
