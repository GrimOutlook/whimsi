#[derive(Debug, Clone, Copy, PartialEq, whimsi_macros::ReprToValue)]
#[repr(i32)]
pub enum StartType {
    /// A service start during startup of the system.
    AutoStart   = 0x00000002,
    /// A service start when the service control manager calls the StartService
    /// function.
    DemandStart = 0x00000003,
    /// Specifies a service that can no longer be started.
    Disables    = 0x00000004,
}
