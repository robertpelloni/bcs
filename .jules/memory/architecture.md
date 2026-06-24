## Architecture and Vision
*   **BCS Framework**: The project is a cross-platform C++ application framework, originally derived from the Btk/Qt/CopperSpice lineage. Its goal is to achieve 1:1 feature parity with Qt 6/7, JavaFX, JUCE, GTK, WinUI, and Ultimate++, positioning itself as a robust, native windowing toolkit.
*   **Multi-Paradigm Porting**: A core directive is to aggressively port the entire C/C++ framework uniformly across Go, Rust, C#, and Java, file-by-file and function-by-function. The architecture involves synchronized ports (`ports/`) mirroring the C++ `src/` hierarchy.
*   **Multi-User Ownership Model**: BCS introduces first-class multi-user input ownership primitives (`BcsInputOwner`, `BcsFocusToken`, `BcsInputArbitrator`) designed for concurrent, multi-user interactions and focus management locally and remotely.

## Patterns and Paradigms
*   **Strict Naming Conventions**: The framework has been rebranded entirely to `BCS`. All legacy terms (`bobtk`, `btk`, `bobqt`, `copperspice`, `bobcopperspice`) have been systematically replaced in filenames, directories, and file contents across the codebase.
*   **Public API Aliases**: BCS enforces branded aliases for standard types (e.g., `BcsString`, `BcsSharedPointer`, `BcsSignal`) instead of legacy Qt-style naming to assert its unified identity.
*   **Aggressive Assimilation**: The repository tracks major upstream toolkits (e.g., JUCE, Ultimate++) as submodules in `external/` to serve as reference baselines for integrating their unique DSP/Audio or RAD paradigms into BCS.

## Development and Operational Directives
*   **Continuous Autonomous Execution**: The development model mandates an "autopilot" approach, executing tasks sequentially, committing frequently, and pushing to git after every major step without pausing for confirmation.
*   **Documentation Governance**: Extreme emphasis is placed on comprehensive, up-to-date global references. The architecture relies on specific files: `VISION.md`, `ROADMAP.md`, `TODO.md`, `HANDOFF.md`, `CHANGELOG.md`, `VERSION`, `DEPLOY.md`, `MEMORY.md`, and `IDEAS.md`.
*   **Versioning**: Every build requires an explicit version bump recorded in a central `VERSION` text file and referenced in commit messages.
*   **Code Quality**: Deep, substantive code commenting is required, explaining logic, optimizations, and structural side-effects while avoiding redundant explanations of self-evident code. All UI backend features must have comprehensive, detailed frontend representations.