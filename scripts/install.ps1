# install.ps1 - sshp installer for Windows
# Usage: irm https://raw.githubusercontent.com/kaushiktadhani/sshp/main/install.ps1 | iex

$ErrorActionPreference = "Stop"

$Repo = "kaushiktadhani/sshp"
$BinaryName = "sshp"

# Detect architecture
$Arch = $env:PROCESSOR_ARCHITECTURE
switch ($Arch) {
    "AMD64"   { $Target = "x86_64-pc-windows-msvc" }
    "ARM64" {
        Write-Error "ARM64 Windows is not yet supported. Please build from source: cargo install sshp"
        exit 1
    }
    default {
        Write-Error "Unsupported architecture: $Arch"
        exit 1
    }
}

# Get latest release tag
Write-Host "Fetching latest release..."
$Release = Invoke-RestMethod "https://api.github.com/repos/$Repo/releases/latest"
$LatestTag = $Release.tag_name

if (-not $LatestTag) {
    Write-Error "Failed to determine the latest release tag."
    exit 1
}

Write-Host "Installing $BinaryName $LatestTag for $Target..."

$Archive = "$BinaryName-$LatestTag-$Target.zip"
$BaseUrl = "https://github.com/$Repo/releases/download/$LatestTag"

# Download to temp directory
$TmpDir = New-TemporaryFile | ForEach-Object { Remove-Item $_; New-Item -ItemType Directory -Path $_ }
$ArchivePath = Join-Path $TmpDir $Archive
$ChecksumPath = "$ArchivePath.sha256"

try {
    Write-Host "Downloading $Archive..."
    Invoke-WebRequest "$BaseUrl/$Archive" -OutFile $ArchivePath
    Invoke-WebRequest "$BaseUrl/$Archive.sha256" -OutFile $ChecksumPath

    # Verify checksum
    Write-Host "Verifying checksum..."
    $Expected = (Get-Content $ChecksumPath).Split(" ")[0].Trim()
    $Actual = (Get-FileHash $ArchivePath -Algorithm SHA256).Hash.ToLower()

    if ($Actual -ne $Expected) {
        Write-Error "Checksum mismatch! Expected: $Expected  Got: $Actual"
        exit 1
    }
    Write-Host "Checksum verified."

    # Extract archive
    Expand-Archive $ArchivePath -DestinationPath $TmpDir -Force

    # Determine install directory
    $InstallDir = "$env:USERPROFILE\.local\bin"
    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir | Out-Null
    }

    # Copy binary
    $BinarySource = Join-Path $TmpDir "$BinaryName.exe"
    $BinaryDest = Join-Path $InstallDir "$BinaryName.exe"
    Copy-Item $BinarySource $BinaryDest -Force

    Write-Host ""
    Write-Host "$BinaryName $LatestTag installed to $BinaryDest"

    # Add to PATH if not already there
    $UserPath = [System.Environment]::GetEnvironmentVariable("Path", "User")
    if ($UserPath -notlike "*$InstallDir*") {
        Write-Host "Adding $InstallDir to your user PATH..."
        [System.Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
        Write-Host "PATH updated. Restart your terminal for the change to take effect."
    } else {
        Write-Host "$InstallDir is already in your PATH."
    }
} finally {
    Remove-Item -Recurse -Force $TmpDir -ErrorAction SilentlyContinue
}
