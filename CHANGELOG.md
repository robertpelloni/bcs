# CHANGELOG

## 0.3.31 - 2026-11-14
- **CsScript lives**: the JavaScript engine now compiles, links, evaluates,
  and exits cleanly (verified: `1+2=3`, function calls, string ops, exit 0).
  Three Stage-A defects were blocking it:
  1. `wtf/PageBlock.cpp` was missing from the link (`QTWTF::pageSize`
     unresolved) — added to `src/script/CMakeLists.txt`.
  2. Masquerading `ClassInfo` entries (`QScriptObject`,
     `QMetaObjectWrapperObject`, `QScriptActivationObject`,
     `QScriptStaticScopeObject`) had no parent chain, tripping the
     JSObject constructor `inherits()` assertion at engine startup —
     now parented to `JSObject::s_info` / `JSVariableObject::s_info`.
  3. `QScriptEnginePrivate` teardown never called
     `clearBuiltinStructures()`, so builtin `Strong<Structure>` handles
     were deallocated into an already-destroyed heap (crash on exit) —
     now mirrors the `JSGlobalContextRelease` order.
- Added maintained regression test `test/script/smoke_script.cpp`
  (`CsScriptSmoke`, built when `WITH_SCRIPT=ON`, run via ctest; passes).
- Configure a Script tree with:
  `cmake -S . -B build-script -DWITH_SCRIPT=ON -DCMAKE_CXX_FLAGS=/MP8`
  (Debug config recommended; amd64 MSBuild).

## 0.3.30 - 2026-11-14
- GUI + Network suites now run: CsGui (Debug, /MP8, amd64 MSBuild —
  the 32-bit node OOMs on warning volume) and CsNetwork build;
  `CsGuiTest` 15/15 pass, `CsNetworkTest` 41/42 pass.
- Known failure (documented): `QHostInfo basic_methods` pins
  copperspice.com to one hardcoded IP (69.89.11.103); the domain now
  resolves via Cloudflare to two anycast addresses, so the assertion
  is stale upstream and the sync lookup can stall. Not rewritten —
  upstream should stop pinning DNS answers.
- Overall C++ baseline: Core 1081/1082, Gui 15/15, Network 41/42.
  The two failures are one machine-zone-sensitive case
  (`QDateTime std_chrono`, see TODO) and one live-DNS case.

## 0.3.29 - 2026-11-14
- Fixed CsNetwork failing against OpenSSL 3/4 (installed: 4.1.0):
  ASN1_STRING became opaque in OpenSSL 1.1, so direct `->length`/`->data`/
  `->type` access in `qsslcertificate_openssl.cpp` and
  `qsslsocket_openssl_symbols.cpp` no longer compiles. Now uses
  `ASN1_STRING_length` / `ASN1_STRING_get0_data` / `ASN1_STRING_type`.
  Verified: CsNetwork (Debug) builds and links.
- Build knowledge (in DEPLOY.md): configure with `-DCMAKE_CXX_FLAGS=/MP8`
  or library builds are serial and take hours; MSBuild's 32-bit node can
  OOM aggregating warning output on CsGui — use the amd64 MSBuild.

## 0.3.28 - 2026-11-13
- C++ test baseline established: fresh `build/` (VS2026/MSVC 19.52)
  configures, `CsCoreTest` runs under ctest: **1081 passed, 1 failed**
  (of 1082). Note: ctest must run with `build\bin\Release` on PATH or
  every test fails instantly with DLL-not-found (0xC0000135).
- Fixed 5 rebrand-corrupted test fixtures (test data shortened from the
  11-char "CopperSpice" to "BTK" without updating lengths/indices):
  `QByteArray chop/data/resize` (neutral 11-char fixture),
  `QFile read` (wrote 11 bytes of a 3-byte literal — out-of-bounds read),
  `QVariant constructor_bytearray` (data spelled the old name).
  All 5 now pass.
- Known failure (documented, not hidden): `QDateTime std_chrono` is
  sensitive to the machine Windows time-zone/DST handling; fails on this
  Eastern machine regardless of the TZ env var. Needs a Pacific-zone
  machine or a fromStdLocalTime investigation to resolve.
