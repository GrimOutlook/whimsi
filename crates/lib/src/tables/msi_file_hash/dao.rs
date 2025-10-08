use std::io::Read;
use std::path::PathBuf;

use md5::Digest;
use md5::Md5;
use md5::digest::generic_array::GenericArray;
use tracing::debug;

use crate::tables::builder_list_entry::MsiBuilderListEntry;
use crate::tables::dao::IsDao;
use crate::tables::file::table::FileIdentifier;
use crate::tables::file::{self};
use crate::types::column::identifier::Identifier;
use crate::types::helpers::to_unique_msi_identifier::ToUniqueMsiIdentifier;

#[derive(Debug, Clone, PartialEq)]
pub struct MsiFileHashDao {
    file: FileIdentifier,
    options: i16,
    hash_part_1: i32,
    hash_part_2: i32,
    hash_part_3: i32,
    hash_part_4: i32,
}

impl MsiBuilderListEntry for MsiFileHashDao {
    fn conflicts(&self, other: &Self) -> bool {
        self.file == other.file
    }
}

impl ToUniqueMsiIdentifier for MsiFileHashDao {
    fn to_unique_msi_identifier(&self) -> Option<Identifier> {
        None
    }
}

impl IsDao for MsiFileHashDao {
    fn to_row(&self) -> Vec<msi::Value> {
        vec![
            self.file.clone().into(),
            self.options.into(),
            self.hash_part_1.into(),
            self.hash_part_2.into(),
            self.hash_part_3.into(),
            self.hash_part_4.into(),
        ]
    }
}

impl MsiFileHashDao {
    pub fn from_path(
        file_id: FileIdentifier,
        path: &PathBuf,
    ) -> anyhow::Result<Self> {
        let (hash_part_1, hash_part_2, hash_part_3, hash_part_4) =
            Self::get_msi_file_hash_parts(path)?;
        Ok(Self {
            file: file_id,
            // NOTE: This option is currently always set to 0 as it is reserved
            // for future use.
            options: 0,
            hash_part_1,
            hash_part_2,
            hash_part_3,
            hash_part_4,
        })
    }

    /// This implementation is shamelessly stolen from `msitools` in `utils.vala`.
    ///
    /// https://gitlab.gnome.org/GNOME/msitools/-/blob/master/tools/wixl/util.vala?ref_type=heads#L151
    fn get_msi_file_hash_parts(
        path: &PathBuf,
    ) -> anyhow::Result<(i32, i32, i32, i32)> {
        // Open the file
        let mut file = std::fs::File::open(path)?;

        // Create an MD5 hasher instance
        let mut hasher = Md5::new();

        // Create a buffer to read chunks of the file
        let mut buffer = [0; 1024];

        loop {
            // Read a chunk from the file
            let bytes_read = file.read(&mut buffer)?;

            // If no bytes were read, we've reached the end of the file
            if bytes_read == 0 {
                break;
            }

            // Update the hasher with the chunk
            hasher.update(&buffer[..bytes_read]);
        }

        // Finalize the hash and get the result
        let result: [u8; 16] = hasher.finalize().into();
        let bytes: &[i32] = bytemuck::cast_slice(&result);
        debug!("Hash bytes are [{:?}]", bytes);

        Ok((bytes[0], bytes[1], bytes[2], bytes[3]))
    }
}
