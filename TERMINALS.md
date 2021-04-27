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

First link the clscli example program into your PATH, e.g.

```
ln -s $(pwd)/target/debug/examples/clscli ~/.local/share/bin/clscli
```

Open the terminal in its default profile, or as it comes when first installed.

Then use `env | grep TERM` to see what the `TERM` and other related variables look like (make note!).

Look into `/usr/share/terminfo` for a terminfo that matches the terminal, or wherever it is on your
system. If there's a separate but official package for the terminal’s terminfo, use it.

First test with the native terminfo: set it either in the terminal’s settings, or use
`env TERM=name $SHELL`, then with the `TERM` the terminal first starts with by default, and finally
with `xterm-256color` if that’s not been covered yet.

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

When “VTE-based” is stated and nothing else, assume this:

Native `TERM` is `xterm-256color`.

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: normal.
- `VtRis`: normal.
- `XtermClear`: normal.


Emulators
---------

### Alacritty

- Version 0.7.2

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

- Version Nightly-develop-2020-01-26-4033a8b

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

- Version 1.0.1

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

- Version 1.1.1

Native `TERM` is `xterm`.

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: normal.
- `VtRis`: scrollback not cleared.
- `XtermClear`: normal.

### Core Terminal

- Version 4.2.0
- Doesn’t respect user shell by default.

Native `TERM` is `xterm-256color`.

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: normal.
- `VtRis`: scrollback not cleared.
- `XtermClear`: normal.

### Deepin Terminal

- Version 5.4.0.6

Native `TERM` is `xterm-256color`.

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: normal.
- `VtRis`: scrollback not cleared.
- `XtermClear`: normal.

#### Old GTK version

- Version 5.0.4.3

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

- Version 0.6.0
- VTE-based

### eDEX UI

- Version 2.2.7
- Doesn’t respect user shell by default.

Native `TERM` is `xterm-256color`.

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: normal.
- `VtRis`: normal.
- `XtermClear`: normal.

### Electerm

- Version 1.11.16
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
- Untested yet

### ExtraTerm

- Version 0.58.0

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

- Version 1.7.2
- Wayland only

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

- Version 0.9.10.1.1.g55d08df

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

- Version 26
- VTE-based

### Guake

- Version 3.7.0
- VTE-based

### GNOME Terminal

- Version 3.40.0
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

- Wayland only

### Hyper

- Web-based

### iTerm2

### jbxvt

### jfbterm

### JuiceSSH

### Kermit

- Version 3.4
- VTE-based

The `kermit` terminfo also exists, but may not be related, and does not work.

### King’s Cross (kgx)

- Version 0.2.1

Native `TERM` is `xterm-256color`.

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: normal.
- `VtRis`: normal.
- `XtermClear`: normal.

### Kitty

- Version 0.20.1

With native `TERM=xterm-kitty`:

- Default: `VtRis`.
- `Terminfo`: does not clear scrollback, appears to clear the screen, but really erases the screen
  without scrolling the existing output up, thus losing a screenful of information.
- `TerminfoScreen`: appears to clear the screen, but really erases the screen without scrolling the
  existing output up, thus losing a screenful of information.
- `TerminfoScrollback`: does not support E3.
- `VtRis`: normal.
- `XtermClear`: normal.

With `TERM=kitty`: as with `xterm-kitty`.

With `TERM=xterm-256color`:

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: appears to clear the screen, but really erases the screen without scrolling the
  existing output up, thus losing a screenful of information.
- `TerminfoScrollback`: erases scrollback and screen, but does not clear them (can be scrolled, but
  all is blank).
- `VtRis`: normal.
- `XtermClear`: normal.

### KMScon

### Konsole

- Version 21.04.0

With native `TERM=xterm-256color`:

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: appears to clear the screen, but really erases the screen without scrolling the
  existing output up, thus losing a screenful of information.
- `TerminfoScrollback`: normal.
- `VtRis`: doesn’t clear scrollback, appears to clear the screen, but really erases the screen
  without scrolling the existing output up, thus losing a screenful of information.
- `XtermClear`: normal.

With `TERM=konsole`:

- Default: `XtermClear`.
- `Terminfo`: doesn’t clear scrollback, appears to clear the screen, but really erases the screen
  without scrolling the existing output up, thus losing a screenful of information.
- `TerminfoScreen`: appears to clear the screen, but really erases the screen without scrolling the
  existing output up, thus losing a screenful of information.
- `TerminfoScrollback`: does not support E3.
- `VtRis`: doesn’t clear scrollback, appears to clear the screen, but really erases the screen
  without scrolling the existing output up, thus losing a screenful of information.
- `XtermClear`: normal.

### Lilyterm

- libvte-based

### Liri Terminal

- Version 0.2.0
- Doesn’t respect user shell by default.

Native `TERM` is `xterm-256color`.

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: normal.
- `VtRis`: doesn’t clear scrollback.
- `XtermClear`: normal.

### Literm

### lwt

- Version 2020-12-02
- VTE-based
- Doesn’t respect user shell by default.

Native `TERM` is `xterm-256color`.

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: adds a screenful of space to the scrollback before clearing.
- `TerminfoScrollback`: normal.
- `VtRis`: doesn’t clear scrollback.
- `XtermClear`: normal.

### LX Terminal

- Version 0.4.0
- VTE-based

### MacTerm

### MacWise

### Mantid

- Version 1.0.6
- VTE-based

### MATE Terminal

