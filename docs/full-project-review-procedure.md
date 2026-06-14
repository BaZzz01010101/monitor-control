You are working inside VSCode through the Codex extension.

Project context:
- Language: Rust.
- UI framework: Slint.
- Platform: Windows.
- Application type: GUI utility running in the Windows System Tray.
- Purpose: control Dell monitor functionality, including brightness, contrast, KVM features, and input/source switching.
- The application communicates directly or indirectly with the monitor/device.

Task:
Perform a deep, comprehensive review of the entire project.

Main goal:
Find real mistakes, weak points, hidden bugs, architectural problems, performance problems, reliability issues, and design problems. Do not limit the review to compiler errors or obvious Rust mistakes. Look for logical, architectural, runtime, UX, device-communication, and Windows-specific problems.

Focus areas:
1. Rust correctness
   - Incorrect ownership, borrowing, lifetimes, synchronization, error handling, panics, unwrap/expect misuse.
   - Incorrect use of Result, Option, threading, async code, channels, shared state, mutexes, atomics.
   - Code that may compile but behave incorrectly.

2. Runtime stability
   - Possible crashes.
   - Deadlocks.
   - Race conditions.
   - UI freezes.
   - Tray icon lifecycle issues.
   - Background worker failures.
   - Incorrect shutdown behavior.
   - Resource leaks.

3. Windows-specific behavior
   - System Tray integration.
   - Startup behavior.
   - Message loop / event loop interaction.
   - DPI scaling.
   - Multiple monitor handling.
   - Permissions.
   - Sleep/wake handling.
   - Monitor reconnect/disconnect handling.
   - Behavior after display topology changes.

4. Slint UI architecture
   - Incorrect separation between UI and backend logic.
   - Blocking calls from UI callbacks.
   - State synchronization problems.
   - Inefficient property updates.
   - Incorrect use of weak handles, callbacks, or component handles.
   - UX problems caused by implementation decisions.

5. Dell monitor communication
   - Correctness of communication with the monitor.
   - Error handling around device access.
   - Timing problems.
   - Retry logic.
   - Device discovery.
   - Handling unavailable monitor/device state.
   - Handling multiple Dell monitors.
   - Handling unsupported features.
   - Handling stale cached state.
   - Command sequencing problems.
   - Any unsafe assumptions about brightness, contrast, KVM, or input switching behavior.

6. Performance
   - Blocking operations on the UI thread.
   - Excessive polling.
   - Inefficient device queries.
   - Unnecessary allocations.
   - Busy loops.
   - Slow startup.
   - Slow tray/menu refresh.
   - Unnecessary repeated monitor communication.
   - Poor caching strategy.

7. Architecture
   - Module boundaries.
   - Separation of concerns.
   - Testability.
   - State model.
   - Error model.
   - Logging model.
   - Configuration model.
   - Device abstraction.
   - Extensibility for additional monitors or commands.
   - Whether the current architecture will remain maintainable as the app grows.

8. Error handling and observability
   - Missing logs.
   - Logs that are too noisy or not actionable.
   - Lost errors.
   - Silent failures.
   - Poor user-visible error reporting.
   - Missing diagnostics for device communication.
   - Missing tracing around critical operations.

9. Configuration and persistence
   - Incorrect config file handling.
   - Unsafe defaults.
   - Broken migration behavior.
   - Invalid state after config changes.
   - Incorrect persistence of selected monitor/input/KVM state.
   - Poor handling of corrupted config.

10. Testing
   - Missing unit tests.
   - Missing integration tests.
   - Hard-to-test code.
   - Areas where device communication should be mocked.
   - Suggested test strategy for Rust logic, Slint UI interaction, and monitor communication.

Review method:
- First, inspect the project structure and identify the main modules, entry points, UI layer, tray layer, device communication layer, configuration layer, and state management layer.
- Then review the code path by path.
- Search for obvious risky patterns such as unwrap, expect, panic, blocking calls, sleeps, polling loops, global mutable state, unsafe code, TODO/FIXME comments, ignored Results, broad error swallowing, and duplicated logic.
- Do not stop after finding the first problems. Continue until the whole project is reviewed.
- Prefer concrete findings over generic advice.
- If a suspected issue is uncertain, clearly mark it as uncertain and explain what evidence is missing.

Output format:
1. Executive summary
   - Short summary of the most important findings.
   - Severity overview.

2. Project architecture map
   - Main modules/components.
   - Current responsibilities.
   - Main data/control flow.
   - Monitor/device communication flow.

3. Findings
   For each finding, provide:
   - Title.
   - Severity: Critical / High / Medium / Low.
   - Category: correctness / performance / architecture / device communication / UI / Windows integration / testing / maintainability.
   - Exact file and code location.
   - Description of the problem.
   - Why it matters.
   - Concrete fix.
   - Risk of the fix.
   - Suggested test or verification method.

4. Improvement plan
   - Prioritized action plan.
   - Separate immediate fixes, medium-term refactoring, and long-term improvements.
   - Explain dependencies between tasks.

5. Proposed target architecture
   - Recommended structure for UI, tray, state, device communication, config, logging, and tests.
   - Keep the proposal practical for a small Windows tray utility.

6. Testing plan
   - Unit tests.
   - Integration tests.
   - Mock device communication.
   - Manual Windows validation checklist.
   - Regression tests for brightness, contrast, KVM, and input switching.

7. Concrete code changes
   - Where possible, propose exact patches or code snippets.
   - Do not rewrite the whole project unless necessary.
   - Prefer small, safe, incremental changes.

Important constraints:
- Do not make unsupported assumptions.
- Do not claim that something is wrong unless the code supports it.
- If you need to run commands, prefer safe read-only commands first.
- Do not perform destructive changes.
- Before editing files, present the proposed plan.
- When editing, keep changes minimal and focused.
- Use idiomatic Rust.
- Preserve the existing application behavior unless changing it is part of a clearly justified fix.
- Avoid large rewrites unless the current architecture clearly requires it.

Start by mapping the project structure, then perform the review, then produce the findings and improvement plan.