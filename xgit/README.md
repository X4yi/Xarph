# XGit - Interactive Git Workflow Tool

XGit is a Bash-based CLI tool designed to simplify and standardize Git workflows for small teams working on complex projects. It provides an interactive, menu-driven interface that actively requests user input to prevent mistakes and ensure safe operations.

## Features

- **Interactive CLI**: Persistent menu system with clear prompts
- **Branch Management**: Create, switch, delete branches with validation
- **Sync Operations**: Pull, push, fetch with strategy selection
- **Commit System**: Stage and commit with confirmation
- **Restore & Undo**: Safe reset and discard operations
- **Conflict Resolution**: Manual conflict handling (no auto-resolution)
- **Stash Management**: Stash, apply, list, and drop stashes
- **Safety Layer**: Confirmations for risky operations
- **Project Awareness**: Detects Git repositories and shows status

## Installation

1. Clone or download the XGit project
2. Navigate to the XGit directory
3. Make the script executable:
   ```bash
   chmod +x xgit.sh
   ```

### Global Installation (Optional)

To install globally on your system:

```bash
sudo cp xgit.sh /usr/local/bin/xgit
sudo chmod +x /usr/local/bin/xgit
```

Then you can run `xgit` from any directory.

## Usage

1. Navigate to your Git repository
2. Run the tool:
   ```bash
   ./xgit.sh
   ```
   or if installed globally:
   ```bash
   xgit
   ```

3. Follow the interactive menus to perform Git operations

## Project Structure

```
xgit/
├── xgit.sh          # Main executable script
├── lib/
│   ├── ui.sh        # User interface functions
│   ├── git.sh       # Git operation functions
│   ├── safety.sh    # Safety checks and confirmations
│   └── utils.sh     # Utility functions
├── config/
│   └── defaults.sh  # Configuration defaults
└── README.md        # This file
```

## Requirements

- Bash shell
- Git installed and configured
- Basic Unix tools (grep, sed, awk, etc.)

## Safety Features

- **Input Validation**: Validates branch names, commit messages, etc.
- **Confirmation Prompts**: Requires explicit confirmation for destructive operations
- **Conflict Handling**: Never auto-resolves conflicts; guides user through manual resolution
- **Status Display**: Shows current branch and repository status
- **Uncommitted Changes Warning**: Alerts when operations might affect uncommitted work

## Example Usage

```
=====================================
              XGit Tool
=====================================

Current Branch: main
Status: Clean

Choose an option:
1. Branch Management
2. Sync Operations
3. Commit System
4. Restore & Undo
5. Stash Menu
6. Exit

Enter your choice (1-6): 1

Branch Management:
1. Create new branch
2. Switch branch
3. Delete branch
4. List branches
5. Back to main menu

Enter your choice (1-5): 1
Enter new branch name: feature/new-feature
Enter base branch (default: main): main
Performing: Creating branch 'feature/new-feature' from 'main'
✓ Branch 'feature/new-feature' created and switched to.

Press Enter to continue...
```

## Contributing

This tool is designed to be modular and extensible. To add new features:

1. Add functions to appropriate lib files
2. Update menus in ui.sh
3. Add menu handling in xgit.sh
4. Update this README

## License

This project is open source. Feel free to use and modify as needed.

## Disclaimer

While XGit aims to provide a safe interface to Git, it is still possible to lose work if used incorrectly. Always backup important changes and understand what operations do before confirming them.