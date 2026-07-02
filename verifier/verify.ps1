$ErrorActionPreference = "Stop"

$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$decoderCandidates = @(
    (Join-Path $root "1_VERIFY\mol_decoder.exe"),
    (Join-Path $root "target\release\mol_decoder.exe")
)
$decoder = $decoderCandidates | Where-Object { Test-Path -LiteralPath $_ } | Select-Object -First 1
$samples = Join-Path $root "1_VERIFY\samples"
$expectedPath = Join-Path $root "1_VERIFY\expected_sha256.txt"
$decodedRoot = Join-Path $root "2_RESULTS\decoded"

if (-not $decoder) {
    Write-Host "[ERROR] Decoder binary not found."
    Write-Host "        Looked for:"
    $decoderCandidates | ForEach-Object { Write-Host "        $_" }
    Write-Host "        Run: cargo build --release"
    exit 1
}

if (-not (Test-Path -LiteralPath $samples)) {
    Write-Host "[ERROR] samples directory missing"
    exit 1
}

if (-not (Test-Path -LiteralPath $expectedPath)) {
    Write-Host "[ERROR] expected_sha256.txt missing"
    exit 1
}

$expected = @{}
Get-Content -LiteralPath $expectedPath | ForEach-Object {
    $line = $_.Trim()
    if ($line -eq "" -or $line.StartsWith("#")) { return }
    $parts = $line -split "\s+"
    if ($parts.Count -ge 2) {
        $expected[$parts[1]] = $parts[0].ToLowerInvariant()
    }
}

function Get-Sha256Hex {
    param([Parameter(Mandatory = $true)][string]$Path)
    $sha = [System.Security.Cryptography.SHA256]::Create()
    $stream = [System.IO.File]::OpenRead($Path)
    try {
        $hash = $sha.ComputeHash($stream)
    } finally {
        $stream.Dispose()
        $sha.Dispose()
    }
    return (($hash | ForEach-Object { $_.ToString("x2") }) -join "")
}

New-Item -ItemType Directory -Force -Path $decodedRoot | Out-Null

$pass = 0
$fail = 0

Write-Host "Empire MOL decoder verification"
Write-Host "Decoder: $decoder"
Write-Host ""

Get-ChildItem -LiteralPath $samples -Filter "*.mol" | Sort-Object Name | ForEach-Object {
    $archive = $_
    $base = [System.IO.Path]::GetFileNameWithoutExtension($archive.Name)
    $outDir = Join-Path $decodedRoot $base
    if (Test-Path -LiteralPath $outDir) {
        Remove-Item -LiteralPath $outDir -Recurse -Force
    }
    New-Item -ItemType Directory -Force -Path $outDir | Out-Null

    Write-Host "Decoding $($archive.Name)"
    & $decoder $archive.FullName $outDir | Out-Null
    if ($LASTEXITCODE -ne 0) {
        Write-Host "  FAIL: decoder rejected archive"
        $script:fail += 1
        return
    }

    $files = @(Get-ChildItem -LiteralPath $outDir -File)
    if ($files.Count -ne 1) {
        Write-Host "  FAIL: expected one restored file, found $($files.Count)"
        $script:fail += 1
        return
    }

    $decoded = $files[0]
    if (-not $expected.ContainsKey($decoded.Name)) {
        Write-Host "  FAIL: missing expected SHA256 for $($decoded.Name)"
        $script:fail += 1
        return
    }

    $actual = Get-Sha256Hex -Path $decoded.FullName
    if ($actual -eq $expected[$decoded.Name]) {
        Write-Host "  PASS: $($decoded.Name)"
        $script:pass += 1
    } else {
        Write-Host "  FAIL: $($decoded.Name) SHA256 mismatch"
        Write-Host "        expected: $($expected[$decoded.Name])"
        Write-Host "        actual:   $actual"
        $script:fail += 1
    }
}

Write-Host ""
Write-Host "Results: $pass passed, $fail failed"
if ($fail -gt 0) { exit 2 }
exit 0
