# Changelog

## v1.0.11 (2022-12-28)

This is a special final 1.x release that includes all the 2.0.0 changes except for the breaking ones.

- Don't use BORS.
- Update dependencies.
- Handle tmux explicitly ([#9](https://github.com/watchexec/clearscreen/pull/9)).
- Fall back to hardcoded sequence if terminfo is not available ([#9](https://github.com/watchexec/clearscreen/pull/9)).

## v1.0.10 (2022-06-01)

- Use BORS.
- Update to nix 0.24, limit features to only those used ([#6](https://github.com/watchexec/clearscreen/pull/6)).

## v1.0.9 (2021-12-02)

- Change CI test to test Windows 10 detection with a manifested test executable.
- Clarify in documentation the expected behaviour of `is_windows_10()` and what is or not a bug.

## ~~v1.0.8 (2021-12-02)~~ (yanked)

- Stop checking powershell's `PackageManagement` capability as a Win10 check
  ([#5](https://github.com/watchexec/clearscreen/issues/5)).

## v1.0.7 (2021-08-26)

- Flush after E3 sequence in `Terminfo` ([#4](https://github.com/watchexec/clearscreen/issues/4)).

## v1.0.6 (2021-07-22)

- Omit unsupported UTF8 input flag on non-Linux.

## v1.0.5 (2021-07-22)

- Update to nix 0.22.

## v1.0.4 (2021-05-22)

- Fix [#1](https://github.com/watchexec/clearscreen/issues/1): need to flush after writing sequences.

## v1.0.3 (2021-05-08)

- Drop unused `log` dependency.
- Generalise iTerm workaround from 1.0.1 to default behaviour on macOS when the `TERM` starts with
  `xterm` and the terminfo does not have `E3`.
- Hide `WindowsConsoleClear` and `WindowsConsoleBlank` under an undocumented feature as they are
  buggy/do not work as per my testing on Win10. `WindowsVtClear` and `Cls` are sufficient for clear.

## v1.0.2 (2021-04-29)

- Use `VtRis` for Kitty when using its own terminfo.
- Use `VtRis` for SyncTERM, Tess, any rxvt, Zellij, and Zutty.
- Use `XtermClear` for these when using their own terminfo:
  - GNOME Terminal,
  - Konsole,
  - screen,
  - Termite,
  - XFCE4 Terminal.

## v1.0.1 (2021-04-26)

- Use `XtermClear` on iTerm on macOS when `TERM` starts with `xterm`, to work around macOS not
  having the correct terminfo by default.

## v1.0.0 (2021-04-25)

Initial release
