# ToDoTUI
Terminal ToDo app written in rust.

ToDo-Liste:
<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/2ca40af4-0fb5-45e4-9c72-d73b4b15d608" />

Tasks verwalten:
<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/eefde064-0d3c-41e6-895a-eee0f92861ad" />

# Installation
The app is build with rust, so to compile and run it cargo and rustc is needed. This can be installed via the nix package manager.
``` shell
nix develop
```
After installing it the app can be started with cargo. I build it on linux and im not shure if it works on windows out of the box.
``` shell
cargo run
```

# Usage
Create a task:

A new task can be created by pressing enter on the create new task element in the ToDo list.

Edit a task:
- To navigate a task you can cycle through the fields using the Tab key.
- To save changes press enter
- To Finish a task select the finish button and press enter
- To Delete a task select the delete button and press enter

Navigate the ToDo list:

To select a different task than the highlighted one press Tab
