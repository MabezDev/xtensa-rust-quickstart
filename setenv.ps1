# For Windows Powershell
# Use with ". .\setenv.ps1"

# change this to the directory of where you built rustc for xtensa
$env:CUSTOM_RUSTC = "G:\rust-xtensa"

$env:RUST_BACKTRACE = "1"
$env:XARGO_RUST_SRC = "$env:CUSTOM_RUSTC\library" # or /src for an older compiler
$env:RUSTC = "$env:CUSTOM_RUSTC\build\x86_64-pc-windows-msvc\stage2\bin\rustc"
$env:RUSTDOC = "$env:CUSTOM_RUSTC\build\x86_64-pc-windows-msvc\stage2\bin\rustdoc"