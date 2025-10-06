use bitflags::bitflags;

bitflags! {
    /// This column contains the operations to be performed on the named service. Note that when
    /// stopping a service, all services that depend on that service are also stopped. When
    /// deleting a service that is running, the installer stops the service.
    ///
    /// The values in this field are bit fields that can be combined into a single value that
    /// represents several operations.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Event: i16 {
        /// Starts the service during the StartServices action when installing.
        const INSTALL_START = 1 << 0;
        /// Stops the service during the StopServices action when installing.
        const INSTALL_STOP = 1 << 2;
        // /// <reserved>
        // const INSTALL_RESERVED = 1 << 3;
        /// Deletes the service during the DeleteServices action when installing.
        const INSTALL_DELETE = 1 << 4;


        /// Starts the service during the StartServices action when uninstalling.
        const UNINSTALL_START = 1 << 5;
        /// Stops the service during the StopServices action when unsinstalling.
        const UNINSTALL_STOP = 1 << 6;
        // /// <reserved>
        // const UNINSTALL_RESERVED = 1 << 7;
        /// Deletes the service during the DeleteServices action when uninstalling.
        const UNINSTALL_DELETE = 1 << 8;
    }
}