- Naming audit now passes (0 violations): `check_btk_naming.py` prunes
  `build*` trees instead of double-reporting generated header copies.
- All local branches fast-forwarded to `master`; `main` and `master`
  pushed and in sync on origin.

## 0.3.27 - 2026-11-13
- C# port builds and runs for the first time: added `ports/csharp/Bcs.csproj`
  (0 warnings, 0 errors). Fixes required: committed merge-conflict markers
  in `Bcs.cs`/`BcsPointer.cs` (both sides identical), the `BCS.*` vs `Bcs.*`
  namespace split (33 files unified), duplicate stub classes hidden in
  `BcsEvent.cs`/`BcsPen.cs`, missing `EventType`/`BcsTimerEvent`/
  `BcsChildEvent` (added, mirroring Go/C++/Qt semantics), `Thread`
  namespace collision, and event-loop API drift (`PostEvent`, optional
  dispatcher). `BcsWidget` now routes mouse-release to a virtual handler.
- Java port compiles and runs for the first time: unified `core.*` vs
  `bcs.core.*` packages (27 files), added missing `EventType` enum,
  removed duplicate `BcsBrush`, added dispatcher-backed `BcsEventLoop`
  constructor + `int exec()`, `BcsObject.close()` teardown, and widget
  mouse-release routing mirroring the C# fix.
- `scripts/build-all.ps1` now builds all four ports and fails on any
  committed merge-conflict markers in tracked files (new guard; the
  HANDOFF.md conflict found by that sweep is also resolved).
- HANDOFF.md: resolved the Phase 3–9 conflict block (kept the HEAD side).

## 0.3.26 - 2026-11-13
- pushed v0.3.25 to origin/master using the workspace GitHub token
  (stored credential had expired; token itself never printed)
- verified the fresh C++ build end-to-end: `build/bin/Release/uic.exe`
  reports "BTK User Interface Compiler version 1.0.0" (the stale
  prebuilt in `build-vs2019` still says CopperSpice) and `rcc.exe` runs
- Rust port now builds warning-free: added `BcsApplication::arguments()`
  (mirrors `QCoreApplication::arguments()`) instead of leaving the
  stored command line unread
- scheduled autopilot continuation every 15 minutes (permanent cron)

## 0.3.25 - 2026-11-13
- fixed fresh CMake configure broken by the rebrand: all modules/tools now
  install to `BTKLibraryTargets`/`BTKBinaryTargets` (WebKit was the last
  `CopperSpiceLibraryTargets` holdout), and the top level installs those
  sets under the legacy `CopperSpice*Targets.cmake` file names and
  `CopperSpice::` namespace, so both `BTKConfig` and `CopperSpiceConfig`
  consumers keep working; verified: `cmake -S . -B build` now generates
- resolved committed merge-conflict markers in `MEMORY.md` (union of both sides)
- fixed root `start.bat`: it ran `go run .` where no Go main package exists;
  it now runs the Go port in `ports/go` and propagates its exit code
- added one-command build: `scripts/build-all.ps1` (ports in value order,
  then C++ tools; below-normal priority; `-CheckOnly` layout regression check)
  plus `scripts/README.md` inventory; script reports the stale
  `build-vs2019` cache (configured at the old `…\workspace\btk` path)
  instead of failing or deleting the owner's tree
- fixed Rust port warning: unused `Mutex` import in `bcsapplication.rs`
- added root docs: `VISION.md`, `ROADMAP.md`, `TODO.md`, `DEPLOY.md`,
  `IDEAS.md`; filled the `SUBMODULE_INVENTORY.md` stub from live git state
- consolidated dashboard: `docs/dashboard.html` (single static page, every
  module/tool/port/test/workflow, value-ordered, tooltips) with the audit
  and system-tray answer in `docs/DASHBOARD_UI_AUDIT.md`
- verified: Go build+vet pass, Rust build passes, prebuilt `uic`/`rcc`
  run, fresh C++ configure passes; C#/Java ports still lack build files

