use bitflags::bitflags;

use crate::types::column::integer::Integer;
use crate::types::helpers::to_msi_value::IntoMsiValue;
bitflags! {
    /// Specifies options for remote execution.
    ///
    /// # Note:
    /// In the case of an .msi file that is being downloaded from a web
    /// location, the attribute flags should not be set to allow a component to
    /// be run-from-source. This is a limitation of the Windows Installer and
    /// can return a feature state of INSTALLSTATE_BADCONFIG.
    #[derive(Clone, Copy, Debug, PartialEq, whimsi_macros::BitmaskToValue)]
    pub struct ComponentAttributes: Integer {
        /// Component cannot be run from source. Set this bit for all components
        /// belonging to a feature to prevent the feature from being
        /// run-from-network or run-from-source. Note that if a feature has no
        /// components, the feature always shows run-from-source and
        /// run-from-my-computer as valid options.
        const LocalOnly = 0x0000;

        /// Component can only be run from source. Set this bit for all
        /// components belonging to a feature to prevent the feature from being
        /// run-from-my-computer. Note that if a feature has no components, the
        /// feature always shows run-from-source and run-from-my-computer as
        /// valid options.
        const SourceOnly = 0x0001;

        ///  Component can run locally or from source.
        const Optional = 0x0002;

        /// If this bit is set, the value in the KeyPath column is used as a key
        /// into the Registry table. If the Value field of the corresponding
        /// record in the Registry table is null, the Name field in that record
        /// must not contain "+", "-", or "*". For more information, see the
        /// description of the Name field in Registry table.
        ///
        /// Setting this bit is recommended for registry entries written to the
        /// HKCU hive. This ensures the installer writes the necessary HKCU
        /// registry entries when there are multiple users on the same machine.
        const RegistryKeyPath = 0x0004;

        /// If this bit is set, the installer increments the reference count in
        /// the shared DLL registry of the component's key file. If this bit is
        /// not set, the installer increments the reference count only if the
        /// reference count already exists.
        const SharedDLLRefCount = 0x0008;

        /// If this bit is set, the installer does not remove the component
        /// during an uninstall. The installer registers an extra system client
        /// for the component in the Windows Installer registry settings.
        const Permanent = 0x0010;

        /// If this bit is set, the value in the KeyPath column is a key into
        /// the ODBCDataSource table.
        const ODBCDataSource = 0x0020;

        /// If this bit is set, the installer reevaluates the value of the
        /// statement in the Condition column upon a reinstall. If the value was
        /// previously False and has changed to True, the installer installs the
        /// component. If the value was previously True and has changed to
        /// False, the installer removes the component even if the component has
        /// other products as clients.
        ///
        /// This bit should only be set for transitive components. See [Using
        /// Transitive
        /// Components](https://learn.microsoft.com/en-us/windows/win32/msi/using-transitive-components).
        const Transitive = 0x0040;

        /// If this bit is set, the installer does not install or reinstall the
        /// component if a key path file or a key path registry entry for the
        /// component already exists. The application does register itself as a
        /// client of the component.
        /// Use this flag only for components that are being registered by the
        /// Registry table. Do not use this flag for components registered by
        /// the AppId, Class, Extension, ProgId, MIME, and Verb tables.
        const NeverOverwrite = 0x0080;

        /// Set this bit to mark this as a 64-bit component. This attribute
        /// facilitates the installation of packages that include both 32-bit
        /// and 64-bit components. If this bit is not set, the component is
        /// registered as a 32-bit component. If this is a 64-bit component
        /// replacing a 32-bit component, set this bit and assign a new GUID in
        /// the ComponentId column.
        const _64Bit = 0x0100;

        /// Set this bit to disable Registry Reflection on all existing and new
        /// registry keys affected by this component. If this bit is set, the
        /// Windows Installer calls the RegDisableReflectionKey on each key
        /// being accessed by the component. This bit is available with Windows
        /// Installer version 4.0. This bit is ignored on 32-bit systems. This
        /// bit is ignored on the 64-bit versions of Windows XP.
        /// *Note*: 32-bit Windows applications running on the 64-bit Windows
        /// emulator (WOW64) refer to a different view of the registry than
        /// 64-bit applications. Registry reflection copies some registry values
        /// between these two registry views.
        const DisableRegistryReflection = 0x0200;

        /// Set this bit for a component in a patch package to prevent leaving
        /// orphan components on the computer. If a subsequent patch is
        /// installed, marked with the msidbPatchSequenceSupersedeEarlier value
        /// in its MsiPatchSequence table to supersede the first patch, Windows
        /// Installer 4.5 and later can unregister and uninstall components
        /// marked with the msidbComponentAttributesUninstallOnSupersedence
        /// value. If the component is not marked with this bit, installation of
        /// a superseding patch can leave behind an unused component on the
        /// computer.
        /// Setting the MSIUNINSTALLSUPERSEDEDCOMPONENTS property has the same
        /// effect as setting this bit for all components.
        /// *Windows Installer 4.0 and earlier*: The
        /// msidbComponentAttributesUninstallOnSupersedence value is not
        /// supported and is ignored.
        const UninstallOnSupersedence = 0x0400;

        /// If a component is marked with this attribute value in at least one
        /// package installed on the system, the installer treats the component
        /// as marked in all packages. If a package that shares the marked
        /// component is uninstalled, Windows Installer 4.5 can continue to
        /// share the highest version of the component on the system, even if
        /// that highest version was installed by the package that is being
        /// uninstalled.
        /// If the DisableSharedComponent policy is set to 1, no package gets
        /// the shared component functionality enabled by this bit.
        /// *Windows Installer 4.0 and earlier*: The
        /// msidbComponentAttributesShared value is not supported and is
        /// ignored.
        const Shared = 0x0800;
    }
}
