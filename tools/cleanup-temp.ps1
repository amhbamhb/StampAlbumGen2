<#
.SYNOPSIS
  Safe disk cleanup script for Windows: temp files, installer leftovers, and developer caches.

.DESCRIPTION
  Default behavior is a dry-run: it reports sizes and what it would remove. Use -DryRun:$false and/or -Force to actually delete.
  Use -IncludeDevCaches to clear developer caches (npm, pip, NuGet, Maven) where available.

.EXAMPLE
  # Dry-run (default)
  .\cleanup-temp.ps1

.EXAMPLE
  # Perform deletion and include developer caches (prompts unless -Force)
  .\cleanup-temp.ps1 -DryRun:$false -IncludeDevCaches -Force
#>

[CmdletBinding()]
param(
    [switch]
    $DryRun,

    [switch]
    $Force,

    [switch]
    $IncludeDevCaches,

    [string]
    $LogFile = "$PSScriptRoot\cleanup-temp.log"
)

# Default DryRun to true when not specified; allow bare switches for flags
if (-not $PSBoundParameters.ContainsKey('DryRun')) { $DryRun = $true } else { $DryRun = [bool]$DryRun }
if (-not $PSBoundParameters.ContainsKey('Force')) { $Force = $false } else { $Force = [bool]$Force }
if (-not $PSBoundParameters.ContainsKey('IncludeDevCaches')) { $IncludeDevCaches = $false } else { $IncludeDevCaches = [bool]$IncludeDevCaches }

function Write-Log {
    param($Message)
    $timestamp = (Get-Date).ToString('yyyy-MM-dd HH:mm:ss')
    $line = "[$timestamp] $Message"
    Write-Output $line
    try { Add-Content -Path $LogFile -Value $line -ErrorAction SilentlyContinue } catch {}
}

function Get-FolderSize([string]$Path) {
    if (-not (Test-Path -LiteralPath $Path)) { return 0 }
    try {
        $items = Get-ChildItem -LiteralPath $Path -Recurse -Force -ErrorAction Stop
        return ($items | Where-Object { -not $_.PSIsContainer } | Measure-Object -Property Length -Sum).Sum
    } catch {
        return -1
    }
}

function Format-Size([long]$bytes) {
    if ($bytes -lt 0) { return "Unknown" }
    if ($bytes -ge 1GB) { "{0:N2} GB" -f ($bytes/1GB) }
    elseif ($bytes -ge 1MB) { "{0:N2} MB" -f ($bytes/1MB) }
    elseif ($bytes -ge 1KB) { "{0:N2} KB" -f ($bytes/1KB) }
    else { "$bytes bytes" }
}

function Confirm-Delete([string]$Path, [long]$Size) {
    if ($Force) { return $true }
    $sizeText = Format-Size $Size
    $msg = "Delete $Path (approx $sizeText)?"
    return (Read-Host "$msg [Y/N]") -match '^[Yy]'
}

# Determine admin rights
function Test-IsAdmin {
    try { (New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator) } catch { $false }
}

$admin = Test-IsAdmin
if (-not $admin) { Write-Log "Info: Running without admin privileges; some system locations will be skipped." }

# Retry config for removing locked files
$MaxRemoveRetries = 3
$RemoveRetryDelaySec = 1

# Candidate paths to clean
$targets = @()

# User and system temps
$envTemp = $env:TEMP
if ($envTemp) { $targets += $envTemp }
$localTemp = "$env:LOCALAPPDATA\Temp"
if ($localTemp -and -not ($targets -contains $localTemp)) { $targets += $localTemp }
$systemTemp = "$env:SystemRoot\Temp"
if ($systemTemp) { $targets += $systemTemp }

# Windows update / software distribution (requires admin)
$wuPath = "$env:SystemRoot\SoftwareDistribution\Download"
if ($wuPath) { $targets += $wuPath }

# Chocolatey temp
$chocoTemp = "$env:ProgramData\chocolatey\lib\_temp"
if ($chocoTemp) { $targets += $chocoTemp }

# Common installer temp: ProgramData/Package Cache is sensitive - skip; use caution

# Developer caches (optional)
if ($IncludeDevCaches) {
    $npmCache = "$env:APPDATA\npm-cache"
    $yarnCache = "$env:LOCALAPPDATA\Yarn\Cache"
    $pipCache = "$env:LOCALAPPDATA\pip\Cache"
    $m2Repo = "$env:USERPROFILE\.m2\repository"
    $nugetRoot = "$env:USERPROFILE\.nuget\packages"
    $targets += @($npmCache, $yarnCache, $pipCache, $m2Repo, $nugetRoot) | Where-Object { $_ }
}

