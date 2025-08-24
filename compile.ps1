# Minimal PowerShell script to compile lib.c to DLL
Write-Host "Compiling lib.c to DLL..."

# Build builder crate
Push-Location "crates\builder"
cargo build
Pop-Location

# Compile C code to DLL
Push-Location "thirdparty"
$comp_args = @("-shared", "-o", "..\target\debug\libthirdparty.dll", "lib.c", "..\target\debug\builder.lib", "-lkernel32", "-luser32", "-lws2_32", "-lntdll", "-luserenv", "-ladvapi32", "-lmsvcrt", "-Wl,/NODEFAULTLIB:libcmt")
& clang @comp_args
Pop-Location

Write-Host "Done!"