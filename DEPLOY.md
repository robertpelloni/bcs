# DEPLOY — build, install, and run BTK

> Verified against this checkout (Windows 11, VS2019 tree present).
> Every heavy build below should run **below normal priority** so the
> machine stays responsive — the scripts do this for you.

## 1. Prerequisites

| Tool | Needed for | Check |
|---|---|---|
| CMake ≥ 3.18 | C++ framework | `cmake --version` |
| MSVC (VS2019+) or a C++17 compiler | C++ framework | `cl` on PATH via VS prompt |
| Go ≥ 1.19 | `ports/go`, root stub | `go version` |
| Rust (cargo) | `ports/rust` | `cargo --version` |
| .NET SDK | `ports/csharp` | `dotnet --version` |
| JDK | `ports/java` | `java -version` |
| Python 3 | audit/inventory generators in `scripts/` | `python --version` |

Submodules (JUCE / Ultimate++ / bqt reference) are only needed for parity
work, not for building BTK itself:

```powershell
git submodule update --init --recursive
```

Note: submodule checkouts currently differ from the recorded pins
(`git submodule status` shows `+`). Resolve that deliberately before
relying on them — see [TODO.md](TODO.md).

## 2. One-command build (recommended)

```powershell
# Everything that can build, in value order, below-normal priority:
powershell -ExecutionPolicy Bypass -File scripts\build-all.ps1

# Fast layout/entry-point check only (no compilation):
powershell -ExecutionPolicy Bypass -File scripts\build-all.ps1 -CheckOnly
```

## 3. C++ framework (manual)

> **Compile-time warning:** without `/MP`, MSBuild compiles each library's
> sources serially and a full build takes many hours. Always configure with
> `-DCMAKE_CXX_FLAGS=/MP8` (8 compiler processes; raise on bigger machines).
> For test-only purposes the `Debug` configuration compiles several times
> faster than `Release` (`--config Debug`) with identical test semantics.

```powershell
# Configure (Release, tests optional) into the existing VS2019 tree:
cmake -S . -B build-vs2019 -DBUILD_TESTS=ON -DCMAKE_CXX_FLAGS=/MP8
# Build, below normal priority:
$p = Start-Process cmake -ArgumentList '--build','build-vs2019','--config','Release' -PassThru
$p.PriorityClass = 'BelowNormal'; $p.WaitForExit()

# Install (BTK package primary, CopperSpice legacy compat):
cmake --install build-vs2019 --config Release --prefix <install-dir>
```

Downstream use:

```cmake
find_package(BTK REQUIRED Core Gui Network)
target_link_libraries(myapp PRIVATE BTK::Core BTK::Gui)
```

Built tools land in `build-vs2019/bin/Release/` (`uic`, `rcc`,
`lupdate`, `lrelease`, `lconvert`, `linguist`) next to the `Cs*.dll`
libraries. Library ABI/package version is `2.1.0` (top-level
`CMakeLists.txt`); project-tracking version is in [VERSION](VERSION).

## 4. Tests

```powershell
ctest --test-dir build-vs2019 -C Release --output-on-failure
```

Suites: `test/core` (94 Catch2 files), `test/gui`, `test/network`.
Tests are **off by default** (`BUILD_TESTS=OFF`) — a build without them
proves compilation, not correctness.

## 5. Language ports

```powershell
cd ports\go;     go build ./...;  go vet ./...     # module: ports/go/go.mod
cd ports\rust;   cargo build;     cargo test
cd ports\csharp; dotnet build                     # Bcs.csproj (net8.0)
cd ports\java;   javac -d out Main.java + src/**  # or just use build-all.ps1; run: java -cp out Main
```

Root `start.bat` runs the Go port entry point (`ports/go`); it no
longer attempts `go run .` at the repo root, where no main package exists.

## 6. Dashboard

Open [docs/dashboard.html](docs/dashboard.html) in any browser (no server
needed). It is the single consolidated status page for this repo; see
[docs/DASHBOARD_UI_AUDIT.md](docs/DASHBOARD_UI_AUDIT.md) for the module →
UI mapping and the system-tray answer.
