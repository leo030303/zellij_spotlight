# zellij_spotlight
A plugin for [Zellij](https://zellij.dev/) to run commonly used commands, such as to launch TUI apps.
Heavy inspiration from [zellij-forgot](https://github.com/karimould/zellij-forgot) and [harpoon](https://github.com/Nacho114/harpoon)

![demo](https://github.com/leo030303/zellij_spotlight/blob/main/assets/zellij_spotlight_demo.gif)

## Installation
### Prebuilt binary
Simply grab the latest .wasm file from the releases page, place it in the zellij plugins folder, and update your config.kdl.
### Build from source
  gh repo clone leo030303/zellij_spotlight
  cd zellij_spotlight
  cargo build --release
  mv target/wasm32-wasi/release/zellij_spotlight.wasm ~/.config/zellij/plugins/

## Configuration
Add the following to your config.kdl
  shared_except "locked" {
      bind "Home" {
          LaunchOrFocusPlugin "file:~/.config/zellij/plugins/zellij_spotlight.wasm" {
              "File Manager" "joshuto"
              "Editor" "hx"
              "Resource Manager"  "btm"
              "Git Manager"  "gitui"
              "Stack Overflow"  "so"
              "Wikipedia"  "wiki-tui"
              "Email Client"  "meli"
              "Web Browser"  "elinks"
              floating true
              move_to_focused_tab true
          }
      }
      bind "Ctrl f" { ToggleFocusFullscreen; } // optional but I find a shortcut to fullscreen the current pane to be useful
  }
Add whichever commands you want to use, arguments can be included too in the same string as the command. You can change the keybinding too.

## Usage
* Use Up and Down arrows to move through list
* Type any characters to filter output
* Press Enter to run the selected command
* Use Esc or Ctrl C to close the plugin

### Contributing
Feel free to open an issue if you find a bug or what to suggest an improvement.
