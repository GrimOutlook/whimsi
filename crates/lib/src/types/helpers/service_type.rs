use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, whimsi_macros::BitmaskToValue)]
    pub struct ServiceType: i32 {
        /// A Microsoft Win32 service that runs its own process.
        const OwnProcess = 0x00000010;
        /// A Win32 service that shares a process.
        const ShareProcess = 0x00000020;
        /// A Win32 service that interacts with the desktop. This value cannot be used alone and must
        /// be added to one of the two previous types.The StartName column must be set to LocalSystem
        /// or null when using this flag.
        const InteractiveProcess = 0x00000100;
    }
}
