use crate::types::column::integer::Integer;

/// *References*:
/// - [Summary List of All Custom Action Types](https://learn.microsoft.com/en-us/windows/win32/msi/summary-list-of-all-custom-action-types)
/// - [Custom Action Return Processing Options](https://learn.microsoft.com/en-us/windows/win32/msi/custom-action-return-processing-options)
/// - [Custom Action Execution Scheduling Options](https://learn.microsoft.com/en-us/windows/win32/msi/custom-action-execution-scheduling-options)
/// - [Custom Action In-Script Execution Options](https://learn.microsoft.com/en-us/windows/win32/msi/custom-action-in-script-execution-options)
/// - [Custom Action Patch Uninstall Option](https://learn.microsoft.com/en-us/windows/win32/msi/custom-action-patch-uninstall-option)
/// - [Custom Action Hidden Target Option](https://learn.microsoft.com/en-us/windows/win32/msi/custom-action-hidden-target-option)
/// - [Custom Action Return Values](https://learn.microsoft.com/en-us/windows/win32/msi/custom-action-return-values)
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(i32)]
pub enum CustomActionType {
    Basic(BasicCustomActionType),
    // TODO: Much bigger task than implementing the basic tasks unless I want to just let them
    // calculate the type number themselves from the docs which I don't really.
    Advanced,
    Arbitrary(Integer),
}

impl msi::ToValue for CustomActionType {
    fn to_value(&self) -> msi::Value {
        let int = match self {
            CustomActionType::Basic(basic_custom_action_type) => {
                *basic_custom_action_type as i32
            }
            CustomActionType::Advanced => todo!(),
            CustomActionType::Arbitrary(val) => *val as i32,
        };

        msi::Value::Int(int)
    }
}

/// Basic custom actions as outlined in the reference below.
///
/// [*Reference*](https://learn.microsoft.com/en-us/windows/win32/msi/summary-list-of-all-custom-action-types)
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(i16)]
pub enum BasicCustomActionType {
    /// DLL file stored in a Binary table stream.
    ///
    /// Source: Key to Binary table.
    /// Target: DLL entry point.
    BinaryTableDll = 1,

    /// EXE file stored in a Binary table stream.
    ///
    /// Source: Key to Binary table.
    /// Target:  Command-line string.
    BinaryTableExe = 2,

    /// JScript file stored in a Binary table stream.
    ///
    /// Source: Key to Binary table.
    /// Target: An optional JScript function that can be called.
    BinaryTableJScript = 5,

    /// VBScript file stored in a Binary table stream.
    ///
    /// Source: Key to Binary table.
    /// Target: An optional VBScript function that can be called.
    BinaryTableVBScript = 6,

    /// DLL file that is installed with a product.
    ///
    /// Source: Key to File table.
    /// Target: DLL entry point.
    InstalledFileDll = 17,

    /// EXE file that is installed with a product.
    ///
    /// Source: Key to File table.
    /// Target: Command-line string.
    InstalledFileExe = 18,

    /// Displays a specified error message and returns failure, terminating the installation.
    ///
    /// Source: Blank
    /// Target:  	Formatted text string. The literal message or an index into the Error table.
    CustomError = 19,

    /// JScript file that is installed with a product.
    ///
    /// Source: Key to File table.
    /// Target:  	An optional JScript function that can be called.
    InstalledFileJScript = 21,

    /// VBScript file that is installed with a product.
    ///
    /// Source: Key to File table.
    /// Target: An optional VBScript function that can be called.
    InstalledFileVBScript = 22,

    /// EXE file having a path referencing a directory.
    ///
    /// Source: Key to Directory table. This is the working directory for execution.
    /// Target: The Target column is formatted and contains the full path and name of the executable file followed by optional arguments.
    ExistingExeDirectory = 34,

    /// Directory set with formatted text.
    ///
    /// Source: A key to the Directory table. The designated directory is set by the formatted string in the Target field.
    /// Target: A formatted text string.
    SetDirectory = 35,

    ///JScript text stored in this sequence table.
    ///
    /// Source: Null
    /// Target:  	A string of JScript code.
    RawJScript = 37,

    /// VBScript text stored in this sequence table.
    ///
    /// Source: Null
    /// Target: A string of VBScript code.
    RawVBScript = 38,

    /// EXE file having a path specified by a property value.
    ///
    /// Source: Property name or key to Property table.
    /// Target: Command-line string.
    PropertyExe = 50,

    /// Property set with formatted text.
    ///
    /// Source: Property name or key to the Property table. This property is set by the formatted string in the Target field.
    /// Target: A formatted text string.
    SetProperty = 51,

    /// JScript text specified by a property value.
    ///
    /// Source: Property name or key to Property table.
    /// Target: An optional JScript function that can be called.
    PropertyJScript = 53,

    /// VBScript text specified by a property value.
    ///
    /// Source: Property name or key to Property table.
    /// Target: An optional VBScript function that can be called.
    PropertyVBScript = 54,
}
