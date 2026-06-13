# Uninstall the flint CLI.
#
# Usage:
#   irm https://flint.devlayer.app/uninstall.ps1 | iex
#
# Environment variables:
#   FLINT_INSTALL_DIR Directory the binary was installed into (default: %LOCALAPPDATA%\Flint\bin)
$ErrorActionPreference = "Stop"

$BinName = "flint"
$InstallDir = if ($env:FLINT_INSTALL_DIR) { $env:FLINT_INSTALL_DIR } else { "$env:LOCALAPPDATA\Flint\bin" }
$BinPath = Join-Path $InstallDir "$BinName.exe"

if (Test-Path $BinPath) {
  Remove-Item -Force $BinPath
  Write-Host "Removed $BinPath"
} else {
  Write-Host "$BinPath not found, nothing to remove"
}

$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
$PathEntries = $UserPath -split ";"
if ($PathEntries -contains $InstallDir) {
  $NewPath = ($PathEntries | Where-Object { $_ -ne $InstallDir }) -join ";"
  [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
  Write-Host "Removed $InstallDir from your user PATH."
}
