# Changelog

## Next (YYYY-MM-DD)

- Use `VtRis` for Kitty when using its own terminfo.
- Use `VtRis` for SyncTERM.
- Use `XtermClear` for these when using their own terminfo:
  - GNOME Terminal,
  - Konsole,
  - screen,
  - XFCE4 Terminal.

## v1.0.1 (2021-04-26)

- Use `XtermClear` on iTerm on macOS when `TERM` starts with `xterm`, to work around macOS not
  having the correct terminfo by default.

## v1.0.0 (2021-04-25)

Initial release
