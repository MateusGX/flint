# Install the flint CLI from GitHub Releases.
#
# Usage:
#   irm https://flint.devlayer.app/install.ps1 | iex
#
# Environment variables:
#   FLINT_VERSION     Release tag to install (default: latest)
#   FLINT_INSTALL_DIR Directory to install the binary into (default: %LOCALAPPDATA%\Flint\bin)
$ErrorActionPreference = "Stop"

$Repo = "MateusGX/flint"
$BinName = "flint"
$Target = "x86_64-pc-windows-msvc"

$Version = if ($env:FLINT_VERSION) { $env:FLINT_VERSION } else { "latest" }
$InstallDir = if ($env:FLINT_INSTALL_DIR) { $env:FLINT_INSTALL_DIR } else { "$env:LOCALAPPDATA\Flint\bin" }

$Archive = "$BinName-$Target.zip"
if ($Version -eq "latest") {
  $Url = "https://github.com/$Repo/releases/latest/download/$Archive"
} else {
  $Url = "https://github.com/$Repo/releases/download/$Version/$Archive"
}

$TmpDir = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid())
New-Item -ItemType Directory -Path $TmpDir | Out-Null
$ArchivePath = Join-Path $TmpDir $Archive

try {
  Write-Host "Downloading $Url"
  Invoke-WebRequest -Uri $Url -OutFile $ArchivePath

  Expand-Archive -Path $ArchivePath -DestinationPath $TmpDir -Force

  New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
  Move-Item -Path (Join-Path $TmpDir "$BinName.exe") -Destination (Join-Path $InstallDir "$BinName.exe") -Force
} finally {
  Remove-Item -Recurse -Force $TmpDir
}

Write-Host "Installed $BinName to $InstallDir\$BinName.exe"

$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if (-not (($UserPath -split ";") -contains $InstallDir)) {
  [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
  Write-Host "Added $InstallDir to your user PATH. Restart your terminal to use it."
}

& "$InstallDir\$BinName.exe" version
