#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ServiceType {
    /// A Microsoft Win32 service that runs its own process.
    OwnProcess = 0x00000010,
    /// A Win32 service that shares a process.
    ShareProcess = 0x00000020,
    // /// A Win32 service that interacts with the desktop. This value cannot be used alone and must
    // /// be added to one of the two previous types.The StartName column must be set to LocalSystem
    // /// or null when using this flag.
    // TODO: Make this enum a bitflag so we can uncomment this.
    // InteractiveProcess = 0x00000100,
}
