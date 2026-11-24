<#
.SYNOPSIS
    Syncs AlbumEasy / native PDF fixtures from StampAlbumGen2 to stampalbumgenerator2-site.

.DESCRIPTION
    Copies PDFs and source files (.txt, .stampalbum) with appropriate naming conventions,
    updates manifest.json, generates thumbnails, and recompiles examples.html.

.PARAMETER FixtureId
    Specific fixture folder name (e.g. "margins-v1", "album-author-v1").
    If omitted or "all", all fixtures in tests/fixtures/albumeasy are synced.

.PARAMETER SiteRepoPath
    Path to the stampalbumgenerator2-site repository.
    Auto-detects ../StampAlbumGen2-website, ../stampalbumgenerator2-site, or Windows defaults.

.EXAMPLE
    .\scripts\sync_fixture_to_site.ps1 -FixtureId margins-v1
    .\scripts\sync_fixture_to_site.ps1 -FixtureId album-author-v1
    .\scripts\sync_fixture_to_site.ps1 -FixtureId all
#>
param(
    [Parameter(Position = 0)]
    [string]$FixtureId = "all",

    [Parameter(Position = 1)]
    [string]$SiteRepoPath = ""
)

$ErrorActionPreference = "Stop"

$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$FixturesSourceRoot = Join-Path $RepoRoot "tests/fixtures/albumeasy"
if (-not (Test-Path $FixturesSourceRoot)) {
    $FixturesSourceRoot = Join-Path $RepoRoot "tests\fixtures\albumeasy"
}

if (-not (Test-Path $FixturesSourceRoot)) {
    throw "Cannot find fixtures directory at $FixturesSourceRoot"
}

# Auto-detect website repo path if not explicitly provided or not found
if ([string]::IsNullOrWhiteSpace($SiteRepoPath) -or -not (Test-Path $SiteRepoPath)) {
    $candidatePaths = @(
        (Join-Path $RepoRoot "..\StampAlbumGen2-website"),
        (Join-Path $RepoRoot "..\stampalbumgenerator2-site"),
        (Join-Path $RepoRoot "../StampAlbumGen2-website"),
        (Join-Path $RepoRoot "../stampalbumgenerator2-site"),
        "C:\Users\ray1\repo\stampalbumgenerator2-site",
        "C:\Users\ray1\repo\StampAlbumGen2-website"
    )
    foreach ($cand in $candidatePaths) {
        if (Test-Path $cand) {
            $SiteRepoPath = (Resolve-Path $cand).Path
            break
        }
    }
}

if (-not (Test-Path $SiteRepoPath)) {
    throw "Cannot find website repository at $SiteRepoPath"
}

$SiteFixturesDir = Join-Path $SiteRepoPath "static/pages/fixtures"
$SitePdfsDir = Join-Path $SiteFixturesDir "pdfs"
$SiteSourceDir = Join-Path $SiteFixturesDir "source"
$SiteThumbsDir = Join-Path $SiteFixturesDir "thumbs"
$ManifestPath = Join-Path $SiteFixturesDir "manifest.json"

# Ensure target directories exist
$null = New-Item -ItemType Directory -Force -Path $SitePdfsDir
$null = New-Item -ItemType Directory -Force -Path $SiteSourceDir
$null = New-Item -ItemType Directory -Force -Path $SiteThumbsDir

# Identify fixtures to sync
$targetFixtures = @()
if ($FixtureId -eq "all" -or [string]::IsNullOrWhiteSpace($FixtureId)) {
    $targetFixtures = Get-ChildItem -Path $FixturesSourceRoot -Directory | Select-Object -ExpandProperty Name
} else {
    $targetPath = Join-Path $FixturesSourceRoot $FixtureId
    if (-not (Test-Path $targetPath)) {
        throw "Fixture '$FixtureId' does not exist in $FixturesSourceRoot"
    }
    $targetFixtures = @($FixtureId)
}

Write-Host "========================================================" -ForegroundColor Cyan
Write-Host "Syncing $($targetFixtures.Count) fixture(s) to website..." -ForegroundColor Cyan
Write-Host "Source: $FixturesSourceRoot" -ForegroundColor Gray
Write-Host "Target: $SiteFixturesDir" -ForegroundColor Gray
Write-Host "========================================================" -ForegroundColor Cyan

# Load existing manifest
$manifest = $null
if (Test-Path $ManifestPath) {
    try {
        $manifest = Get-Content -Path $ManifestPath -Raw -Encoding UTF8 | ConvertFrom-Json
    } catch {
        Write-Warning "Could not parse existing manifest.json; creating new one."
        $manifest = [PSCustomObject]@{}
    }
} else {
    $manifest = [PSCustomObject]@{}
}

$pdfCopyCount = 0
$srcCopyCount = 0

