# Changelog

## Next (YYYY-MM-DD)

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
