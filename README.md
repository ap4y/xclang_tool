# xclang

CLI for common editor actions for Objective-C. `xclang` focuses on solving common `clang` integration problems into text editors. Currently projects main focus is `iOS` and `MacOS` platforms.

## Problem

While it's really tempting to write `Objective-C` application with your favorite editor, it can be a hard task without proper syntax checker and code completion that `XCode` provides by default. Luckily, [clang tooling](http://clang.llvm.org/doxygen/index.html) can solve most of the problems and it probably used by `XCode` internally. Unfortunately, `clang` tooling requires some preparation steps and it's exposes low-level `C`/`C++` API. `xclang` solves preparation steps for you and exposes simple `CLI` that can be integrated into your editors.

## Design

`xclang` tries to be agnostic to the editors, but it returns results in format that is usable by popular plugins for `Emacs` (`flycheck`, `company`, `autocomplete`) and `Vim`(`syntastic`).

At this point `xclang` doesn't work as backend server, on the other hand it works pretty fast, in most cases latency in `Emacs` is barely noticeable. This can be chance in future.

## Commands

At this point `xclang` provides this operations:

- `compilation-database`. Serves as preparation step for other operations, it builds your project with `xcodebuild` CLI, parses output into [Compilation Database](http://clang.llvm.org/docs/JSONCompilationDatabase.html) and writes it into `compile_commands.json`. All other commands will try to find recursively `compile_commands.json` starting from current folder. You can run this command in `continuous` mode (`--continuous`), which will refresh `compilation database` each time you are adding new file.

- `syntax-check`.

## Compilation

`xclang` is written with [Rust](http://www.rust-lang.org), so you will need a `Rust` compiler.

In order to work properly with `iOS` it requires `clang` version that is bundled with `XCode`, so you will need to install `XCode`.

Compilation process is super simple, just run: `make` from the project directory. You can also run tests with `make test`.

`xclang` will be compiled to the `bin` folder. You may consider symlinking it to the directory that is in your `PATH` environment variable.

## Editor Integration

Currently only `Emacs` integration is provided (`company` and `flycheck`). `Syntastic` integration for `Vim` may be provided. Unfortunately, I'm not aware of good completion plugin for `Vim`, thus it's a bit hard to provide `lightweight` completion solution.

### Flycheck

Syntax check for `Emacs` provided as a [flycheck](https://github.com/flycheck/flycheck) plugin, so it has to be installed. Once you have `flycheck` you can just `load-file` on the plugin file from the `emacs` folder.

Syntax check happens automatically for `errors` and `warnings`. If it's not happening, you may consider checking if `xclang` is in `flycheck-checkers` list. If you don't have `xclang` in your `PATH` you can set path to the `xclang` directly in `flycheck-xclang-executable` variable.

## Company

Code completion for `Emacs` provided as a [company-mode](https://github.com/company-mode/company-mode) plugin. It requires [yasnippet](https://github.com/capitaomorte/yasnippet) for snippet expansion. Once you have installed plugins you can `load-file` on the plugin file from the `emacs` folder, `company` mode and `yasnippet` minor mode have to be active.

If you don't have `xclang` in your `PATH` you can set path to the `xclang` directly in `company-xclang-executable` variable.
