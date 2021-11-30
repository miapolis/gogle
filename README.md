## Gogle is simple utility for getting a Google URL for a query

##### Use case

1. Someone asks a question in a chat that can be answered by just asking Google the same thing
2. You copy their query and press a shortcut and your clipboard is modified with a Google URL
3. You paste in the Google URL

### Instructions:

##### Requirements:

- Cargo and the Rust toolchain installed

##### Linux

Arch:
Gogle is available on the AUR at: https://aur.archlinux.org/packages/gogle-git/

All other distributions:

1. Clone the repo and navigate to the project directory
2. Run `cargo build --release` and wait for the build to finish
3. Find the resulting 'gogle' binary in the target/release directory
4. Copy the binary to your /bin directory or any directory in your PATH environment variable

###### Setting up the shortcut:

Gnome:

1. Navigate to _Settings/Keyboard/Keyboard Shortcuts/View and Customize Shortcuts/Custom Shortcuts_
2. Hit the '+' button and set the shortcut to just `gogle`
3. Set the name and the shortcut to whatever you want, personally I use `Super + Shift + G`

### Note

I haven't had the ability yet to expirement on other platforms and desktop environments to see how it should be set up there, but I assume the process would be pretty similar:

1. Building the project and placing the binary somewhere in PATH
2. Adding a custom shortcut which opens a terminal with the 'gogle' command
