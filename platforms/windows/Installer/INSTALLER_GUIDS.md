# WiX Installer GUIDs

**CRITICAL: DO NOT CHANGE THESE GUIDs AFTER FIRST RELEASE!**

Changing GUIDs breaks upgrade capability for existing installations.

## Product GUIDs

### UpgradeCode
**GUID:** `2D9A3D64-E1BC-4F40-960D-37843A882626`

**Purpose:** Identifies the product family across all versions. Used by Windows Installer to detect upgrades.

**CRITICAL:** This GUID must NEVER change. Changing it will break automatic upgrades.

## Component GUIDs

### MainExecutableComponent
**GUID:** `AA36B1C4-6BFF-43C4-BFD6-7992931FE702`

**Files:** GoNhanh.exe

**Purpose:** Main application executable component.

### RustCoreComponent
**GUID:** `8E4D8D37-1ABA-4BA4-8B4F-7793D375208A`

**Files:** gonhanh_core.dll

**Purpose:** Rust core engine shared library.

### RuntimeDllsComponent
**GUID:** `816DB392-2DF3-45B7-AE00-AEB02015AB0D`

**Files:** All .NET runtime DLLs (harvested via heat.exe)

**Purpose:** .NET 8.0 runtime dependencies.

### RegistryComponent
**GUID:** `E83098FD-9FF1-46BE-A6C4-22B0E507466E`

**Registry Keys:**
- `HKCU\SOFTWARE\GoNhanh\InstallPath`
- `HKCU\SOFTWARE\GoNhanh\Version`
- `HKCU\SOFTWARE\GoNhanh\InputMethod`
- `HKCU\SOFTWARE\GoNhanh\ModernTone`
- `HKCU\SOFTWARE\GoNhanh\Enabled`
- `HKCU\SOFTWARE\GoNhanh\FirstRun`

**Purpose:** Application settings and metadata stored in Windows Registry.

### StartMenuShortcutComponent
**GUID:** `55078EA0-3740-46FE-8901-BD59563F8A7A`

**Shortcut:** Start Menu → GoNhanh

**Purpose:** Start Menu shortcut for launching the application.

### DesktopShortcutComponent (Optional)
**GUID:** `882F506B-C7DD-4870-BA94-63CA2ADDFBF7`

**Shortcut:** Desktop → GoNhanh

**Purpose:** Desktop shortcut (user can opt-in during installation).

### AutoStartComponent (Optional)
**GUID:** `550B9022-FDDE-41A5-94D5-8CD73B85AF28`

**Registry Key:** `HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Run\GoNhanh`

**Purpose:** Auto-start application on Windows login (user can opt-in).

---

## GUID Generation

Generated on: 2025-01-XX using PowerShell:
```powershell
[guid]::NewGuid().ToString().ToUpper()
```

## Notes

- **UpgradeCode** is shared across all versions (1.0.0, 1.0.1, 2.0.0, etc.)
- **Component GUIDs** should remain stable unless the component's install location changes
- If you change what a component installs (add/remove files), the GUID should change
- If you only update the file content (new version), keep the same GUID
