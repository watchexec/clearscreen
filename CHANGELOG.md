# Changelog

## Next (YYYY-MM-DD)

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