foreach ($fix in $targetFixtures) {
    $srcDir = Join-Path $FixturesSourceRoot $fix

    # 1. Copy PDFs
    $pdfs = Get-ChildItem -Path $srcDir -Filter "*.pdf" -File -ErrorAction SilentlyContinue
    foreach ($pdf in $pdfs) {
        $destName = "$fix--$($pdf.Name)"
        $destPath = Join-Path $SitePdfsDir $destName
        Copy-Item -Path $pdf.FullName -Destination $destPath -Force
        $pdfCopyCount++
    }

    # 2. Copy source files (.txt, .stampalbum)
    $sources = Get-ChildItem -Path $srcDir -Include "*.txt", "*.stampalbum" -File -Recurse -ErrorAction SilentlyContinue
    foreach ($src in $sources) {
        # Only copy top-level fixture sources, skip internal/nested if needed
        if ($src.DirectoryName -eq $srcDir) {
            $destName = "$fix--$($src.Name)"
            $destPath = Join-Path $SiteSourceDir $destName
            Copy-Item -Path $src.FullName -Destination $destPath -Force
            $srcCopyCount++
        }
    }

    # Generate missing thumbnails for this fixture using pdftoppm if available
    $pdftoppmCmd = Get-Command pdftoppm -ErrorAction SilentlyContinue
    if ($pdftoppmCmd) {
        $syncedPdfs = Get-ChildItem -Path $SitePdfsDir -Filter "$fix--*.pdf" -File
        foreach ($p in $syncedPdfs) {
            $baseStem = [System.IO.Path]::GetFileNameWithoutExtension($p.Name)
            $thumbDest = Join-Path $SiteThumbsDir "$baseStem.png"
            if (-not (Test-Path $thumbDest)) {
                $thumbPrefix = Join-Path $SiteThumbsDir $baseStem
                & pdftoppm -png -scale-to-x 400 -scale-to-y -1 -f 1 -singlefile $p.FullName $thumbPrefix 2>$null
            }
        }
    }

    # 3. Update manifest entry for this fixture
    $fixPdfs = Get-ChildItem -Path $SitePdfsDir -Filter "$fix--*.pdf" -File | Sort-Object Name
    $fixSources = Get-ChildItem -Path $SiteSourceDir -Filter "$fix--*" -File | Sort-Object Name
    $fixThumbs = Get-ChildItem -Path $SiteThumbsDir -Filter "$fix--*.png" -File | Sort-Object Name

    $pdfList = @()
    foreach ($p in $fixPdfs) {
        $origName = $p.Name.Substring("$fix--".Length)
        $pdfList += [PSCustomObject]@{
            name = $origName
            web_path = "/pages/fixtures/pdfs/$($p.Name)"
        }
    }

    $srcList = @()
    foreach ($s in $fixSources) {
        $origName = $s.Name.Substring("$fix--".Length)
        $srcList += [PSCustomObject]@{
            name = $origName
            web_path = "/pages/fixtures/source/$($s.Name)"
        }
    }

    $thumbList = @()
    foreach ($t in $fixThumbs) {
        $origName = $t.Name.Substring("$fix--".Length)
        $thumbList += [PSCustomObject]@{
            name = $origName
            web_path = "/static/pages/fixtures/thumbs/$($t.Name)"
        }
    }

    $entry = [PSCustomObject]@{
        fixture = $fix
        pdfs = $pdfList
        thumbs = $thumbList
        sources = $srcList
    }

    if ($manifest.PSObject.Properties[$fix]) {
        $manifest.PSObject.Properties[$fix].Value = $entry
    } else {
        $manifest | Add-Member -MemberType NoteProperty -Name $fix -Value $entry
    }

    Write-Host "  -> Synced $fix ($($pdfList.Count) PDFs, $($srcList.Count) sources, $($thumbList.Count) thumbs)" -ForegroundColor Green
}

# Save updated manifest
$manifest | ConvertTo-Json -Depth 5 | Set-Content -Path $ManifestPath -Encoding UTF8
Write-Host "Updated manifest: $ManifestPath" -ForegroundColor Cyan

# Determine PowerShell command name
$psCmd = if (Get-Command pwsh -ErrorAction SilentlyContinue) { "pwsh" } else { "powershell" }

# 4. Generate thumbnails on Windows via WinRT if pdftoppm was not used
if (-not (Get-Command pdftoppm -ErrorAction SilentlyContinue)) {
    $thumbScript = Join-Path $SiteRepoPath "generate_all_thumbs.ps1"
    if (Test-Path $thumbScript) {
        Write-Host "`nRunning thumbnail generator ($thumbScript)..." -ForegroundColor Yellow
        & $psCmd -ExecutionPolicy Bypass -File $thumbScript
    }
}

# 5. Regenerate examples.html gallery page
$pageScript = Join-Path $SiteRepoPath "generate_examples_page.ps1"
if (Test-Path $pageScript) {
    Write-Host "`nRegenerating examples.html ($pageScript)..." -ForegroundColor Yellow
    $siteStatic = Join-Path $SiteRepoPath "static"
    & $psCmd -ExecutionPolicy Bypass -File $pageScript -SiteRoot $siteStatic
}

Write-Host "`n[SUCCESS] Sync completed: $pdfCopyCount PDFs and $srcCopyCount sources synced to website!" -ForegroundColor Green
