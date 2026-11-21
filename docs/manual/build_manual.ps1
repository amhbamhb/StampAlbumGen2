param(
    [switch]$Open
)

$ErrorActionPreference = "Stop"

$ManualDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Split-Path -Parent (Split-Path -Parent $ManualDir)
$SourceTyp = Join-Path $ManualDir "src\manual.typ"
$ManualPdf = Join-Path $ManualDir "StampAlbumGenerator2HelpManual.pdf"
$RootPdf = Join-Path $RepoRoot "StampAlbumGenerator2HelpManual.pdf"

$TypstCmd = "typst"
$WingetTypst = "C:\Users\ray1\AppData\Local\Microsoft\WinGet\Packages\Typst.Typst_Microsoft.Winget.Source_8wekyb3d8bbwe\typst-x86_64-pc-windows-msvc\typst.exe"
if (Test-Path $WingetTypst) {
    $TypstCmd = $WingetTypst
}

Write-Host "Compiling StampAlbumGen2 Help Manual (Option A) with Typst..." -ForegroundColor Cyan
& $TypstCmd compile $SourceTyp $ManualPdf

if ($LASTEXITCODE -eq 0 -and (Test-Path $ManualPdf)) {
    Write-Host "Manual compiled successfully: $ManualPdf" -ForegroundColor Green
    
    # Keep the root executable deployment artifact updated
    Copy-Item $ManualPdf $RootPdf -Force
    Write-Host "Synchronized embedded executable copy: $RootPdf" -ForegroundColor Green
    
    $fileInfo = Get-Item $RootPdf
    Write-Host "File size: $([math]::Round($fileInfo.Length / 1KB, 1)) KB" -ForegroundColor Yellow

    if ($Open) {
        Write-Host "Opening PDF in default viewer..." -ForegroundColor Cyan
        Start-Process $RootPdf
    }
} else {
    Write-Error "Failed to compile manual with Typst."
}
