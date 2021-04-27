Research on Terminals
=====================

All tested with their latest version obtainable of Arch Linux (or macOS 11, Windows 10) as of
writing. Version tested is noted where possible, but otherwise compare to git blame date for the
entry.

To contribute entries:

- Insert in the correct category, in lexicographic order
- Test with both the terminal’s own terminfo, and with `xterm-256color`.
- If the terminal doesn’t have its own terminfo, note that, and note which it is trying to emulate.
  - And consider filing a bug to tell them to provide their own terminfo!
- If a terminal has forks, especially if there’s a lot of them, only document a fork if its
  behaviour is different.
- If the terminal is based on a common library, mention it.
- If the terminal is web-based, mention that.
- Document the current selection of `::default()`.
- Document the behaviour of at least:
  - `Terminfo`
  - `TerminfoScreen`
  - `TerminfoScrollback`
  - `VtRis`
  - `XtermClear`
- “Normal” behaviour refers to:
  - `::default()`: screen and scrollback (if at all possible) cleared
  - `Terminfo`: at least screen cleared, and optionally scrollback
  - `TerminfoScreen`: only screen cleared
  - `TerminfoScrollback`: only scrollback cleared
  - `VtRis`: screen and scrollback cleared, and (at least some modes of) terminal reset
  - `XtermClear`: screen and scrollback cleared
- There is zero tolerance for advertising via this document.

How to test:
------------

First link the cli example program into your PATH, e.g.

```
ln -s $(pwd)/target/debug/examples/cli ~/.local/share/bin/clscli
```

Open the terminal in its default profile, or as it comes when first installed.

Then use `env | grep TERM` to see what the `TERM` and other related variables look like (make note!).

Look into `/usr/share/terminfo` for a terminfo that matches the terminal, or wherever it is on your
system. If there's a separate but official package for the terminal’s terminfo, use it.

First test with the native terminfo: set it either in the terminal’s settings, or use
`env TERM=name $SHELL`, then with the `TERM` the terminal first starts with by default, and finally
with `xterm-256color` if that’s not been covered yet.

(Recall that `clscli` is the `cli` example program in this same repo.)

 1. First run `clscli auto`. Look quick, the name of the variant selected by default will be printed,
    and one second later, hopefully, the screen will clear. Document that variant.
 2. Then run `clscli Variant` where the variant is: `Terminfo`, `TerminfoScreen`,
    `TerminfoScrollback`, `VtRis`, `XtermClear`, and the variant discovered in 1, if not one of
    these. Before each, run `seq 1 100` or something like it to fill the screen and some scrollback.
    Document the behaviour if it differs from normal, or state “normal.”
 3. Optionally (if you want), if `clscli auto` does not exhibit the normal behaviour, open an issue
    and provide enough details to be able to modify the `::default()` selection to select a
    different default that works. If you’re really enthusiastic, you can even open a PR with it!
 4. To submit your research, either submit a PR to this file (preferred, you can even do it in the
    GitHub Web UI), or open an issue with your research (I’ll merge it in), or send me an email.


Emulator libraries
------------------

### BearLibTerminal

### libamxt

### libt3widget

### libt3window

### libterm

### libtickit

### libtsm

### libvterm

### Qtermwidget

### Rote

### VTE


Emulators
---------

### Alacritty

With `TERM=alacritty`:

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: normal.
- `VtRis`: normal.
- `XtermClear`: normal.

With `TERM=xterm-256color`:

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: normal.
- `VtRis`: normal.
- `XtermClear`: normal.

### Aminal

Native `TERM` is `xterm-256color`.

- Default: `Terminfo`. **The better option would be `VtRis`, but there’s no way to tell we’re
  running in Aminal.**
- `Terminfo`: does not clear scrollback, appears to clear the screen, but really erases the screen
  without scrolling the existing output up, thus losing a screenful of information.
- `TerminfoScreen`: appears to clear the screen, but really erases the screen without scrolling the
  existing output up, thus losing a screenful of information.
- `TerminfoScrollback`: does not clear scrollback, erases the screen, but leaves cursor position
  intact, i.e. at the bottom of the screen if we were there.
- `VtRis`: clears screen, doesn’t clear scrollback, but does push the existing output up, so that
  information is not lost.
- `XtermClear`: as for `Terminfo`.

### Android Terminal Emulator

### Archipelago

- Web-based

### ate

Native `TERM` is `xterm-256color`.

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: normal.
- `VtRis`: normal.
- `XtermClear`: normal.

### Blink Shell (iOS)

### Bterm

### Butterfly

- Web-based

### Cathode

### CMD.EXE

### ConEMU

