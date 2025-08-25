# Minimal PowerShell script to compile lib.c to DLL
Write-Host "Compiling lib.c to DLL..."

# Build builder crate with debug symbols
Push-Location "crates\builder"
cargo build
Pop-Location

# Compile C code to DLL with debug symbols
Push-Location "thirdparty"
$comp_args = @("-g", "-shared", "-o", "..\target\debug\libthirdparty.dll", "lib.c", "-lkernel32", "-luser32", "-lws2_32", "-lntdll", "-luserenv", "-ladvapi32", "-lmsvcrt", "-Wl,/NODEFAULTLIB:libcmt")
& clang @comp_args
Pop-Location