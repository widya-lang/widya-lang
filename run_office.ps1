# Peluncur Otomatis WidyaOffice Desktop
$Exe = Join-Path $PSScriptRoot "dist_desktop\WidyaOffice Desktop_windows\widya_office_desktop.exe"
Write-Host "Meluncurkan WidyaOffice Desktop dari: $Exe" -ForegroundColor Cyan
Start-Process $Exe -ArgumentList "--gui"
