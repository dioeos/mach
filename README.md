# Mach

**M**acro **A**nd **C**ommand **H**elper is a lightweight, cross-platform Rust-powered application that lets you trigger an overlay via a global hotkey to display your user-defined macros and commands from a JSON file.

> [!WARNING] > **Why MACH?** MACH exists for a simple reason: to stop you from constantly Googling or forgetting needed commands and macros.  
> Whether it's using a CLI command, executing some VS Code keybinding, or just running that one obscure script you can never remember, MACH keeps all your important commands and macros just one hotkey away...

## Demo

https://github.com/user-attachments/assets/9d1562a2-78e0-4d8e-b136-02ec7f858263

## Installation

**Step 1:** Ensure you have [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed on your system. If you already have this, skip to the next step.

**Step 2:** You can install the application using `cargo`:

```
cargo install mach-keys
```

Once installed, you can run the application from anywhere.

## Usage

The JSON file that Mach reads to display your macros and commands can be found at:

- Windows: `%APPDATA%\mach\config\macros.json`
- macOS: `~Library/Application Support/mach/config/macros.json`
- Linux: `~/.config/mach/config/macros.json`
  By default the file contains the following content:

```
[
    {
        "keys": "Alt + /",
        "action": "Open MACH"
    },
    {
        "keys": "Alt + /",
        "action": "Hide MACH"
    },
    {
        "keys": "Ctrl + N",
        "action": "New browser tab"
    },
]
```

Change this file as necessary, but ensure it follows proper JSON syntax because Mach will fail to load if the file is incorrectly formatted.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

[![Made with Slint](https://raw.githubusercontent.com/slint-ui/slint/master/logo/MadeWithSlint-logo-whitebg.png)](https://slint.dev)
<br>
This project uses the [Slint UI toolkit](https://slint.dev), licensed under the [Slint Royalty-Free Desktop, Mobile, and Web Applications License](https://slint.dev/terms-and-conditions#royalty-free).
