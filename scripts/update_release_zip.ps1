Add-Type -AssemblyName System.IO.Compression.FileSystem

$zipPath = "dist/releases/v1.0.0/widya-v1.0.0-windows-x64.zip"
$pdfPath = "Buku_Panduan_Pemrograman_Widya_Lang.pdf"

Write-Host "Membuka zip archive..."
$zip = [System.IO.Compression.ZipFile]::Open($zipPath, [System.IO.Compression.ZipArchiveMode]::Update)

$entry = $zip.GetEntry("Buku_Panduan_Pemrograman_Widya_Lang.pdf")
if ($entry) {
    Write-Host "Menghapus entry PDF lama dari zip..."
    $entry.Delete()
}

Write-Host "Menambahkan Buku_Panduan_Pemrograman_Widya_Lang.pdf baru ke zip..."
[System.IO.Compression.ZipFileExtensions]::CreateEntryFromFile($zip, $pdfPath, "Buku_Panduan_Pemrograman_Widya_Lang.pdf", [System.IO.Compression.CompressionLevel]::Optimal)

$zip.Dispose()
Write-Host "Zip archive berhasil diperbarui!"
