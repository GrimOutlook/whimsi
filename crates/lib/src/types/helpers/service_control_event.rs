use bitflags::bitflags;

bitflags! {
    /// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/servicecontrol-table#event)
    #[derive(Clone, Copy, Debug, PartialEq, whimsi_macros::BitmaskToValue)]
    pub struct ServiceControlEvent: i16 {
        /// Starts the service during the StartServices action when the MSI is being installed.
        const INSTALL_START = 1;
        /// Stops the service during the StopServices action when the MSI is being installed.
        const INSTALL_STOP = 2;
        /// Deletes the service during the DeleteServices action when the MSI is being installed.
        const INSTALL_DELETE = 8;
        /// Starts the service during the StartServices action when the MSI is being uninstalled.
        const UNINSTALL_START = 16;
        /// Stops the service during the StopServices action when the MSI is being uninstalled.
        const UNINSTALL_STOP = 32;
        /// Deletes the service during the DeleteServices action when the MSI is being uninstalled.
        const UNINSTALL_DELETE = 128;
    }
}
