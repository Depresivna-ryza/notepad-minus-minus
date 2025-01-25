# Notepad--

## A text editor with additional features developed in Rust

### Main description

Create a text editor (desktop application) that will support basic text file handling. The program will be able to open folders and create files of different types. Control is possible using both mouse and keyboard.

In the settings it is possible to specify the number of steps (undo/redo) that the editor remembers. The application will support standard editor functionality including keyboard shortcuts, e.g. move with arrow keys, jump with ctrl + arrow keys, select with shift + arrow keys, copy & paste & cut, undo & redo, find & replace.

The application can run a "live share" mode that will create a watch session and return a code and password for the created session. In other instances of the application, this session can be connected to and the code watched. How this mode is implemented (i.e. cloud vs. P2P) is up to you. Adding different themes to the UI will be appreciated.

### Functionality
- File handling (open, create)
- Undo/redo
- History of recent changes
- Keyboard shortcuts
- Find & replace
- Realtime sharing sessions

### Requirements
- Controllable with mouse and keyboard
- Has to have settings menu for configuration
- Has to support standard text editor functionality (shift select, ctrl movement, copy&paste)
- Users are able to login to live sessions using code + password
- ?? Multiple UIjnjjjijikoko themes