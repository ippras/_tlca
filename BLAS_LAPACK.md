# BLAS

* [blas-src](https://github.com/blas-lapack-rs/blas-src)
* [openblas-src](https://github.com/blas-lapack-rs/openblas-src)

[vcpkg](https://learn.microsoft.com/ru-ru/vcpkg/get_started/get-started?pivots=shell-powershell)

## Install

`vcpkg install openblas --triplet x64-windows`
`vcpkg install openblas --triplet x64-windows-static-md`
`vcpkg install openblas --triplet x64-windows-static`

## Environment

```sh
$env:VCPKG_ROOT = "C:\Users\*\git\vcpkg"
$env:PATH = "$env:VCPKG_ROOT;$env:PATH"
```

Hi, thx for 
I try `cargo test` or `cargo build` scirs2-stats, but I get an error:

```cmd
error: linking with `link.exe` failed: exit code: 1181
  |
  = note: "C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\VC\\Tools\\MSVC\\14.35.32215\\bin\\HostX64\\x64\\link.exe" "/DEF:C:\\Users\\*\\AppData\\Local\\Temp\\rustch8t0Bb\\lib.def" "/NOLOGO" "C:\\Users\\*\\AppData\\Local\\Temp\\rustch8t0Bb\\symbols.o" "<6 object files omitted>" "C:\\Users\\*\\AppData\\Local\\Temp\\rustch8t0Bb\\rmeta.o" "<1 object files omitted>" "lapack.lib" 
"blas.lib" "C:\\Users\\*\\git\\scirs\\target\\debug\\deps/libvirtue-ee9d2a5028ecb32c.rlib" "<sysroot>\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib/{libproc_macro-*,librustc_literal_escaper-*,librustc_std_workspace_std-*,libstd-*,libpanic_unwind-*,libwindows_targets-*,librustc_demangle-*,libstd_detect-*,libhashbrown-*,librustc_std_workspace_alloc-*,libunwind-*,libcfg_if-*,librustc_std_workspace_core-*,liballoc-*,libcore-*,libcompiler_builtins-*}.rlib" "lapack.lib" "blas.lib" "kernel32.lib" "kernel32.lib" "kernel32.lib" "ntdll.lib" "userenv.lib" "ws2_32.lib" "dbghelp.lib" "/defaultlib:msvcrt" "/NXCOMPAT" "/LIBPATH:/opt/homebrew/opt/lapack/lib" "/LIBPATH:/opt/homebrew/opt/openblas/lib" "/OUT:C:\\Users\\*\\git\\scirs\\target\\debug\\deps\\bincode_derive-b7f8c87dfe913369.dll" "/OPT:REF,NOICF" "/DLL" "/IMPLIB:C:\\Users\\*\\git\\scirs\\target\\debug\\deps\\bincode_derive-b7f8c87dfe913369.dll.lib" "/DEBUG" "/PDBALTPATH:%_PDB%" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\intrinsic.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\liballoc.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libcore.natvis" "/NATVIS:<sysroot>\\lib\\rustlib\\etc\\libstd.natvis"
  = note: some arguments are omitted. use `--verbose` to show all linker arguments
  = note: LINK : fatal error LNK1181: cannot open input file 'lapack.lib'␍


error: could not compile `bincode_derive` (lib) due to 1 previous error

Caused by:
  process didn't exit successfully: `C:\Users\*\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin\rustc.exe --crate-name bincode_derive --edition=2021 C:\Users\*\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\bincode_derive-2.0.1\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=195 --crate-type proc-macro --emit=dep-info,link -C prefer-dynamic -C embed-bitcode=no --check-cfg cfg(docsrs,test) --check-cfg "cfg(feature, values())" -C metadata=99bdaa2b345858ce -C extra-filename=-b7f8c87dfe913369 --out-dir C:\Users\*\git\scirs\target\debug\deps -L dependency=C:\Users\*\git\scirs\target\debug\deps --extern virtue=C:\Users\*\git\scirs\target\debug\deps\libvirtue-ee9d2a5028ecb32c.rlib --extern proc_macro --cap-lints allow -L /opt/homebrew/opt/lapack/lib -L /opt/homebrew/opt/openblas/lib -l lapack -l blas` (exit code: 1)
```

I installed openblas:

`vcpkg install openblas --triplet x64-windows`

В чем 

> Windows 10 Pro x86_64; 22H2; 19045.6332

> 1.90.0-nightly 855e0fe46 2025-07-11
