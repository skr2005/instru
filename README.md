This is a simple tool to interactively play sine waves at different pitches in terminals. 

It has been my playground for trying out [`clap`](https://crates.io/crates/clap), [`crossterm`](https://crates.io/crates/crossterm), and [`web-audio-api`](https://crates.io/crates/web-audio-api). `clap` and `crossterm` are good crates for building applications providing interfaces in terminals, and `web-audio-api` is friendly because one can learn conceptions from many existing documentations, blog posts, Q&As about [Web Audio Api](https://developer.mozilla.org/docs/Web/API/Web_Audio_API).

The application is built based on these assumptions:

-   ANSI QWERTY keyboard layout (US)
-   (TODO)

Tested and working environments:

-   [Windows Terminal](https://github.com/microsoft/terminal) on Windows 11
-   Windows Console Host on Windows 11

Tested and not working environments:

-   VS Code integrated terminal on Windows 11
-   [mintty](https://github.com/mintty/mintty), which is shipped with MSYS2

Unfortunately, there is currently no furthur documentation. I tried to keep the code not too much to read, though.

Code in this repository is licensed under 0BSD, see LICENSE for the details.