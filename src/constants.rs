// Found by inspecting MSIs made by other tools. Couldn't find official
// documentation on these values.
pub const CONDITION_MAX_LEN: usize = 255;
pub const DEFAULT_DIR_MAX_LEN: usize = 255;
pub const FILENAME_MAX_LEN: usize = 255;
pub const GUID_MAX_LEN: usize = 38;
pub const IDENTIFIER_MAX_LEN: usize = 72;
pub const VERSION_MAX_LEN: usize = 72;
pub const LANGUAGE_MAX_LEN: usize = 20;

// Found here: https://learn.microsoft.com/en-us/windows/win32/msi/filename
pub const SHORT_FILENAME_MAX_LEN: usize = 8;
//TODO: Move filename invalid character array list here.

// Found here: https://learn.microsoft.com/en-us/windows/win32/msi/media-table
pub const LAST_SEQUENCE_MIN: usize = 0;
pub const LAST_SEQUENCE_MAX: usize = 32767;

// Found here: https://learn.microsoft.com/en-us/windows/win32/msi/media-table
pub const DISK_ID_MIN: usize = 1;
