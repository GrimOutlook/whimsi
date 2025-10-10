/// This column specifies the action taken by the startup program if the service fails to start
/// during startup. These values affect the ServiceControl StartService events for installed
/// services. One of the following error control flags must be specified in this column.
#[derive(Debug, Clone, Copy, PartialEq, whimsi_macros::IntToValue)]
#[repr(i32)]
pub enum ErrorControl {
    /// Logs the error and continues with the startup operation.
    Ignore = 0x00000000,
    /// Logs the error, displays a message box and continues the startup operation.
    Normal = 0x00000001,
    /// Logs the error if it is possible and the system is restarted with the last configuration
    /// known to be good. If the last-known-good configuration is being started, the startup
    /// operation fails.
    Critical = 0x00000003,
    /// Adding the constant msidbServiceInstallErrorControlVital (value = 0x08000) to the flags in
    /// the following table specifies that the overall install should fail if the service cannot be
    /// installed into the system.
    Vital = 0x08000,
}