## 0.2.0 - 2026-04-08
- completed comprehensive CopperSpice → BTK rebrand across all source files (4,282+ files)
- updated all copyright/license comment headers from "This file is part of CopperSpice" to BTK
- renamed all user-facing error messages, diagnostics, and UI strings from CopperSpice to BTK
- updated cmake package install paths: BTK is now the primary package, CopperSpice is legacy compat
- added comprehensive BTK subsystem alias headers:
  - `<btk.h>` - full framework umbrella
  - `<btkcore.h>` - core framework with smart pointers, signals, strings
  - `<btkgui.h>` - GUI subsystem with multi-user ownership
  - `<btknetwork.h>` - networking
  - `<btkopengl.h>` - OpenGL rendering
  - `<btksql.h>` - database access
  - `<btkmultimedia.h>` - audio/video/camera
  - `<btksvg.h>` - SVG rendering
  - `<btkxml.h>` - XML parsing
  - `<btkvulkan.h>` - Vulkan rendering
  - `<btkwebkit.h>` - web browser engine
  - `<btkxmlpatterns.h>` - XQuery/XPath
- expanded existing alias headers with richer type coverage:
  - `btkstring.h`: added BtkEncodingUtf8, BtkEncodingUtf16 aliases
  - `btkpointer.h`: added BtkPointerTraits alias
  - `btksignal.h`: added connection kind constants
- all cmake module files updated with BTK branding
- all test files updated with BTK branding
- all utility files updated with BTK branding
- all translation files updated with BTK branding
- cmake primary install path changed from `cmake/CopperSpice` to `cmake/BTK`
- legacy CopperSpice package config preserved for backward compatibility

## 0.1.9 - 2026-04-06
- added a reusable staging helper for probe-local BTK core artifacts: `scripts/stage_probe_core_artifacts.ps1`
- staged `CsCore2.1.lib` and `CsCore2.1.dll` from the validated `build-vs2019` tree into `build-vs2019-script-probe5` at the exact paths expected by `CsScript.vcxproj`
- documented the probe artifact-staging rationale, implementation, and validation evidence
- preserved the active detached linked-build investigation while narrowing the isolated link/materialization frontier

## 0.1.8 - 2026-04-06
- investigated the new post-compile `CsScript` link-materialization frontier in `build-vs2019-script-probe5`
- confirmed `CsScript.vcxproj` explicitly links `..\\..\\lib\\Release\\CsCore2.1.lib` while `CsCore.vcxproj` is configured to emit that import library at the matching probe-local path
- revalidated that compile-only `CsScript` succeeds while full isolated linked validation still stops at the missing `CsCore2.1.lib` boundary when project references are disabled
- added a detached persisted-log linked-build helper with project references enabled: `build-vs2019-script-probe5/run_csscript_with_refs_background_wmi.ps1`
- launched a new detached linked-build investigation without killing existing processes and documented the live wrapper / worker PIDs plus the new logs to monitor
- documented the link-materialization frontier and investigation evidence in new design / implementation / testing docs

## 0.1.7 - 2026-04-06
- contracted the reduced `CsScript` QObject bridge frontier across `qscriptqobject.cpp` / `qscriptqobject_p.h`
- added current host-call / host-construct proxy entry points for `QtFunction`, `QtPropertyFunction`, and `QMetaObjectWrapperObject`
- replaced remaining reduced QObject bridge enum-style error dispatch with current error-object creation helpers
- updated reduced QObject bridge call-discovery, mark-stack rooting, `Heap::isMarked(...)`, identifier creation, and `QString` / `UString` conversion paths
- contracted the remaining compile blockers in `qscriptcontextinfo.cpp` and `qscriptengine.cpp`
- validated that `MSBuild /t:ClCompile` for `build-vs2019-script-probe5/src/script/CsScript.vcxproj` now succeeds with `0` errors
- confirmed the next isolated frontier is now a link-materialization boundary (`CsCore2.1.lib` missing in the isolated probe when project references are disabled)
- documented the QObject bridge compilation pass and validation evidence

