# Windows VT capability probe (replacing version detection)

Implements the design from #41 (DHowett, Windows Console maintainer), as settled
in the thread with passcod: stop detecting Windows ≥10, and probe VT support
directly — the console host rejects `ENABLE_VIRTUAL_TERMINAL_PROCESSING` cleanly
when unsupported, regardless of OS version.

## API changes (breaking)

- Remove `ClearScreen::WindowsVt` and `ClearScreen::WindowsVtClear` entirely
  (agreed in #41; `WindowsVtClear` was `WindowsVt` + `XtermClear`, and with the
  scoped enable below, `XtermClear` alone does the job on Windows).
- Remove `pub fn is_windows_10()`: nothing detects Windows ≥10 any more, so the
  name would lie. Its long doc block (manifesting, version-lie caveats) goes
  with it.
- Keep `pub fn is_microsoft_terminal()`, now defined as the capability probe:
  true for any console host that supports VT processing (Windows Terminal,
  conhost on Win10+, the Win8 custom hosts that support it). Its doc already
  licensed behaviour changes.
- Ungate `ClearScreen::WindowsConsoleClear` from the `windows-console` feature:
  it is now the fallback path and must always exist. `WindowsConsoleBlank`
  stays feature-gated.

## Mechanism

- `win::vt()` returns a three-way result:
  - `NotAConsole`: stdout is not a console (redirected/captured) — no mode to
    manage; sequence-writing variants proceed unchanged (the `clear_to` custom
    writer use case keeps working).
  - `NoVt`: console host rejected the VT bit.
  - `Enabled(RestoreConsoleMode)`: VT enabled; a guard restores the original
    console mode on drop, so the enable is scoped to the operation. No
    `release()`/keep-on: the only would-be consumer (`WindowsVt`) is removed.
- `clear_to()` probes once at the top on Windows. `XtermClear` and `XtermReset`
  fall back to the legacy buffer clear (`win::clear()`) only when the probe says
  `NoVt`. Probe errors (GetStdHandle hard failure) do not abort sequence
  writing — plain `XtermClear` never touched the console before either.
- `Default` on Windows: VT-capable → `XtermClear`; else the existing
  terminfo/tput/`Cls` chain, unchanged.
- Delete the whole version cascade: `um_verify_version`, `um_netserver`,
  `um_workstation`, `vt_attempt`, `ABRACADABRA_THRESHOLD`, and the
  NetManagement/SystemInformation imports (including the netapi code just fixed
  in #53 — the redesign removes the need for it entirely).

## Follow-ups (out of scope here)

- watchexec CLI uses `ClearScreen::WindowsVt` (crates/cli/src/config.rs) — needs
  updating when this ships as a release; do it against the release then.
- TERMINALS.md per-terminal tables reference `WindowsVtClear` as the old
  default: add a dated note about the mechanism change; the tables themselves
  are historical observations and need re-testing, not editing.
- The commented-out `WindowsConsoleClear`/`WindowsConsoleBlank` platform tests
  stay commented out (need a non-VT console to exercise the fallback).
