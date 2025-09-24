// TODO: Check if any of the other architectures need to be upper or lower case.
#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display)]
pub enum MsiArchitecture {
    /// x86 architecture (32-bit).
    #[display("x86")]
    X86,
    /// x64 architecture (64-bit).
    #[display("x64")]
    X64,
    /// Itanium/Intel architecture.
    Intel,
    /// Itanium/Intel 64-bit architecture.
    Intel64,
    /// ARM architecture.
    Arm,
    /// ARM64 architecture.
    Arm64,
    /// Represents an unknown or unsupported architecture.
    Unknown(String),
}