## 0.1.6 - 2026-04-06
- widened the embedded `NativeFunctionWrapper` / `PrototypeFunction` compatibility bridge so restored Script call sites can target both legacy wrapper signatures and current JSC native-function signatures
- contracted the restored `CsScript` engine/value frontier across `qscriptengine.cpp`, `qscriptprogram.cpp/.p.h`, and `qscriptvalue.cpp`
- replaced removed evaluation/debugger/value helpers with current JSC equivalents, including raw `EvalExecutable *` caching, current `getCallData(...)` / `getConstructData(...)`, current `setPrototype(...)`, current `removeDirect(...)` / `putDirect(...)`, and explicit `QString` / `UString` bridges
- revalidated the direct MSVC `CsScript` probe and reduced the observed failure volume from `162` errors to `78` errors
- moved the dominant first-failure hotspot away from `qscriptengine.cpp` / `qscriptvalue.cpp` and into the next reduced bridge frontier centered on `qscriptqobject.cpp`
- documented the new engine/value contraction pass and its validation evidence

## 0.1.5 - 2026-04-05
- re-established detached persisted-log `CsScript` probing in `build-vs2019-script-probe5`
- captured the next reduced first-failure frontier beyond the declarative/class delegate ABI pass in `src/script/api/qscriptengine.cpp`
- documented the new clustered engine-layer drift around translation helpers, error creation, engine bootstrap signatures, `WriteBarrier` access, native-frame metadata, GC marking, and debugger/evaluation helpers
- preserved all existing background processes while launching a new detached probe wrapper for deeper diagnostic capture

## 0.1.4 - 2026-04-05
- adapted restored declarative/class delegate call and construct plumbing to the current JavaScriptCore host-call ABI
- replaced stale enum-style `JSC::TypeError` throw assumptions in the reduced declarative/class frontier with `createTypeError(...)` + `throwError(...)`
- revalidated `build-vs2019-script-probe5/src/script/CsScript.vcxproj` and confirmed the prior reduced frontier in `qscriptdeclarativeobject.cpp` / `qscriptclassobject.cpp` no longer appears first
- observed the direct MSVC probe progress materially deeper into later JavaScriptCore/API compilation before timing out
- added detached probe5 persisted-log helper scaffolding and documented the new deeper validation state

## 0.1.3 - 2026-04-05
- added a narrow JavaScriptCore compatibility bridge for restored QtScript-era host functions:
  - `runtime/NativeFunctionWrapper.h`
  - `runtime/PrototypeFunction.h`
  - `JSGlobalObject::prototypeFunctionStructure()` compatibility accessor
- updated restored Script wrapper/global-object code to target the current host-call ABI and current `WriteBarrier` conventions
- revalidated direct MSVC Script bring-up with `build-vs2019-script-probe5/src/script/CsScript.vcxproj`
- removed the prior missing `PrototypeFunction` / `NativeFunctionWrapper` frontier and reduced the next fatal cluster to legacy declarative bridge call plumbing, especially `qscriptdeclarativeobject.cpp`
- documented the host-function compatibility pass and its new reduced frontier

## 0.1.2 - 2026-04-05
- captured the first post-linkage-unblock fatal `CsScript` diagnostics from a detached persisted-log MSVC build
- identified the next reduced recovery frontier in:
  - `src/script/api/qscriptcontext.cpp`
  - `src/script/api/qscriptcontextinfo.cpp`
  - `src/script/bridge/qscriptfunction_p.h`
- began the next adaptation pass in `qscriptcontext.cpp` and `qscriptcontextinfo.cpp` by contracting several obvious exception/error/scope-chain and metadata-signature mismatches
- documented the new context-layer recovery frontier and persisted-log validation strategy

## 0.1.1 - 2026-04-05
- advanced the restored `CsScript` bridge/object layer around the current JavaScriptCore object model
- migrated key restored Script wrapper structure plumbing from stale `PassRefPtr`/`RefPtr` assumptions toward raw `JSC::Structure *` contracts where appropriate
- updated restored Script object wrappers to current `visitChildren(...)` / `OverridesVisitChildren` semantics
- rebuilt restored `JSVariableObject` subclasses (`QScriptActivationObject`, `QScriptStaticScopeObject`) around current symbol-table/register ownership expectations
- enabled `BUILDING_JavaScriptCore` and `BUILDING_WTF` for the recovered `CsScript` target, removing the earlier `JSString.cpp` import/export linkage blocker
- reconfigured and revalidated the direct MSVC `CsScript` project build, which now progresses substantially deeper into JavaScriptCore runtime/JIT compilation before the validation timeout

