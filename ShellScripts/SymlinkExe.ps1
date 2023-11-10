param (
    [string]$linkName,
    [string]$destination,
    [string]$target
)

$linkPath = Join-Path $destination $linkName

Write-Host "Creating a symlink:"
Write-Host "Link name: $linkName"
Write-Host "Destination: $destination"
Write-Host "Target: $target"
Write-Host "Full link path: $linkPath"

if (Test-Path $linkPath) {
    Write-Host "A file or symlink already exists at $linkPath. Removing it."
    Remove-Item $linkPath -Force
}

New-Item -ItemType SymbolicLink -Path $linkPath -Target $target

Write-Host "Symlink created successfully."
