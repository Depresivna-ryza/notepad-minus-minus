# Notepad--

## A text editor with additional features developed in Rust

https://github.com/user-attachments/assets/a2ca1de7-2330-45e1-b1d7-6d15ac586ff9



https://github.com/user-attachments/assets/7ac02b67-6460-4a61-81df-69d871cdd611



https://github.com/user-attachments/assets/56763fc2-9433-4fdb-b646-92292afcf72c


### Main description

Create a text editor (desktop application) that will support basic text file handling. The program will be able to open folders and create files of different types. Control is possible using both mouse and keyboard.

In the settings it is possible to specify the number of steps (undo/redo) that the editor remembers. The application will support standard editor functionality including keyboard shortcuts, e.g. move with arrow keys, jump with ctrl + arrow keys, select with shift + arrow keys, copy & paste & cut, undo & redo, find & replace.

The application can run a "live share" mode tha


t will create a watch session and return a code and password for the created session. In other instances of the application, this session can be connected to and the code watched. How this mode is implemented (i.e. cloud vs. P2P) is up to you. Adding different themes to the UI will be appreciated.

### Implemented Functionality
- General Editor Functionality
- File handling (open, create, rename, delete)
- Automatic File Explorer Update
- Undo/redo
- History of recent changes
- Keyboard shortcuts
- Find
- (Multiple) Terminal Support



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
 