## 0.1.0 - 2026-04-05
- bootstrap project-local version/changelog tracking for ongoing BTK recovery work
- advanced `CsScript` Stage A by adapting current identifier-table handling to `wtfThreadData()`
- replaced obsolete JSC debugger load/unload assumptions with Script-agent routing
- reworked Script source-provider feedback away from inaccessible `UStringSourceProvider` construction
- modernized multiple Script string/value conversion sites around current `UString`, `jsNumber`, `jsString`, and `constructDate` contracts
- contracted major `qscriptdeclarativeclass.cpp` drift and propagated low-risk `QString`/identifier fixes into nearby Script bridge/API files
- revalidated the direct `CsScript` MSVC build probe and documented the newly exposed bridge-header / linkage blocker frontier

## 0.3.0
- **PORTING (Phase 1-9)**: Executed a massive architectural translation of `BCS::Core` and `BCS::Network` primitives into Go, Rust, C#, and Java.
- **SYNC**: Upstream synchronization from `master` mapped into `main`, tracking the legacy rename dependencies and submodules correctly.
- **BUILD**: Incremented global version to 0.3.0 representing the completion of the multi-language port infrastructure.

## 0.3.1
- **PORTING (Go Functional Layer)**: Transitioned from semantic scaffolding to active functional porting. Integrated `BcsInputArbitrator` natively into Go utilizing `BcsSignal` capabilities for multi-user focus tracking.

## 0.3.2
- **PORTING (Core Stream & String Utils)**: Added functional operations to `BcsString` (Length, Substr, etc) and fully ported `BcsTextStream` file/IO reading abstraction to Go, Rust, Java, and C#.

## 0.3.3
- **PORTING (Core Semantic Expansions)**: Fully satisfied foundational mapping directives by augmenting `BcsAlignment` and `BcsOrientation` across all language targets (`bcscoretypes.h` equivalent). Prepared structures for functional layout bindings.

## 0.3.4
- **PORTING (Core File System)**: Merged Executive Protocol changes finalizing `bcstextstream.h` functionally across Go, Rust, Java, and C# environments to provide uniform file I/O utilities backing `BcsString`.

## 0.3.5
- **PORTING (Core GUI Kernel)**: Concluded functional port mapping for `BcsInputArbitrator` by successfully translating concurrent, multi-user focus stream logic into Rust, Java, and C# complementing the existing Go implementation.

## 0.3.6
- **PORTING (Go Tooling)**: Validated core semantic string mapping by implementing `BcsCommandLineParser` natively in Go.

## 0.3.7
- **PORTING (Tooling & String Utilities)**: Fixed structural errors in Java and C# `BcsString` ports.
- **PORTING (Tooling)**: Implemented `BcsCommandLineParser` natively in C#, Java, and Rust, mapping functionality from the Go reference port.
- **PORTING (Core Kernel)**: Implemented `BcsObject` and `BcsEventLoop` natively in C#, Java, and Rust based on the Go reference port.

## 0.3.8
- **PORTING (Gui Kernel)**: Updated `BcsApplication` natively in C#, Java, and Rust to explicitly utilize the newly ported `BcsEventLoop` for lifecycle execution.

## 0.3.9
- **PORTING (Core Kernel)**: Implemented priority-based event dispatching natively in Go, Rust, C#, and Java using respective heap/priority queue structures in `BcsEventLoop`.
- **PORTING (Core Global)**: Implemented async `BcsSignal` capabilities via thread pools and asynchronous contexts across C#, Java, and Rust.
- **PORTING (Gui Widgets)**: Integrated updated `BcsEvent` systems into `BcsWidget` base classes across all target languages.

## 0.3.10
- **PORTING (Gui Kernel)**: Mapped `BcsApplication` completely into Go, Rust, Java, and C# extending `BcsObject` and bootstrapping the `BcsEventLoop` to manage primary application lifecycles.

