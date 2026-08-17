This is a simple tool to interactively play sine waves at different pitches in terminals.

It has been my playground for trying out [`clap`](https://crates.io/crates/clap), [`crossterm`](https://crates.io/crates/crossterm), [`web-audio-api`](https://crates.io/crates/web-audio-api), and [`evdev`](https://crates.io/crates/evdev). `clap` and `crossterm` are good crates for building applications with interfaces in terminals, and `web-audio-api` is a friendly crate providing functionality and interfaces similar to the [Web Audio API](https://developer.mozilla.org/docs/Web/API/Web_Audio_API). `evdev` is used to provide an alternative way to detect key events on Linux.

The application is built based on these assumptions:

-   ANSI QWERTY keyboard layout (US)
-   (TODO)

Tested and working environments:

-   [Windows Terminal](https://github.com/microsoft/terminal) on Windows 11
-   Windows Console Host on Windows 11
-   GNOME Terminal, using "evdev" mode

Tested and not working environments:

-   VS Code integrated terminal on Windows 11
-   [mintty](https://github.com/mintty/mintty), which is shipped with MSYS2

Note: the default backend of `web-audio-api` seems to be dynamically linked on Linux, and it needs certain environment variables to work. Be aware that the `sudo` command does not preserve the environment by default.

Unfortunately, there is currently no further documentation. I tried to keep the code relatively easy to read, though.

Code in this repository is licensed under `0BSD`; see LICENSE for details.