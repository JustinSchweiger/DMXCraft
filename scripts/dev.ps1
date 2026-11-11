# Pull the latest main, stop DMXCraft processes that still run from this
# build folder (they lock target\release\*.exe on Windows) and start the app.
#
# Usage (from the repository root):  .\scripts\dev.ps1 [show.dmxcraft]

$ErrorActionPreference = "Stop"
Set-Location (Split-Path $PSScriptRoot -Parent)

git pull
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

$target = Join-Path (Get-Location) "target"
Get-Process dmxcraft, dmxcraft-mcp -ErrorAction SilentlyContinue |
    Where-Object { $_.Path -and $_.Path.StartsWith($target, "OrdinalIgnoreCase") } |
    ForEach-Object {
        Write-Host "Stopping $($_.Name) (PID $($_.Id)): $($_.Path)" -ForegroundColor Yellow
        Stop-Process -Id $_.Id -Force
        $_.WaitForExit(5000) | Out-Null
    }

# Build both programs: dmxcraft and the small dmxcraft-mcp bridge (DMXCraft
# copies the bridge into its config folder for Claude Desktop).
cargo build --release -p dmxcraft --bins
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
cargo run --release -p dmxcraft -- @args
