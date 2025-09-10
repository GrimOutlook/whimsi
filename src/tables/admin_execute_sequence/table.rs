use crate::{
    msi_list_boilerplate,
    tables::{
        builder_list::MsiBuilderList, generic_sequence::dao::GenericSequenceDao,
    },
};

pub struct AdminExecuteSequence {
    entries: Vec<GenericSequenceDao>,
}

msi_list_boilerplate!(AdminExecuteSequence, GenericSequenceDao);
