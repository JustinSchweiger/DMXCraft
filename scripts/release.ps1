# Make a new release: bump the version, commit, tag and push.
#
# Usage (from anywhere in the repository):
#   .\scripts\release.ps1 0.4.2          # or v0.4.2
#   .\scripts\release.ps1 0.4.2 -SkipTests
#
# Pushing the tag starts the GitHub release builds (installers, macOS DMG,
# install notes on the release page).

param(
    [Parameter(Mandatory = $true, Position = 0)][string]$Version,
    [switch]$SkipTests
)

$ErrorActionPreference = "Stop"
Set-Location (Split-Path $PSScriptRoot -Parent)

function Fail($msg) { Write-Host $msg -ForegroundColor Red; exit 1 }
function Run($exe, [string[]]$argv) {
    & $exe @argv
    if ($LASTEXITCODE -ne 0) { Fail "'$exe $($argv -join ' ')' failed." }
}

# --- Check the version -------------------------------------------------------
$Version = $Version.Trim().TrimStart("v", "V")
if ($Version -notmatch '^\d+\.\d+\.\d+(-[0-9A-Za-z.-]+)?$') {
    Fail "'$Version' is not a version like 0.4.2 (or 0.5.0-beta.1)."
}
$tag = "v$Version"

$toml = Get-Content Cargo.toml -Raw
$m = [regex]::Match($toml, '(?ms)^\[workspace\.package\].*?^version\s*=\s*"([^"]+)"')
if (-not $m.Success) { Fail "No version found in [workspace.package] of Cargo.toml." }
$current = $m.Groups[1].Value

function Core($v) { [version]($v -replace '-.*$', '') }
if ((Core $Version) -lt (Core $current) -or $Version -eq $current) {
    Fail "New version $Version must be higher than the current $current."
}

# --- Check the repository ----------------------------------------------------
$branch = (git rev-parse --abbrev-ref HEAD).Trim()
if ($branch -ne "main") { Fail "Not on main (on '$branch')." }
if (git status --porcelain) { Fail "There are uncommitted changes. Commit or stash them first." }

Run git @("pull", "--ff-only")
Run git @("fetch", "--tags", "--quiet")
if (git tag --list $tag) { Fail "Tag $tag already exists." }

Write-Host "Releasing $current -> $Version" -ForegroundColor Cyan

# --- Bump --------------------------------------------------------------------
$start = $m.Groups[1].Index
$toml = $toml.Substring(0, $start) + $Version + $toml.Substring($start + $current.Length)
[System.IO.File]::WriteAllText((Resolve-Path Cargo.toml), $toml)
Run cargo @("update", "--workspace", "--quiet")   # updates Cargo.lock

if (-not $SkipTests) {
    Write-Host "Running tests ..." -ForegroundColor Cyan
    & cargo test --workspace --quiet
    if ($LASTEXITCODE -ne 0) {
        git checkout -- Cargo.toml Cargo.lock
        Fail "Tests failed - version change undone."
    }
}

# --- Commit, tag, push -------------------------------------------------------
Run git @("add", "Cargo.toml", "Cargo.lock")
Run git @("commit", "--quiet", "-m", "Version $Version")
Run git @("tag", "-a", $tag, "-m", "DMXCraft $Version")
Run git @("push", "--quiet", "origin", "main")
Run git @("push", "--quiet", "origin", $tag)

$repo = "https://github.com/JustinSchweiger/DMXCraft"
Write-Host ""
Write-Host "Released $tag." -ForegroundColor Green
Write-Host "Builds:  $repo/actions"
Write-Host "Release: $repo/releases/tag/$tag (ready in about 15-20 minutes)"
