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

### Aminal

### An9wer-st

### Android Terminal Emulator

### Archipelago

- Web-based

### ate

### Blink Shell (iOS)

### Bterm

### Butterfly

- Web-based

### Cathode

### CMD.EXE

### ConEMU

### ConsoleZ

### Cool Retro Term

### Core Terminal

### Deepin Terminal

#### Old GTK version

### Dinu

### dmenu-term?

### domterm

### dterm

### dwt

### Edex UI

### Electerm

### Elokab Terminal

- Arabic language support!

### eterm

### Evil VTE

- VTE-based

### ExtraTerm

### fbpad

### Fingerterm

### Fluent Terminal

### Foot

### Fqterm

### Germinal

- VTE-based

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
