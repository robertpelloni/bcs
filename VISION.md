# VISION — BTK (BCS Framework)

> Single source of truth for what this project is trying to become.
> Version tracking lives in [VERSION](VERSION); history in [CHANGELOG.md](CHANGELOG.md);
> execution order in [ROADMAP.md](ROADMAP.md) and [TODO.md](TODO.md).

## 1. Ultimate goal

BTK is a cross-platform C++ application framework descended from the
CopperSpice/Qt lineage, being reworked into a systems foundation for
large-scale software and, longer term, an OS/application stack where
multiple local and remote users can interact with the same software
concurrently.

The target is 1:1 feature parity with Qt 6/7, JavaFX, JUCE, GTK, WinUI,
and Ultimate++ — with a smoother, more robust API and documentation than
any of them — plus one capability none of them have as a first-class
concept: **multi-user ownership and input arbitration** (several people,
several input devices, one shared UI, explicit focus/ownership rules).

## 2. Foundational concepts

| Concept | Meaning in BTK | Where it lives |
|---|---|---|
| Object model | Parent/child ownership, signals/slots, properties, timers, event loop | `src/core` (`CsCore`) |
| Widgets & windowing | Painting, input, focus, platform windows, multi-user ownership | `src/gui` (`CsGui`) |
| Input arbitration | Decides which user/device owns focus and input at any moment | `BcsInputArbitrator` (C++ design + all ports) |
| Declarative UI | BML (BTK Markup Language) runtime over the script engine | `src/declarative`, `src/script` |
| Multi-language kernel | The same semantics re-implemented natively in Go, Rust, C#, Java | `ports/go`, `ports/rust`, `ports/csharp`, `ports/java` |
| Reference assimilation | JUCE and Ultimate++ are kept as submodules and mined for parity gaps | `external/juce`, `external/ultimatepp`, `external/bqt-reference` |

## 3. Modules (C++)

`BtkCore` foundation types/threading/IO · `BtkGui` widgets/painting/input ·
`BtkNetwork` sockets/HTTP/SSL · `BtkXml` DOM/SAX · `BtkSql` DB drivers ·
`BtkSvg` · `BtkOpenGL` · `BtkVulkan` · `BtkMultimedia` audio/video/camera ·
`BtkWebKit` embedded browser · `BtkXmlPatterns` XQuery/XPath ·
`BtkBml` declarative runtime. Tools: `uic`, `rcc`, `lupdate`, `lrelease`,
`lconvert`, `linguist`. Public umbrella headers: `<btk.h>`, `<btkcore.h>`,
`<btkgui.h>`, `<btknetwork.h>`, and one per module above.

Internal CMake targets keep their historical `Cs*` names for ABI
stability and are aliased as `BTK::*`. The library ABI/package version
(`2.1.0`, set in the top-level `CMakeLists.txt`) is intentionally
independent of the project-tracking version in [VERSION](VERSION).

## 4. User-satisfaction design rules

1. **Every backend feature gets a UI surface.** New widgets/APIs ship with
   a label, a description, and a tooltip (`Label` / `Description` /
   `Tooltip` on the ported `BcsWidget` baseline; `setToolTip` /
   `setWhatsThis` in C++). A feature with no discoverable UI is unfinished.
2. **One place to look.** Status, build health, and module coverage are
   consolidated onto a single dashboard page (see
   [docs/DASHBOARD_UI_AUDIT.md](docs/DASHBOARD_UI_AUDIT.md)) instead of
   being fragmented across per-tool pages.
3. **Highest value first.** Core object model, GUI, and the multi-user
   arbitrator outrank every other module in docs, dashboard order, and
   porting order.
4. **No silent failure.** Build/run scripts print what they did and exit
   non-zero on failure; long builds run below normal priority so the
   machine stays usable.

## 5. Non-goals (for now)

- No production readiness claim: this is alpha software, breaking changes
  are expected (see [README.md](README.md)).
- No web admin dashboard in this repo. The operational dashboard for the
  wider project lives in the `bqt` submodule (`external/bqt-reference`,
  OmniDash). This repo's dashboard is a static, generated status page —
  see [docs/DASHBOARD_UI_AUDIT.md](docs/DASHBOARD_UI_AUDIT.md).
- Ports are semantic re-implementations, not bindings: no cgo/JNI/P-Invoke
  dependency on the C++ build.
