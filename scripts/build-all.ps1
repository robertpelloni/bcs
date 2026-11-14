<#
.SYNOPSIS
  One-command build/check for every BTK component, in value order.

.DESCRIPTION
  Modes:
    -CheckOnly  Fast, no compilation: verifies every entry point exists
                and every toolchain is present. This is the maintained
                layout regression check for repo entry points.
    (default)   Builds the language ports and, if the VS2019 tree is
                configured, the C++ tools. Heavy processes are started
                BelowNormal priority so the machine stays responsive.

  Exit code is non-zero if any component that was attempted fails.
  A missing toolchain for an optional port is reported, not fatal,
  unless -RequireAll is given.
#>
[CmdletBinding()]
param(
    [switch]$CheckOnly,
    [switch]$RequireAll
)

$ErrorActionPreference = 'Continue'
$RepoRoot = Split-Path -Parent $PSScriptRoot
$failures = @()
$skipped  = @()

function Test-Tool([string]$Name) {
    $cmd = Get-Command $Name -ErrorAction SilentlyContinue
    if ($cmd) { Write-Host "  [ok]      $Name -> $($cmd.Source)"; return $true }
    Write-Host "  [missing] $Name"; return $false
}

function Invoke-BelowNormal([string]$File, [string[]]$ArgList, [string]$WorkDir, [string]$Label) {
    Write-Host "  building $Label (BelowNormal priority)..."
    # Diagnostics.Process (not Start-Process) so ExitCode stays readable
    # after WaitForExit; Start-Process loses it once the handle closes.
    $psi = [System.Diagnostics.ProcessStartInfo]::new()
    $psi.FileName = $File
    $psi.WorkingDirectory = $WorkDir
    $psi.UseShellExecute = $false
    # PS 5.1 (.NET Framework) has no ArgumentList; use the quoted string form.
    $psi.Arguments = ($ArgList | ForEach-Object { if ($_ -match '\s') { "`"$_`"" } else { $_ } }) -join ' '
    $p = [System.Diagnostics.Process]::Start($psi)
    try { $p.PriorityClass = 'BelowNormal' } catch { }
    $p.WaitForExit()
    if ($p.ExitCode -ne 0) { $script:failures += "$Label (exit $($p.ExitCode))" }
    else { Write-Host "  [ok] $Label" }
}

Write-Host "== BTK repo layout check =="
$entryPoints = @(
    'CMakeLists.txt', 'VERSION', 'start.bat',
    'ports\go\main.go', 'ports\go\go.mod',
    'ports\rust\Cargo.toml', 'ports\rust\main.rs',
    'ports\csharp\Program.cs', 'ports\java\Main.java',
    'src\core\CMakeLists.txt', 'src\gui\CMakeLists.txt',
    'test\core\CMakeLists.txt'
)
foreach ($e in $entryPoints) {
    if (Test-Path (Join-Path $RepoRoot $e)) { Write-Host "  [ok]      $e" }
    else { Write-Host "  [MISSING] $e"; $failures += "missing entry point: $e" }
}

Write-Host "== Toolchains =="
$have = @{}
foreach ($t in 'cmake','go','cargo','dotnet','java','python') { $have[$t] = Test-Tool $t }

if ($CheckOnly) {
    if ($RequireAll -and ($have.Values -contains $false)) { $failures += 'missing toolchain(s)' }
    if ($failures) { Write-Host "`nCHECK FAILED: $($failures -join '; ')"; exit 1 }
    Write-Host "`nCHECK PASSED (layout + entry points)"; exit 0
}

Write-Host "== Ports (value order: Go, Rust, C#, Java) =="
if ($have['go'])    { Invoke-BelowNormal 'go'    @('build','./...') (Join-Path $RepoRoot 'ports\go') 'ports/go' }
else { $skipped += 'ports/go (no go)' }
if ($have['cargo']) { Invoke-BelowNormal 'cargo' @('build') (Join-Path $RepoRoot 'ports\rust') 'ports/rust' }
else { $skipped += 'ports/rust (no cargo)' }
# ports/csharp has no .csproj yet (see TODO.md): report, do not fake a build.
if (Test-Path (Join-Path $RepoRoot 'ports\csharp\*.csproj')) {
    if ($have['dotnet']) { Invoke-BelowNormal 'dotnet' @('build') (Join-Path $RepoRoot 'ports\csharp') 'ports/csharp' }
    else { $skipped += 'ports/csharp (no dotnet)' }
} else { $skipped += 'ports/csharp (no .csproj yet - see TODO.md)' }
$skipped += 'ports/java (no build file yet - see TODO.md)'

Write-Host "== C++ tools (existing build-vs2019 tree) =="
# The checked-in build-vs2019 cache was configured at the old path
# (C:\Users\hyper\workspace\btk); if that cache is stale, cmake refuses.
# Report it honestly instead of deleting the owner's tree.
$vsTree = Join-Path $RepoRoot 'build-vs2019'
$cache = Join-Path $vsTree 'CMakeCache.txt'
$cacheStale = $false
if (Test-Path $cache) {
    $homeDir = Select-String -Path $cache -Pattern '^CMAKE_HOME_DIRECTORY:INTERNAL=(.+)$' |
        ForEach-Object { $_.Matches[0].Groups[1].Value }
    if ($homeDir -and ((Resolve-Path $RepoRoot).Path -ne $homeDir)) { $cacheStale = $true }
}
if ($cacheStale) {
    $skipped += 'C++ build (build-vs2019 cache points at old repo path; reconfigure a fresh build dir - see DEPLOY.md)'
    Write-Host "  [stale]   build-vs2019/CMakeCache.txt was configured at a different repo path; skipping (owner's tree left untouched)"
} elseif ((Test-Path $vsTree) -and $have['cmake']) {
    Invoke-BelowNormal 'cmake' @('--build','build-vs2019','--config','Release','--target','uic','rcc') $RepoRoot 'C++ tools (uic, rcc)'
} else { $skipped += 'C++ build (build-vs2019 not configured or no cmake)' }

Write-Host ""
if ($skipped) { Write-Host "Skipped: $($skipped -join '; ')" }
if ($failures) { Write-Host "FAILED: $($failures -join '; ')"; exit 1 }
Write-Host "BUILD PASSED (all attempted components)"
exit 0