- Version 1.24.1
- VTE-based

### Maui Station

- Version 1.2.1

Native `TERM` is `xterm`.

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: normal.
- `VtRis`: doesn’t clear scrollback.
- `XtermClear`: normal.

### Microsoft Terminal

### Miniterm

- Version 1.7.0
- VTE-based

### MinTTY (Windows)

### Miro

### MLTERM

- Version 3.9.0

Native `TERM` is `xterm`.

- Default: `Terminfo`. (No real good option here.)
- `Terminfo`: doesn’t clear scrollback, appears to clear the screen, but really erases the screen
  without scrolling the existing output up, thus losing a screenful of information.
- `TerminfoScreen`: appears to clear the screen, but really erases the screen without scrolling the
  existing output up, thus losing a screenful of information.
- `TerminfoScrollback`: does nothing.
- `VtRis`: doesn’t clear scrollback, appears to clear the screen, but really erases the screen
  without scrolling the existing output up, thus losing a screenful of information.
- `XtermClear`: as for `Terminfo`.

### MobaXTerm

### mrxvt

### mt

### Nautilus Terminal

- Version 3.5.0
- VTE-based

### Nemo Terminal

- Version 4.8.0
- VTE-based

### Neovim

- Version 0.4.4

`TERM` is inherited. With `xterm-256color`:

- Default: `Terminfo`. (No real good option here.)
- `Terminfo`: doesn’t clear scrollback, appears to clear the screen, but really erases the screen
  without scrolling the existing output up, thus losing a screenful of information.
- `TerminfoScreen`: appears to clear the screen, but really erases the screen without scrolling the
  existing output up, thus losing a screenful of information.
- `TerminfoScrollback`: does nothing.
- `VtRis`: doesn’t clear scrollback, appears to clear the screen, but really erases the screen
  without scrolling the existing output up, thus losing a screenful of information.
- `XtermClear`: as for `Terminfo`.

### Orbterm

### Pangoterm

- libvterm-based

### Pantheon/Elementary Terminal

- Version 5.5.2

Native `TERM` is `xterm-256color`.

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: normal.
- `VtRis`: normal.
- `XtermClear`: normal.

### PowerCmd

### PuTTY

- Version 0.74

With native `TERM=xterm`:

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: normal.
- `VtRis`: does not clear scrollback.
- `XtermClear`: normal.

With `TERM=putty`:

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: does nothing.
- `VtRis`: does not clear scrollback.
- `XtermClear`: normal.

### QML Konsole

- Version 0.1.r2.g81e74ad

Native `TERM` is `xterm`.

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: does nothing.
- `VtRis`: does not clear scrollback.
- `XtermClear`: normal.

### Qt DOM term

### Qterminal

- Version 0.16.1

Native `TERM` is `xterm-256color`.

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: normal.
- `VtRis`: normal.
- `XtermClear`: normal.

### rcfvt

- Version r66.d390d61

Native `TERM` is `xterm-256color`.

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: normal.
- `VtRis`: normal.
- `XtermClear`: normal.

### ROXTerm

- Version 3.10.1
- VTE-based

### Runes

### rxvt

### Sakura

- Version 3.8.1
- VTE-based

### sdvt

### Snowflake

### st

- Version 0.8.4

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

### sterm

- Version 0.1.2
- VTE-based

Native `TERM` is `xterm-256color`.

There’s no scrollback at all, so it’s impossible to know how things are really handled, but 🤷.

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: normal.
- `VtRis`: normal.
- `XtermClear`: normal.

### stgl

### StupidTerm

- Version 1.r24.gf824e41
- VTE-based

### Syncterm

- Version 1.1

Native `TERM` is `syncterm`.

- Default: `VtRis`.
- `Terminfo`: no terminfo found.
- `TerminfoScreen`: no terminfo found.
- `TerminfoScrollback`: no terminfo found.
- `VtRis`: normal.
- `XtermClear`: does not clear scrollback.

### Taterm

- Version 12

Native `TERM` is `xterm-256color`.

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: normal.
- `VtRis`: normal.
- `XtermClear`: normal.

### Tdrop

### Terminal.app (GNUstep)

### Terminal.app (macOS)

### Terminaleco

### Terminalpp

### Terminate

- VTE-based
- Untested yet

### Terminator

### Terminol

### Terminology

### Terminus

### Termistor

### Termit

- VTE-based
- Untested yet

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
- Untested yet

### Topinambour

- VTE-based
- Untested yet

### Tortosa

- VTE-based
- Untested yet

### Ume

- VTE-based
- Untested yet

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

- Version 0.8.10
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

- Version 21.04.0
- Konsole-based

Native `TERM` is `xterm-256color`.

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: appears to clear the screen, but really erases the screen without scrolling the
  existing output up, thus losing a screenful of information.
- `TerminfoScrollback`: normal.
- `VtRis`: doesn’t clear scrollback, appears to clear the screen, but really erases the screen
  without scrolling the existing output up, thus losing a screenful of information.
- `XtermClear`: normal.

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

- Version 1.3.2

`TERM` is inherited, here with `xterm-256color`:

There’s no scrollback at all, so it’s impossible to know how things are really handled, but 🤷.

- Default: `Terminfo`.
- `Terminfo`: normal.
- `TerminfoScreen`: normal.
- `TerminfoScrollback`: does nothing.
- `VtRis`: normal.
- `XtermClear`: normal.

### mtm

### Screen

- Version 4.08.00

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

- Version 3.2

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
