$ErrorActionPreference = 'Stop'
[Console]::OutputEncoding = New-Object System.Text.UTF8Encoding($false)
$dialog = $null
$exitCode = 2
try {
    Add-Type -AssemblyName System.Windows.Forms
    $start = $env:STAMP_PICKER_START
    $directory = $null
    if (Test-Path -LiteralPath $start -PathType Container) {
        $directory = (Get-Item -LiteralPath $start).FullName
    } elseif (Test-Path -LiteralPath $start -PathType Leaf) {
        $directory = (Get-Item -LiteralPath $start).DirectoryName
    }
    if ($env:STAMP_PICKER_KIND -eq 'folder') {
        $dialog = New-Object System.Windows.Forms.FolderBrowserDialog
        $dialog.Description = $env:STAMP_PICKER_TITLE
        $dialog.ShowNewFolderButton = $true
        if ($directory) {
            $dialog.SelectedPath = $directory
        }
    } else {
        $dialog = New-Object System.Windows.Forms.OpenFileDialog
        $dialog.Title = $env:STAMP_PICKER_TITLE
        $dialog.CheckFileExists = $true
        $dialog.Multiselect = $false
        $dialog.RestoreDirectory = $true
        if ($directory) {
            $dialog.InitialDirectory = $directory
        }
        switch ($env:STAMP_PICKER_KIND) {
            'csv' { $dialog.Filter = 'Colnect CSV (*.csv)|*.csv' }
            'stampworld' { $dialog.Filter = 'StampWorld PDF/HTML (*.pdf;*.html;*.htm)|*.pdf;*.html;*.htm' }
            'executable' {
                $dialog.Filter = 'Executables (*.exe;*.com;*.bat;*.cmd)|*.exe;*.com;*.bat;*.cmd|All files (*.*)|*.*'
                if (Test-Path -LiteralPath $start -PathType Leaf) {
                    $dialog.FileName = $start
                }
            }
            default { throw 'Unsupported Windows picker kind' }
        }
    }
    if ($dialog.ShowDialog() -eq [System.Windows.Forms.DialogResult]::OK) {
        $selected = if ($env:STAMP_PICKER_KIND -eq 'folder') {
            $dialog.SelectedPath
        } else {
            $dialog.FileName
        }
        [Console]::WriteLine((ConvertTo-Json -InputObject $selected -Compress))
        $exitCode = 0
    } else {
        $exitCode = 10
    }
} catch {
    [Console]::Error.WriteLine($_.Exception.Message)
} finally {
    if ($dialog) {
        $dialog.Dispose()
    }
}
exit $exitCode
