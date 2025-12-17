# Migrate repository URLs from khaphanspace to nguyennhuanhle
param(
    [string]$OldOwner = "khaphanspace",
    [string]$NewOwner = "nguyennhuanhle",
    [switch]$DryRun = $false
)

$ErrorActionPreference = "Stop"

$RootDir = Split-Path -Parent $PSScriptRoot

Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "  Repository URL Migration" -ForegroundColor Cyan
Write-Host "  $OldOwner → $NewOwner" -ForegroundColor Cyan
if ($DryRun) {
    Write-Host "  MODE: DRY RUN (no changes)" -ForegroundColor Yellow
}
Write-Host "=====================================" -ForegroundColor Cyan

# Files to update (relative to root)
$FilesToUpdate = @(
    "core\Cargo.toml",
    "README.md",
    "Makefile",
    "CONTRIBUTING.md",
    ".github\workflows\release.yml",
    ".github\workflows\ci.yml",
    ".github\ISSUE_TEMPLATE\config.yml",
    "scripts\install-linux.sh",
    "scripts\generate-release-notes.sh",
    "platforms\windows\GoNhanh\Core\AppMetadata.cs",
    "platforms\macos\AppMetadata.swift",
    "platforms\macos\UpdateChecker.swift",
    "platforms\linux\scripts\gonhanh-cli.sh",
    "docs\README.md",
    "docs\development.md",
    "docs\install-linux.md",
    "docs\install-macos.md",
    "docs\install-windows.md",
    "docs\project-overview-pdr.md",
    "docs\codebase-summary.md",
    "docs\common-issues.md",
    "docs\core-engine-algorithm.md",
    "docs\system-architecture.md",
    "docs\vietnamese-language-system.md",
    "docs\validation-algorithm.md"
)

$UpdateCount = 0
$TotalChanges = 0
$FileChangeDetails = @()

foreach ($RelPath in $FilesToUpdate) {
    $FilePath = Join-Path $RootDir $RelPath

    if (!(Test-Path $FilePath)) {
        Write-Host "  [SKIP] $RelPath (not found)" -ForegroundColor Gray
        continue
    }

    $Content = Get-Content $FilePath -Raw -Encoding UTF8
    $OriginalContent = $Content

    # Replace patterns
    $Content = $Content -replace "$OldOwner/gonhanh\.org", "$NewOwner/gonhanh.org"
    $Content = $Content -replace "$OldOwner/gonhanh`"", "$NewOwner/gonhanh`""
    $Content = $Content -replace "raw\.githubusercontent\.com/$OldOwner/", "raw.githubusercontent.com/$NewOwner/"
    $Content = $Content -replace "github\.com/$OldOwner/homebrew-gonhanh", "github.com/$NewOwner/homebrew-gonhanh"
    $Content = $Content -replace "api\.github\.com/repos/$OldOwner/", "api.github.com/repos/$NewOwner/"

    # Count changes
    $BeforeLines = $OriginalContent -split "`n"
    $AfterLines = $Content -split "`n"
    $Changes = 0

    for ($i = 0; $i -lt $BeforeLines.Count; $i++) {
        if ($BeforeLines[$i] -ne $AfterLines[$i]) {
            $Changes++
        }
    }

    if ($Content -ne $OriginalContent) {
        $UpdateCount++
        $TotalChanges += $Changes

        $FileChangeDetails += [PSCustomObject]@{
            File = $RelPath
            Changes = $Changes
        }

        if (!$DryRun) {
            Set-Content $FilePath -Value $Content -NoNewline -Encoding UTF8
            Write-Host "  [UPDATE] $RelPath ($Changes line(s) changed)" -ForegroundColor Green
        } else {
            Write-Host "  [WOULD UPDATE] $RelPath ($Changes line(s) would change)" -ForegroundColor Yellow
        }
    } else {
        Write-Host "  [NO CHANGE] $RelPath" -ForegroundColor Gray
    }
}

Write-Host "`n=====================================" -ForegroundColor Cyan
Write-Host "  Summary:" -ForegroundColor Cyan
Write-Host "  Files updated: $UpdateCount" -ForegroundColor $(if ($UpdateCount -gt 0) { "Green" } else { "Gray" })
Write-Host "  Total changes: $TotalChanges line(s)" -ForegroundColor $(if ($TotalChanges -gt 0) { "Green" } else { "Gray" })
Write-Host "=====================================" -ForegroundColor Cyan

if ($FileChangeDetails.Count -gt 0) {
    Write-Host "`nFiles with changes:" -ForegroundColor Cyan
    $FileChangeDetails | ForEach-Object {
        Write-Host "  - $($_.File): $($_.Changes) line(s)" -ForegroundColor White
    }
}

if ($DryRun) {
    Write-Host "`nRun without -DryRun to apply changes." -ForegroundColor Yellow
} else {
    Write-Host "`n[SUCCESS] Migration complete!" -ForegroundColor Green
    Write-Host "Run git status to review changes." -ForegroundColor Cyan
}