### ConsoleZ

### Cool Retro Term

Native `TERM` is `xterm`.

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: normal.
- `VtRis`: scrollback not cleared.
- `XtermClear`: normal.

### Core Terminal

- Doesn’t respect user shell by default.

Native `TERM` is `xterm-256color`.

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: normal.
- `VtRis`: scrollback not cleared.
- `XtermClear`: normal.

### Deepin Terminal

Native `TERM` is `xterm-256color`.

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: normal.
- `VtRis`: scrollback not cleared.
- `XtermClear`: normal.

#### Old GTK version

Native `TERM` is `xterm-256color`.

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: normal.
- `VtRis`: normal.
- `XtermClear`: normal.

### Dinu

### dmenu-term?

### domterm

### dwt

- VTE-based

Native `TERM` is `xterm-256color`.

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: normal.
- `VtRis`: normal.
- `XtermClear`: normal.

### eDEX UI

- Doesn’t respect user shell by default.

Native `TERM` is `xterm-256color`.

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: normal.
- `VtRis`: normal.
- `XtermClear`: normal.

### Electerm

- Doesn’t respect user shell by default.

Native `TERM` is `xterm-256color`.

- Default: `Terminfo`. **The better option would be `VtRis`, but there’s no way to tell we’re
  running in Electerm.**
- `Terminfo`: normal, except scrollbar is weird, like it thinks there’s still all the old content,
  but without showing any scrolling when going up or down.
- `TerminfoScreen`: appears to clear the screen, but really erases the screen without scrolling the
  existing output up, thus losing a screenful of information.
- `TerminfoScrollback`: normal.
- `VtRis`: normal.
- `XtermClear`: as for `Terminfo`.

### Elokab Terminal

- Arabic language support!

### eterm

### Evil VTE

- VTE-based

### ExtraTerm

Native `TERM` is `xterm-256color`.

- Default: `Terminfo`. (Mostly because it’s the least worst and has a chance to get better.)
- `Terminfo`: does not clear scrollback, appears to clear the screen, but really erases the screen
  without scrolling the existing output up, thus losing a screenful of information.
- `TerminfoScreen`: appears to clear the screen, but really erases the screen without scrolling the
  existing output up, thus losing a screenful of information.
- `TerminfoScrollback`: does nothing.
- `VtRis`: behaves like `Terminfo` but also prints `[2m` (badly handled unknown escape).
- `XtermClear`: as for `Terminfo`.

### fbpad

### Fingerterm

### Fluent Terminal (Windows)

### Foot

With `TERM=foot`:

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: appears to clear the screen, but really erases the screen without scrolling the
  existing output up, thus losing a screenful of information.
- `TerminfoScrollback`: normal.
- `VtRis`: normal.
- `XtermClear`: normal.

With `TERM=xterm-256color`:

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: appears to clear the screen, but really erases the screen without scrolling the
  existing output up, thus losing a screenful of information.
- `TerminfoScrollback`: normal.
- `VtRis`: normal.
- `XtermClear`: normal.

### FQTerm

With `TERM=vt102`:

- Default: `Terminfo`.
- `Terminfo`: doesn’t clear scrollback.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: doesn’t support E3.
- `VtRis`: does nothing.
- `XtermClear`: doesn’t clear scrollback.

With `TERM=xterm-256color`:

- Default: `Terminfo`.
- `Terminfo`: doesn’t clear scrollback.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: does nothing.
- `VtRis`: does nothing.
- `XtermClear`: doesn’t clear scrollback.

### Germinal

- VTE-based

Native `TERM` is `xterm-256color`.

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: normal.
- `VtRis`: normal.
- `XtermClear`: normal.

### Guake

### GNOME Terminal

- VTE-based

With `TERM=gnome-256color`:

- Default: `XTermClear`.
- `Terminfo`: behaves like `TerminfoScreen`, doesn’t clear scrollback.
- `TerminfoScreen`: adds a screenful of space to the scrollback before clearing.
- `TerminfoScrollback`: terminfo does not support E3.
- `VtRis`: normal.
- `XtermClear`: normal.

With `TERM=xterm-256color`:

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: adds a screenful of space to the scrollback before clearing.
- `TerminfoScrollback`: normal.
- `VtRis`: normal.
- `XtermClear`: normal.

### Goterminal

### Havoc

### Hyper

- Web-based

### iTerm2

### jbxvt

### jfbterm

### JuiceSSH

### Kermit

### kgx

### Kitty

### KMScon

### Konsole

### Lilyterm

### Liri Terminal

### Literm

### lwt

- VTE-based

### LX Terminal

- VTE-based

### MacTerm

