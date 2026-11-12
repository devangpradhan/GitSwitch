param(
    [string]$Version = "",
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

$RootDir = Split-Path -Parent $PSScriptRoot
Push-Location $RootDir

try {
    if ([string]::IsNullOrWhiteSpace($Version)) {
        $tauriConf = Get-Content (Join-Path $RootDir "src-tauri\tauri.conf.json") -Raw | ConvertFrom-Json
        $Version = $tauriConf.version
    }

    $CleanVersion = $Version.TrimStart("v")
    $TagVersion = "v$CleanVersion"
    $ReleaseDir = Join-Path $RootDir "release\$TagVersion"
    $BundleName = "GitSwitch-$TagVersion-windows-x64"
    $BundleDir = Join-Path $ReleaseDir $BundleName
    $ZipPath = Join-Path $ReleaseDir "$BundleName.zip"

    if (-not $SkipBuild) {
        Write-Host "Building frontend and Tauri desktop binary ($TagVersion)..."
        & (Join-Path $RootDir "frontend\node_modules\.bin\tauri.cmd") build --no-bundle
        if ($LASTEXITCODE -ne 0) {
            throw "Tauri build failed with exit code $LASTEXITCODE"
        }

        Write-Host "Building MCP server binary ($TagVersion)..."
        cargo build --release --bin gitswitch-mcp --manifest-path (Join-Path $RootDir "src-tauri\Cargo.toml")
        if ($LASTEXITCODE -ne 0) {
            throw "MCP cargo build failed with exit code $LASTEXITCODE"
        }
    }

    $GuiExe = Join-Path $RootDir "src-tauri\target\release\gitswitch.exe"
    $McpExe = Join-Path $RootDir "src-tauri\target\release\gitswitch-mcp.exe"

    if (-not (Test-Path $GuiExe)) { throw "Missing $GuiExe" }
    if (-not (Test-Path $McpExe)) { throw "Missing $McpExe" }

    if (-not (Test-Path $BundleDir)) {
        New-Item -ItemType Directory -Path $BundleDir -Force | Out-Null
    }

    # Populate portable bundle folder (overwrite files in-place to avoid Windows Defender directory-delete locks)
    Copy-Item $GuiExe (Join-Path $BundleDir "gitswitch.exe") -Force
    Copy-Item $McpExe (Join-Path $BundleDir "gitswitch-mcp.exe") -Force
    Copy-Item (Join-Path $RootDir "mcp_config.json") (Join-Path $BundleDir "mcp_config.json") -Force
    Copy-Item (Join-Path $RootDir "README.md") (Join-Path $BundleDir "README.md") -Force
    Copy-Item (Join-Path $RootDir "CHANGELOG.md") (Join-Path $BundleDir "CHANGELOG.md") -Force
    Copy-Item (Join-Path $RootDir "LICENSE") (Join-Path $BundleDir "LICENSE") -Force

    # Copy direct standalone download assets for GitHub Releases
    $StandaloneGui = Join-Path $ReleaseDir "gitswitch-$TagVersion-windows-x64.exe"
    $StandaloneMcp = Join-Path $ReleaseDir "gitswitch-mcp-$TagVersion-windows-x64.exe"
    try {
        Copy-Item $GuiExe $StandaloneGui -Force -ErrorAction Stop
    } catch {
        Write-Warning "Could not overwrite $StandaloneGui (file may be open/running): $($_.Exception.Message)"
    }
    try {
        Copy-Item $McpExe $StandaloneMcp -Force -ErrorAction Stop
    } catch {
        Write-Warning "Could not overwrite $StandaloneMcp (file may be open/running): $($_.Exception.Message)"
    }

    # Rebuild portable zip archive
    if (Test-Path $ZipPath) {
        Remove-Item $ZipPath -Force
    }
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    [System.IO.Compression.ZipFile]::CreateFromDirectory($BundleDir, $ZipPath)

    # Generate SHA256SUMS.txt
    $SumFiles = @(
        "gitswitch-$TagVersion-windows-x64.exe",
        "gitswitch-mcp-$TagVersion-windows-x64.exe",
        "$BundleName.zip"
    )
    $Checksums = foreach ($f in $SumFiles) {
        $full = Join-Path $ReleaseDir $f
        $hash = (Get-FileHash -Path $full -Algorithm SHA256).Hash.ToLowerInvariant()
        "$hash  $f"
    }
    $Checksums | Set-Content -Path (Join-Path $ReleaseDir "SHA256SUMS.txt") -Encoding UTF8

    Write-Host "Release artifacts created at: $ReleaseDir"
    Get-ChildItem $ReleaseDir | Format-Table Name, Length, LastWriteTime -AutoSize
}
finally {
    Pop-Location
}
