use crate::tables::media::dao::MediaDao;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MediaTable(Vec<MediaDao>);