### MacWise

### Mantid

- VTEng-based

### MATE Terminal

### Microsoft Terminal

### Miniterm

- VTE-based

### Mintty

### Miro

### MLTERM

### MobaXTerm

### mrxvt

### mt

### Nautilus Terminal

### Nemo Terminal

### Neovim

### Orbterm

### Pangoterm

- libvterm-based

### Pantheon Terminal

### PowerCmd

### PuTTY

### QML Konsole

### Qt DOM term

### Qterminal

### rcfvt

### ROXTerm

- VTE-based

### Runes

### rxvt

### Sakura

- VTE-based

### sdvt

### Snowflake

### st

With `TERM=st-256color`:

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: also clears scrollback.
- `TerminfoScrollback`: does not support E3.
- `VtRis`: normal.
- `XtermClear`: normal.

With `TERM=xterm-256color`:

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: also clears scrollback.
- `TerminfoScrollback`: does nothing.
- `VtRis`: normal.
- `XtermClear`: normal.

### Station

### sterm

- VTE-based

### stgl

### StupidTerm

- VTE-based

### Sugar Terminal?

### Syncterm

### Taterm

### Tdrop

### Terminal.app (GNUstep)

### Terminal.app (macOS)

### Terminaleco

### Terminalpp

### Terminate

- VTE-based

### Terminator

### Terminol

### Terminology

### Terminus

### Termistor

### Termit

- VTE-based

### Termite

### Termius

### Termy

### Terra

### Tess

### The Terminal

### TreeTerm

### Tilda

### Tilix

### Tinyterm

- VTE-based

### Topinambour

- VTE-based

### Tortosa

- VTE-based

### Ume

- VTE-based

### urxvt

### uterm

- libtsm-based

### uuterm

### Viter

### vt100-parser

### Wayst

### Wezterm

### WindTerm

### Wlterm

- libtsm-based

### wlgxterm

### XFCE4 Terminal

- VTE-based

With `TERM=xfce`:

- Default: `XTermClear`.
- `Terminfo`: behaves like `TerminfoScreen`, doesn’t clear scrollback.
- `TerminfoScreen`: adds a screenful of space to the scrollback before clearing.
- `TerminfoScrollback`: terminfo does not support E3.
- `VtRis`: normal.
- `XtermClear`: normal.

With `TERM=xterm-256color`:

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: adds a screenful of space to the scrollback before clearing.
- `TerminfoScrollback`: normal.
- `VtRis`: normal.
- `XtermClear`: normal.

### xiate

### Xterm

### Yaft

### Yaftx

### Yakuake

- Konsole-based

### Yeah Console

### z/Scope

### Zoc

### Zterm

### Zutty


Serial terminal emulators?
-------------------------

### Bootterm

### Coolterm

### Cutecom

### dterm

### Easyterm

### HTerm

### iserterm

### Microcom

### Minicom

### Moserial

### Picocom

### ssterm

### tio


Multiplexers
------------

### 3mux

### Byobu

### Dvtm

### Eternal Terminal

### Mosh

### mtm

### Screen

With `TERM=screen`:

- Default: `XtermClear`.
- `Terminfo`: normal (does not clear scrollback).
- `TerminfoScreen`: adds a screenful of space to the scrollback before clearing.
- `TerminfoScrollback`: terminfo does not support E3.
- `VtRis`: adds a screenful of space to the scrollback before clearing, does not clear scrollback.
- `XtermClear`: normal.

With `TERM=xterm-256color`:

- Default: `Terminfo`.
- `Terminfo`: **clears scrollback**, even though `TerminfoScrollback` below doesn’t work.
- `TerminfoScreen`: adds a screenful of space to the scrollback before clearing.
- `TerminfoScrollback`: doesn’t do anything.
- `VtRis`: adds a screenful of space to the scrollback before clearing, does not clear scrollback.
- `XtermClear`: normal.

### Tab-rs

### Tmux

With `TERM=tmux-256color`:

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: normal.
- `VtRis`: does not clear scrollback.
- `XtermClear`: normal.

With `TERM=xterm-256color`:

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: adds a screenful of space to the scrollback before clearing.
- `TerminfoScrollback`: normal.
- `VtRis`: does not clear scrollback.
- `XtermClear`: normal.

### ttysterm

### Zellij


Recorders
---------

### Asciinema

### Asciinema Rust

### GoTTY

### Hasciinema?

### ipbt

### Shell in a box

### Shellshare

### Showterm

### T-Rec

### Term to SVG

### Terminalizer

### Termrec

### tmate.io

### ts-player

### tty-share

### TTYcast

### ttyd

### upterm

### webtty
