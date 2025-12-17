# Build Windows MSI Installer using WiX Toolset
param(
    [string]$Version = "",
    [string]$Configuration = "Release"
)

$ErrorActionPreference = "Stop"

# Paths
$RootDir = Split-Path -Parent $PSScriptRoot
$CoreDir = Join-Path $RootDir "core"
$PlatformDir = Join-Path $RootDir "platforms\windows"
$InstallerDir = Join-Path $PlatformDir "Installer"
$PublishDir = Join-Path $PlatformDir "publish"
$OutputDir = Join-Path $PlatformDir "msi-output"

Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "  GoNhanh MSI Installer Build" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan

# 1. Check WiX Toolset installation
Write-Host "`n[1/8] Checking WiX Toolset..." -ForegroundColor Yellow
$WixDir = "${env:WIX}bin"
if (-not $env:WIX -or -not (Test-Path $WixDir)) {
    Write-Host "ERROR: WiX Toolset not found!" -ForegroundColor Red
    Write-Host "Please install WiX Toolset 3.11+ from https://wixtoolset.org/releases/" -ForegroundColor Yellow
    exit 1
}

$CandleExe = Join-Path $WixDir "candle.exe"
$LightExe = Join-Path $WixDir "light.exe"
$HeatExe = Join-Path $WixDir "heat.exe"

Write-Host "  [OK] WiX Toolset found at: $WixDir" -ForegroundColor Green

# 2. Determine version
Write-Host "`n[2/8] Determining version..." -ForegroundColor Yellow
if ($Version -eq "") {
    # Try to get from git tag
    $GitTag = git describe --tags --abbrev=0 2>$null
    if ($GitTag) {
        $Version = $GitTag.TrimStart('v')
    } else {
        # Fallback: read from Cargo.toml
        $CargoToml = Get-Content (Join-Path $CoreDir "Cargo.toml") -Raw
        if ($CargoToml -match 'version\s*=\s*"([^"]+)"') {
            $Version = $Matches[1]
        } else {
            $Version = "1.0.0"
        }
    }
}
Write-Host "  Version: $Version" -ForegroundColor Green

# 3. Build Rust core
Write-Host "`n[3/8] Building Rust core..." -ForegroundColor Yellow
Push-Location $CoreDir
try {
    cargo build --release --target x86_64-pc-windows-msvc
    if ($LASTEXITCODE -ne 0) { throw "Cargo build failed" }
    Write-Host "  [OK] Rust core built" -ForegroundColor Green
} finally {
    Pop-Location
}

# 4. Copy DLL to Native/
Write-Host "`n[4/8] Copying Rust DLL..." -ForegroundColor Yellow
$NativeDir = Join-Path $PlatformDir "GoNhanh\Native"
New-Item -ItemType Directory -Force -Path $NativeDir | Out-Null
Copy-Item (Join-Path $CoreDir "target\x86_64-pc-windows-msvc\release\gonhanh_core.dll") `
          (Join-Path $NativeDir "gonhanh_core.dll") -Force
Write-Host "  [OK] DLL copied to Native/" -ForegroundColor Green

# 5. Build .NET app and publish
Write-Host "`n[5/8] Publishing .NET application..." -ForegroundColor Yellow
Push-Location (Join-Path $PlatformDir "GoNhanh")
try {
    # Clean previous publish
    if (Test-Path $PublishDir) {
        Remove-Item $PublishDir -Recurse -Force
    }

    dotnet publish -c $Configuration -r win-x64 --self-contained false -o $PublishDir /p:Version=$Version
    if ($LASTEXITCODE -ne 0) { throw "Dotnet publish failed" }
    Write-Host "  [OK] .NET app published to: $PublishDir" -ForegroundColor Green
} finally {
    Pop-Location
}

# 6. Create output directory
Write-Host "`n[6/8] Preparing output directory..." -ForegroundColor Yellow
if (Test-Path $OutputDir) {
    Remove-Item $OutputDir -Recurse -Force
}
New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null
Write-Host "  [OK] Output directory ready" -ForegroundColor Green

# 7. Compile WiX sources
Write-Host "`n[7/8] Compiling WiX sources..." -ForegroundColor Yellow
Push-Location $InstallerDir
try {
    # Candle: Compile .wxs to .wixobj
    & $CandleExe -arch x64 `
        -dPublishDir="$PublishDir" `
        -dVersion="$Version" `
        -ext WixUIExtension `
        -ext WixUtilExtension `
        -ext WixNetFxExtension `
        -out "$OutputDir\" `
        Product.wxs Components.wxs

    if ($LASTEXITCODE -ne 0) { throw "Candle compilation failed" }
    Write-Host "  [OK] WiX sources compiled" -ForegroundColor Green
} finally {
    Pop-Location
}

# 8. Link into MSI
Write-Host "`n[8/8] Linking MSI..." -ForegroundColor Yellow
Push-Location $OutputDir
try {
    $MsiName = "GoNhanh-$Version-win-x64.msi"
    $MsiNameLatest = "GoNhanh-win-x64.msi"

    & $LightExe -out $MsiName `
        -ext WixUIExtension `
        -ext WixUtilExtension `
        -ext WixNetFxExtension `
        -cultures:en-us `
        -loc "$InstallerDir\en-us.wxl" `
        -spdb `
        Product.wixobj Components.wixobj

    if ($LASTEXITCODE -ne 0) {
        # Try without localization file if it doesn't exist
        & $LightExe -out $MsiName `
            -ext WixUIExtension `
            -ext WixUtilExtension `
            -ext WixNetFxExtension `
            -spdb `
            Product.wixobj Components.wixobj

        if ($LASTEXITCODE -ne 0) { throw "Light linking failed" }
    }

    # Create version-less copy for "latest" download
    Copy-Item $MsiName $MsiNameLatest

    Write-Host "  [OK] MSI created: $MsiName" -ForegroundColor Green
    Write-Host "  [OK] Latest copy: $MsiNameLatest" -ForegroundColor Green
} finally {
    Pop-Location
}

# Summary
Write-Host "`n=====================================" -ForegroundColor Cyan
Write-Host "  [SUCCESS] Build Complete!" -ForegroundColor Green
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "`nOutput files:" -ForegroundColor Cyan
Write-Host "  - $OutputDir\GoNhanh-$Version-win-x64.msi" -ForegroundColor White
Write-Host "  - $OutputDir\GoNhanh-win-x64.msi" -ForegroundColor White
Write-Host "`nTo install:" -ForegroundColor Cyan
Write-Host "  msiexec /i `"$OutputDir\GoNhanh-$Version-win-x64.msi`"" -ForegroundColor Yellow