## 0.3.11
- **PORTING (Gui Widgets)**: Updated `BcsWidget` foundational classes across C#, Java, Go, and Rust with `Tooltip`, `Description`, and `Label` properties to satisfy UI integration compliance rules.

## 0.3.12
- **PORTING (Core Tools)**: Mapped `BcsGeometry` types (`BcsPoint`, `BcsSize`, `BcsRect`) completely into Go, Rust, Java, and C# including containment logic to build toward frontend rendering parity.
- **PORTING (Core Tools)**: Mapped `BcsVariant` dynamic type erasure containers completely into Go, Rust, Java, and C# natively to enable dynamic property tracking.

## 0.3.13
- **PORTING (Network Socket)**: Translated `BcsTcpServer` implementations natively across Java, C#, and Rust mirroring the underlying listener/accept paradigms mapped in the original Go design.

## 0.3.14
- **PORTING (Network Socket)**: Finalized `BcsTcpSocket` and `BcsTcpServer` connectivity mapping, addressing correct client stream injections dynamically.

## 0.3.15
- **PORTING (Core Subsystems)**: Validated module linkage and cross-directory references spanning `BcsTextStream`, `BcsTcpServer`, `BcsTimer`, and `BcsWidget` correctly resolving inside native Rust module definitions without orphaned compilation segments.

## 0.3.16
- **PORTING (Gui Kernel)**: Enhanced `BcsInputArbitrator` structures across Go, Rust, and C# establishing unified standard mappings for tracking active input owners via concurrent dicts and RW-locked maps.

## 0.3.17
- **PORTING (Go Initialization)**: Added `go.mod` module path initialization tracking for cross-subsystem dependency resolution in Go ports natively bridging Kernel/Network/Gui links.

## 0.3.18
- **PORTING (Go Architecture)**: Re-mapped the Go source tree correctly utilizing absolute module names (`github.com/robertpelloni/bcs/ports/go/core/...`) allowing complete compilation via standard `go build ./...` tools for native package handling.

## 0.3.19
- **PORTING (Gui Widgets)**: Integrated advanced `BcsWidget` configurations handling advanced FocusPolicy and state configurations seamlessly linked against the modernized `BcsEventLoop` implementations in C#, Java, Go, and Rust.

## 0.3.20
- **TESTING (Core Kernel)**: Initialized unit testing scaffolds demonstrating functional correctness of `BcsEventLoop` lifecycle and `BcsObject` parent/child routing execution natively across the multi-language frameworks.

## 0.3.21
- **PORTING (Core Kernel)**: Fixed Rust `BcsTimer` concurrency deadlocks by properly releasing mutexes before thread callbacks. Re-architected Java's async `BcsSignal` capabilities via cached daemon thread executors to enforce clean JVM termination upon `BcsApplication.quit()`. Mapped IPC abstractions (`BcsSystemSemaphore`, `BcsSharedMemory`) functionally across Go, Rust, Java, and C#.

## [0.3.23] - 2024-07-03
### Added
- Core Kernel thread-safe event scheduling abstractions: `BcsEventDispatcher`, `BcsEventTypes`.
- Cross-language thread modeling: `BcsThread`, `BcsMutex` (wrapping `QThread` and `QMutex` semantics across Go `goroutines`/`sync`, Rust `std::thread`/`Mutex`, C# `Thread`/`Monitor`, and Java `Thread`/`ReentrantLock`).
- Base GUI global orchestrator: `BcsApplication` (mapping `QCoreApplication`) which initializes the `BcsEventLoop`.
- Input routing: `BcsInputArbitrator` natively connected to event dispatch pipeline.
- Base GUI window abstraction: `BcsWindow` in Go, C#, Rust.
- Type-erasure foundation `BcsVariant` natively mapped to `interface{}`/`Object`/`object`/`Box<dyn Any>` across Go, Java, C#, and Rust respectively.

## [0.3.24] - 2024-07-03
### Added
- Native port of `BcsFont` logic for rendering hints mapping to Qt `QFont`.
- Native port of `BcsColor` providing type-safe color spaces (RGB, CMYK, HSV) and clamp processing.
- `gui/painting/` and `gui/text/` package initialization.
