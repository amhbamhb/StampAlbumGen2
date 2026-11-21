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
    Defaults to "C:\Users\ray1\repo\stampalbumgenerator2-site".

.EXAMPLE
    .\scripts\sync_fixture_to_site.ps1 -FixtureId margins-v1
    .\scripts\sync_fixture_to_site.ps1 -FixtureId album-author-v1
    .\scripts\sync_fixture_to_site.ps1 -FixtureId all
#>
param(
    [Parameter(Position = 0)]
    [string]$FixtureId = "all",

    [Parameter(Position = 1)]
    [string]$SiteRepoPath = "C:\Users\ray1\repo\stampalbumgenerator2-site"
)

$ErrorActionPreference = "Stop"

$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$FixturesSourceRoot = Join-Path $RepoRoot "tests\fixtures\albumeasy"

if (-not (Test-Path $FixturesSourceRoot)) {
    throw "Cannot find fixtures directory at $FixturesSourceRoot"
}

if (-not (Test-Path $SiteRepoPath)) {
    throw "Cannot find website repository at $SiteRepoPath"
}

$SiteFixturesDir = Join-Path $SiteRepoPath "static\pages\fixtures"
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

    Write-Host "  -> Synced $fix ($($pdfList.Count) PDFs, $($srcList.Count) sources)" -ForegroundColor Green
}

# Save updated manifest
$manifest | ConvertTo-Json -Depth 5 | Set-Content -Path $ManifestPath -Encoding UTF8
Write-Host "Updated manifest: $ManifestPath" -ForegroundColor Cyan

# 4. Generate thumbnails for any new PDFs
$thumbScript = Join-Path $SiteRepoPath "generate_all_thumbs.ps1"
if (Test-Path $thumbScript) {
    Write-Host "`nRunning thumbnail generator ($thumbScript)..." -ForegroundColor Yellow
    & powershell -ExecutionPolicy Bypass -File $thumbScript
}

# 5. Regenerate examples.html gallery page
$pageScript = Join-Path $SiteRepoPath "generate_examples_page.ps1"
if (Test-Path $pageScript) {
    Write-Host "`nRegenerating examples.html ($pageScript)..." -ForegroundColor Yellow
    & powershell -ExecutionPolicy Bypass -File $pageScript
}

Write-Host "`n[SUCCESS] Sync completed: $pdfCopyCount PDFs and $srcCopyCount sources synced to website!" -ForegroundColor Green
