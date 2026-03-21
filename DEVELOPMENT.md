# Development Setup

## Prerequisites

Install [mise](https://mise.jl) — it manages the Rust toolchain for this project. You do not need Rust installed globally.

```sh
# macOS
brew install mise

# or via the installer script
curl https://mise.run | sh
```

Then activate mise in your shell (add to `~/.zshrc` or `~/.bashrc`):

```sh
eval "$(mise activate zsh)"   # zsh
eval "$(mise activate bash)"  # bash
```

## Toolchain setup

From the repository root, run:

```sh
mise install
```

This reads `mise.toml` and installs stable Rust. Verify:

```sh
cargo --version
rustc --version
```

## Build & test

```sh
cargo build         # compile
cargo test          # run all tests
cargo clippy        # lints

cargo run -- scale C major      # try it out
cargo run -- scale A aeolian
```

## VS Code

1. Open the repository folder (`File > Open Folder`)
2. Accept the recommended extensions prompt (rust-analyzer, even-better-toml, crates) — or install them from `.vscode/extensions.json` manually
3. Format-on-save and the 120-column ruler are pre-configured via `.vscode/settings.json`

## Zed

1. Open the repository folder (`zed .` or `File > Open`)
2. Open the Extensions panel (`cmd+shift+x`) and install **Rust**
3. Format-on-save is pre-configured via `.zed/settings.json`

Zed uses rust-analyzer under the hood, so completions, go-to-definition, and inline errors work out of the box once the extension is installed.

## LazyVim

LazyVim does not have per-project config, so setup is one-time on each machine.

1. Install [rustaceanvim](https://github.com/mrcjkb/rustaceanvim) — the modern rust-analyzer plugin for Neovim. Add it to your LazyVim extras or plugins config:

   ```lua
   -- ~/.config/nvim/lua/plugins/rust.lua
   return {
     "mrcjkb/rustaceanvim",
     version = "^5",
     lazy = false,
   }
   ```

2. Install the [editorconfig-nvim](https://github.com/editorconfig/editorconfig-vim) plugin to pick up `.editorconfig` (indentation, line endings, rulers):

   ```lua
   -- ~/.config/nvim/lua/plugins/editorconfig.lua
   return { "editorconfig/editorconfig-vim" }
   ```

   LazyVim 14+ includes EditorConfig support natively — no plugin needed if you're on a recent version.

3. Open the project directory. rust-analyzer starts automatically and provides completions, diagnostics, and formatting via `rustfmt`.
