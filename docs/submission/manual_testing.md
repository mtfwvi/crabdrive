# Manual system test: Checklist

This is the set of assertions to test in order to cover all relevant functionality.
Styling and visual integrity is tested simply by checking everything looks as expected.

## Setup
- [ ] Run server
- [ ] Build frontend in release mode


***


## Basic
- [ ] The user interface is visible under the root URL `http://localhost:2722`

## Authentication
- [ ] IF the user is not logged in: UPON opening the path `/` or any path of form `/<NodeID>` the page redirects to login
- [ ] Login and register pages are visible under `/login` and `/register` and allow registering/logging in
- [ ] Submitting is prevented if username is empty or password is to short
- [ ] UPON trying to submit a registration request with a password of insufficient length: a warning is shown
- [ ] UPON submission: a toast shows the progress
- [ ] IF any error occurs during login: another toast replaces the progress-toast and shows a usable error message
- [ ] A button allows switching between login and register

## Accept Share Page
- [ ] IF the user opens a share link, then
    - [ ] IF the shared node is a file, it is immediately downloaded
    - [ ] IF the shared node is a folder, it is opened

## Home page
### Side bar
- [ ] A button allows the user to log out
- [ ] UPON logout: the page redirects to the login page
- [ ] Buttons allow opening the users root or trash node
- [ ] UPON redirecting to root or trash: the details panel closes
- [ ] Any toasts appear at the bottom of the side bar
- [ ] A button allows dismissing all toasts

### Folder view
- [ ] The folder view contains
    - [ ] IF no node is selected (= if the path `/` is open): a message that notes that
    - [ ] IF there is an error loading the path or children of the node: an error message is shown in the box instead
    - [ ] the path from the root node to the currently open node, or the title Trash/Shared and appropriate icons if the trash or shared roots are open
    - [ ] An overview of:
        - [ ] IF a normal folder is open: all children of the current node
        - [ ] IF trash is open: deleted nodes
        - [ ] IF shared is open: all nodes shared and accepted by the current user
    - [ ] The items:
        - [ ] each includes the file name and an icon corresponding to the file extension
        - [ ] each includes an appropriate icon if the file has been shared with other users
        - [ ] separated by type (folder, file, link)
        - [ ] clicking a node opens the details panel
        - [ ] double clicking a folder opens it
        - [ ] OR IF there are no children: an appropriate message
    - [ ] A bottom bar of icons, containing
        - [ ] IF a normal node is open: Buttons to upload files and create folders
        - [ ] IF the trash is open: A button to empty the trash

#### Create folder dialog
- [ ] UPON clicking the corresponding button: a dialog opens for the user to input the folder name
- [ ] UPON opening of the dialog: the input is focused
- [ ] UPON clicking submit, or on enter: the dialog closes and the page reloads to show the folder has been created
- [ ] IF the dialog is closed without submitting: nothing happens

#### Create file dialog
- [ ] UPON clicking the corresponding button: a dialog opens for the user to select the files to upload
- [ ] UPON selecting one or more files (drag and drop or dialog on click): the files are listed in the dialog
- [ ] UPON confirming the selection: the dialog closes and the upload starts, signified by a toast
- [ ] UPON finishing: the page reloads to show the files have been created alongside a success toast, OR displays a helpful error message in a toast
- [ ] IF the dialog is closed without submitting: nothing happens

### Details panel
- [ ] The side panel contains
    - [ ] the node name
    - [ ] its time of creation
    - [ ] its time of last modification
    - [ ] its owner, if available
    - [ ] the other users this file has been shared with, if applicable
    - [ ] IF the node is a file:
        - [ ] its size in human readable form, if available
        - [ ] its type, if available
        - [ ] a button to download the file
    - [ ] a button with a hover menu for modifying the node, that is
        - [ ] renaming
        - [ ] moving
        - [ ] moving to trash
        - [ ] IF the node is a file: uploading a new revision
    - [ ] a button to close the panel
- [ ] IF the file name is too long: It's shortened with an ellipsis near, NOT *at* the end of the name

#### Download button
- [ ] UPON clicking the download button: the file is downloaded (and opened depending on the browser) [CHECK FILE INTEGRITY!]
- [ ] IF there is some error: a toast shows a useful error message

#### Modify node menu
- [ ] UPON selecting a menu item: the corresponding action is initiated
- [ ] FOR rename, this means a dialog allows the input of the new file name, and upon confirmation
- [ ] FOR share, this means:
    - [ ] a toast is shown that includes a link, which the receiving user can use to accept the share
    - [ ] the link is immediately copied to the user's clipboard
- [ ] FOR all action:
    - [ ] UPON completion: the folder view reloads its children list to immediately show the changes
    - [ ] IF there is an error: a toast shows a useful error message