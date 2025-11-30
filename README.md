# Notepad--


https://github.com/user-attachments/assets/22fc5f11-5ac9-48db-aa66-fb47db472f16


## A text editor developed in Rust

Notepad-- is a desktop text editor implemented in Rust and Dioxus. Supports standard text editor functionality, including numerous keybindings and keyboard navigation.


The editor is implemented from low-level constructs only (using Dioxus). All features are implemented from scratch, like:
 - Mouse input handling
 - Keyboard shortcut handling
 - Rendering of the text editor
 - Text selection
 - Editor history:
   - Events creation and aggregation into more complex elements (e.g. multiple non-whitespace inputs into one larger input)
   - Every action in the editor has a history element (e.g. Alt+Up), and it's reversible alternative
   - Interactive history list



## Functionality
- General Editor Functionality
- File handling (open, create, rename, delete)
- Automatic File Explorer Update
- Undo/redo
- History of recent changes
- Keyboard shortcuts
- Find
- (Multiple) Terminal Support

History

https://github.com/user-attachments/assets/4c5dbb30-1a9a-4d79-994a-a5dd6bdb1713


Find

https://github.com/user-attachments/assets/bc60a448-5937-4c30-b8be-c10d2c2a7d03

Terminal

https://github.com/user-attachments/assets/136f90bd-aa58-4f9c-868a-a08a1acd5ecd


# Running the application

install Dioxus: 
```
> cargo install dioxus-cli
``` 
(or follow https://dioxuslabs.com/learn/0.6/getting_started/)

run the application:
```
> dx serve -r
```
 
