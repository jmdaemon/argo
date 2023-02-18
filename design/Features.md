# Features

- Filename/path aliases. Write up a rust library to create aliases at runtime that will be expandable in both apps programmatically,
    shell scripts, and even guis.
    - These will be used in the path field to expand file path aliases.
    - These will also be used at the shell level to navigate directories.
- Emotes: An emote mode that references files directly without interfering with shell scripts, executable names, etc.
- Toggleable modes/sets of emotes or aliases for files. Let's say you want to use conflicting
    alias names or use another similar alias for the same file, you can toggle which mode is used at runtime.
- Structured file finder mode: Find structured files more easily in databases according to a defined schema.
    - Ability to apply/edit tags
    - Ability to search by tags in a given database
    - Automatic tagging ability given a function.
- Automatic presentation: Automatically determine/choose the best view to represent your files at runtime.
    - Provide user the ability to manually override the view.
- Most used toolbar: Toolbar filled with useful goodies such as (Add, Rename, Remove, Properties)
    - Add support for custom toolbar widgets?
- Module support: A plugin system for customizing the user interface and for providing
    additional functionality not present in the original application.
    - Provide support for modules written in Rust only for now.
    - Create "usage" module that tracks the biggest directories,
        most opened/frequently visited files, most modified files,
        and provides graphics for them that can be exported.
- Customization: Provide options to both reset and save the current user's settings
    in a specific profile.
    - Backup important settings and allow for enabling/disabling of backups.
- Persistence: User's previous session is restored on open.
- Multiple views: Provides both a gui, and a tui
- Smart saving: Additional options can be passed to the file browser on open such as
    - List of tags to save the file as.
    - Filename
    - Automatic checksum checking
- Customizable Columns:
    Name | Type | Extension | Last Modified | Last Created | Size | Checksum
    Name | Type | Extension | Title | Artist | Size
    - Columns can be further configured in the user's profile.
- Powerful mnemonic shortcuts: Shortcuts for
    - Rename, Delete
    - Referencing bookmarks
    - Navigating to views (Search tags panel, Edit tags panel, Files panel)
    - Toggling Item view mode/Icon view mode.
    - Navigating between folder tabs
    - Enabling/disabling left and right side panels
    - Opening up the settings panel
    Shortcuts should also be modal, and any number of modes may be added (for now lets just add 3)
        (I:insert mode, N:normal mode, V:visual)
        (I:, N:Navigation mode, V:View mode)
        Nav Mode: Enables navigation between open folder tabs
        View Mode:
    Proposed Defaults List:
        "Rename": "r"
        "Delete": "dd"
        "Copy": "ctrl+c"
        "Paste": "ctrl+v"
        Bookmarks/Favorites:
            "[[Your Directory Name]]": "Optional Shortcut"
        "Tags Search Panel": "tp"
        "Tags Add Panel": ""
- Powerful dropdown list
    - When used in "save file" mode i.e as a file save dialog, the dropdown list must
        contain at least these functions.
        - Make New Folder
        - New File
        - Open
        - Open With
        - Open with Terminal
            make new file, open with