# Deduplicate and keep only existing paths
$targets = $targets | Where-Object { $_ -and (Test-Path -LiteralPath $_) } | Sort-Object -Unique

if ($targets.Count -eq 0) {
    Write-Log "No candidate cleanup paths exist on this system. Exiting."
    return
}

Write-Log "Cleanup run started. DryRun=$DryRun; IncludeDevCaches=$IncludeDevCaches; Force=$Force"

foreach ($path in $targets) {
    $size = Get-FolderSize -Path $path
    $sizeText = Format-Size $size

    # Skip some system dirs if not admin
    if (($path -like "$env:SystemRoot*" -or $path -like "C:\Windows*") -and -not $admin) {
        Write-Log "Skipping system path (requires admin): $path (approx $sizeText)"
        continue
    }

    Write-Log "Found: $path (approx $sizeText)"

    if ($DryRun) {
        Write-Log "Dry-run: would remove contents of $path"
        continue
    }

    # Confirm
    if (-not (Confirm-Delete -Path $path -Size $size)) {
        Write-Log "User declined deletion of $path"
        continue
    }

    try {
        # Remove children but keep folder itself to avoid breaking system paths
        function Remove-ItemWithRetries {
            param($Item)
            $name = $null
            try { $name = $Item.FullName } catch { $name = $Item.ToString() }

            for ($i = 0; $i -lt $MaxRemoveRetries; $i++) {
                try {
                    if ($Item -is [System.IO.FileInfo]) {
                        if ($Item.Attributes -band [System.IO.FileAttributes]::ReadOnly) {
                            try { $Item.Attributes = $Item.Attributes -bxor [System.IO.FileAttributes]::ReadOnly } catch {}
                        }
                        Remove-Item -LiteralPath $name -Force -ErrorAction Stop
                    } elseif ($Item -is [System.IO.DirectoryInfo]) {
                        # Clear readonly on all children first
                        try { Get-ChildItem -LiteralPath $name -Recurse -Force -ErrorAction SilentlyContinue | ForEach-Object { if ($_.Attributes -band [System.IO.FileAttributes]::ReadOnly) { try { $_.Attributes = $_.Attributes -bxor [System.IO.FileAttributes]::ReadOnly } catch {} } } } catch {}
                        Remove-Item -LiteralPath $name -Recurse -Force -ErrorAction Stop
                    } else {
                        # fallback for strings
                        Remove-Item -LiteralPath $name -Recurse -Force -ErrorAction Stop
                    }
                    Write-Log ("Removed: {0}" -f $name)
                    return $true
                } catch {
                    $err = $_.Exception.Message
                    Write-Log ("Remove attempt {0} failed for {1}: {2}" -f ($i+1), $name, $err)
                    Start-Sleep -Seconds $RemoveRetryDelaySec
                }
            }
            Write-Log ("Giving up on: {0}" -f $name)
            return $false
        }

        Get-ChildItem -LiteralPath $path -Force -ErrorAction Stop | ForEach-Object { Remove-ItemWithRetries $_ }
        Write-Log "Removed contents of $path"
    } catch {
        Write-Log ("Error cleaning {0}: {1}" -f $path, $_.Exception.Message)
    }
}

# Developer-specific clear commands where available
if (-not $DryRun -and $IncludeDevCaches) {
    # dotnet/nuget locals
    if (Get-Command dotnet -ErrorAction SilentlyContinue) {
        try {
            Write-Log "Running: dotnet nuget locals all --clear"
            if (-not $DryRun) { dotnet nuget locals all --clear | Out-Null }
            Write-Log "dotnet NuGet locals cleared"
        } catch { Write-Log ("dotnet nuget clear failed: {0}" -f $_.Exception.Message) }
    }

    # npm cache (prefer programmatic folder deletion already attempted)
    if (Get-Command npm -ErrorAction SilentlyContinue) {
        try {
            Write-Log "Attempting: npm cache clean --force"
            npm cache clean --force 2>$null
            Write-Log "npm cache clean attempted"
        } catch { Write-Log ("npm cache clean failed: {0}" -f $_.Exception.Message) }
    }

    # pip cache via python -m pip
    if (Get-Command python -ErrorAction SilentlyContinue) {
        try {
            Write-Log "Attempting: python -m pip cache purge"
            python -m pip cache purge 2>$null
            Write-Log "pip cache purge attempted"
        } catch { Write-Log ("pip cache purge failed: {0}" -f $_.Exception.Message) }
    }
}

Write-Log "Cleanup run completed. Log: $LogFile"

Write-Output "Done. Log written to: $LogFile"
