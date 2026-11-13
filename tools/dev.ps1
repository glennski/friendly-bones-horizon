param(
    [ValidateSet('test','package','install')][string]$Action='test',
    [string]$Launcher='G:\Program Files\Horizon Launcher\horizon-launcher.exe',
    [string]$Instance='Friendly Bones Test',
    [ValidateSet('performance','dev','release')][string]$Profile='performance',
    [switch]$TrustNativeCode
)
# test:    the engine-independent core tests with the launcher's Rust, plus the payload freshness check.
# package: dist/friendlybones_<version>_....hnmod (native code) and dist/friendlybones-content_<version>_....hmod (content).
# install: package, copy both into the test instance (horizon-native-mods/, horizon-mods/), turn the mod on and build the game for it
#          (the launcher's native build; needs -TrustNativeCode because the mod's code is compiled and run).
$ErrorActionPreference='Stop'
$project=(Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..')).Path
function Invoke-Checked { param([string]$What, [scriptblock]$Run) & $Run; if ($LASTEXITCODE -ne 0) { throw "$What failed: $LASTEXITCODE" } }
if ($Action -eq 'test') {
    $rust=(& $Launcher rust --json status | Out-String | ConvertFrom-Json).installed
    if (!$rust) { throw "Rust is missing. Run: & '$Launcher' rust install" }
    $previous=@{PATH=$env:PATH; CARGO_TARGET_DIR=$env:CARGO_TARGET_DIR}
    try {
        $env:PATH="$rust\bin;$rust\mingw64\bin;$env:PATH"
        $env:CARGO_TARGET_DIR=Join-Path $project '.build\target'
        Invoke-Checked 'payload check' { python (Join-Path $PSScriptRoot 'assemble.py') --check }
        Invoke-Checked 'core tests' { cargo test --manifest-path (Join-Path $project 'Cargo.toml') }
    } finally { $env:PATH=$previous.PATH; $env:CARGO_TARGET_DIR=$previous.CARGO_TARGET_DIR }
    return
}
Invoke-Checked 'assemble' { python (Join-Path $PSScriptRoot 'assemble.py') }
Invoke-Checked 'package' { python (Join-Path $PSScriptRoot 'package.py') }
if ($Action -eq 'install') {
    if (!$TrustNativeCode) { throw 'Read mixins/friendlybones and pass -TrustNativeCode to build it into the game.' }
    $found=(& $Launcher instance list --json | Out-String | ConvertFrom-Json)
    $dir=$null
    foreach ($i in @($found)) { if ($i.name -eq $Instance) { $dir=$i.home } }
    if (!$dir) { Invoke-Checked 'instance create' { & $Launcher instance create $Instance }; $found=(& $Launcher instance list --json | Out-String | ConvertFrom-Json); foreach ($i in @($found)) { if ($i.name -eq $Instance) { $dir=$i.home } } }
    if (!$dir) { throw "Instance '$Instance' not found." }
    $native=Join-Path $dir 'horizon-native-mods'
    $content=Join-Path $dir 'horizon-mods'
    New-Item -ItemType Directory -Path $native,$content -Force | Out-Null
    Get-ChildItem -LiteralPath $native -Filter 'friendlybones*' | Remove-Item -Force
    Get-ChildItem -LiteralPath $content -Filter 'friendlybones*' | Remove-Item -Force
    Copy-Item -LiteralPath (Get-ChildItem (Join-Path $project 'dist') -Filter '*.hnmod').FullName -Destination (Join-Path $native 'friendlybones.hnmod')
    Copy-Item -LiteralPath (Get-ChildItem (Join-Path $project 'dist') -Filter '*.hmod').FullName -Destination (Join-Path $content 'friendlybones.hmod')
    $list=Join-Path $dir 'horizon-mods.txt'
    if (!(Test-Path -LiteralPath $list) -or !(Select-String -LiteralPath $list -Pattern '^mod\.friendlybones:true' -Quiet)) { Add-Content -LiteralPath $list -Value 'mod.friendlybones:true' }
    Invoke-Checked 'native build' { & $Launcher native-build --instance $Instance --profile $Profile --trust-native-code }
}
