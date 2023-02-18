# Interface

How it works:

When Argo starts up, it will open the current directory to the current user's /home/ directory.
Listed will be the many files in that directory.

Argo will offer various functions for use:
1. Default file view mode
2. Tagged / Structured file view mode

## Widgets

### Nav Bar

The navigation bar will contain at least these buttons:
- Back button
- Forward button
- Home Button
- Parent folder

These buttons will navigate the current folder view accordingly.

### Directory Bar

The directory bar is used to select and navigate the currently open directory of files. It should:
- Display truncated file paths if the text will run "off screen".
- Display the miniature file icon at the start of the bar.
- Display the file paths separated by '>' marks, but on click/edit, show full filepaths with '/'

### Files View

The files view should:

- Display files with thumbnails
- Support separate "views" for files
    - Icon View mode that displays large icons & thumbnails front and center, with the filenames below
    - List View mode that displays small icons, with file attributes
- Show file attributes in columns: Name, Size, Type, Modified
- Be toggleable for different kinds of directories:
    - Folders with general files display using List View
    - Folders with images display using Icon View
    - Folders with audio files display using List View (but maybe offer different toggleable columns by default)
- List View:
- Files displaying with List View are not too big.
    - Columns have a set max limit of 64.
    - Anything that goes over this limit automatically is truncated
- Metadata containing layout settings for certain directories is saved locally.
- By default, Argo will choose the best layout settings automatically for the directory available.
- Argo offers a button to reset the layout settings for a directory.

- When searching using the tags panel, the main view is replaced with a flat files view widget that displays only files, no folders

### View Toggle Bar

The view toggle bar will offer two icon buttons to change the view type of the Files View widget.
- Icon View Button
- List View Button

<!--- Structured View-->
    <!--- Structured view will offer an additional Tags Panel used for searching-->
<!--- Unstructured View-->
    <!--- Default file view, in icon/attribute mode-->

### Tool Bar

Tool bar will offer certain actions:
- Rename currently selected item
- Make New Folder in currently open directory
- Delete currently selected item

### Alias Panel

- Reference files using aliases (like emotes).
- Toggleable
- Contains an input bar:
    - The input bar is searchable with a dropdown menu that shows similar matches/all matches.
- Autocomplete / Tab complete
- Uses thumbnail widget/preview widget
- Alias sets are toggleable.
    - If two sets contain conflicting definitions, they will not be allowed to load at the same time.
- Toggleable modes activate groups of sets at a time

### Tags Panel

- Perform db lookups with treeleaves
- Toggleable
- Contains an input bar:
    - The input bar is searchable with a dropdown menu that shows similar matches/all matches.
- Uses thumbnail widget/preview widget

### Favorites/Access Panel

- Contains labels and folders in a flat hierarchy.
- Important widgets are: Recents, Favorites, Devices, Trash, Network.

## Potential Features

- Detachable/reattachable widgets. For greater config capability, making every widget dockable/adjustable/attachable
    will make it easier for users to set up their file manager the way they like it.
- Support for custom schemas with Treeleaves.
