# Verify all URLs are updated correctly
param([string]$Owner = "nguyennhuanhle")

$ErrorActionPreference = "Stop"

$RootDir = Split-Path -Parent $PSScriptRoot

Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "  Verifying Repository URLs" -ForegroundColor Cyan
Write-Host "  Expected owner: $Owner" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan

# File extensions to check
$Extensions = @("*.md", "*.yml", "*.yaml", "*.toml", "*.cs", "*.swift", "*.sh", "*.ps1", "*.json")

# Check for old owner references (excluding personal URLs)
Write-Host "`n[1/3] Checking for old references..." -ForegroundColor Yellow

$OldRefs = @()
foreach ($Ext in $Extensions) {
    $Files = Get-ChildItem $RootDir -Recurse -File -Include $Ext -ErrorAction SilentlyContinue
    foreach ($File in $Files) {
        # Skip verification scripts themselves
        if ($File.Name -eq "verify-urls.ps1" -or $File.Name -eq "migrate-repo-urls.ps1") {
            continue
        }
        $Matches = Select-String -Path $File.FullName -Pattern "khaphanspace/gonhanh" -AllMatches
        foreach ($Match in $Matches) {
            # Exclude personal URLs (LinkedIn, Sponsor)
            if ($Match.Line -notmatch "linkedin|sponsor") {
                $OldRefs += [PSCustomObject]@{
                    File = $File.FullName.Replace($RootDir + "\", "")
                    Line = $Match.LineNumber
                    Content = $Match.Line.Trim()
                }
            }
        }
    }
}

if ($OldRefs.Count -gt 0) {
    Write-Host "  [ERROR] Found $($OldRefs.Count) old reference(s):" -ForegroundColor Red
    $OldRefs | ForEach-Object {
        Write-Host "    $($_.File):$($_.Line)" -ForegroundColor Yellow
        Write-Host "      $($_.Content)" -ForegroundColor Gray
    }
    $HasErrors = $true
} else {
    Write-Host "  [OK] No old references found!" -ForegroundColor Green
}

# Verify new owner exists
Write-Host "`n[2/3] Checking for new references..." -ForegroundColor Yellow

$NewRefs = @()
foreach ($Ext in $Extensions) {
    $Files = Get-ChildItem $RootDir -Recurse -File -Include $Ext -ErrorAction SilentlyContinue
    foreach ($File in $Files) {
        $Matches = Select-String -Path $File.FullName -Pattern "$Owner/gonhanh" -AllMatches
        $NewRefs += $Matches
    }
}

Write-Host "  [OK] Found $($NewRefs.Count) reference(s) to $Owner/gonhanh" -ForegroundColor Green

# Check critical files specifically
Write-Host "`n[3/3] Checking critical files..." -ForegroundColor Yellow

$CriticalFiles = @(
    "README.md",
    "core\Cargo.toml",
    ".github\workflows\release.yml",
    "platforms\windows\GoNhanh\Core\AppMetadata.cs",
    "platforms\macos\AppMetadata.swift",
    "platforms\macos\UpdateChecker.swift"
)

$AllGood = $true
foreach ($RelPath in $CriticalFiles) {
    $FilePath = Join-Path $RootDir $RelPath
    if (Test-Path $FilePath) {
        $Content = Get-Content $FilePath -Raw
        if ($Content -match "$Owner/gonhanh") {
            Write-Host "  [OK] $RelPath - Updated" -ForegroundColor Green
        } else {
            Write-Host "  [ERROR] $RelPath - Missing new owner reference" -ForegroundColor Red
            $AllGood = $false
        }
    } else {
        Write-Host "  [WARN] $RelPath - File not found" -ForegroundColor Yellow
    }
}

# Final summary
Write-Host "`n=====================================" -ForegroundColor Cyan
if ($OldRefs.Count -eq 0 -and $AllGood) {
    Write-Host "  [SUCCESS] All URLs verified successfully!" -ForegroundColor Green
    Write-Host "=====================================" -ForegroundColor Cyan
    exit 0
} else {
    Write-Host "  [FAILED] Verification failed!" -ForegroundColor Red
    Write-Host "=====================================" -ForegroundColor Cyan
    Write-Host "`nPlease review and fix the issues above." -ForegroundColor Yellow
    exit 1
}
