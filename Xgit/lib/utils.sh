# XGit Utilities
# Utility functions for input validation, prompts, etc.

source "$(dirname "${BASH_SOURCE[0]}")/../config/defaults.sh"

# Function to read user input with validation
# Usage: read_input "Prompt message" variable_name [default_value]
read_input() {
    local prompt="$1"
    local var_name="$2"
    local default="$3"

    while true; do
        if [ -n "$default" ]; then
            echo -e "${COLOR_CYAN}$prompt (default: $default):${COLOR_RESET}"
            read -r input
            if [ -z "$input" ]; then
                input="$default"
            fi
        else
            echo -e "${COLOR_CYAN}$prompt:${COLOR_RESET}"
            read -r input
        fi

        if [ -z "$input" ]; then
            echo -e "${COLOR_RED}Input cannot be empty. Please try again.${COLOR_RESET}"
            continue
        fi

        # Validate branch name if it's a branch
        if [[ "$var_name" == *"branch"* ]] && ! is_valid_branch_name "$input"; then
            echo -e "${COLOR_RED}Invalid branch name. Branch names should not contain spaces or special characters.${COLOR_RESET}"
            continue
        fi

        eval "$var_name=\"$input\""
        break
    done
}

# Function to confirm action
# Usage: confirm_action "Action description" [confirmation_word]
confirm_action() {
    local action="$1"
    local confirm_word="${2:-$CONFIRM_YES}"

    echo -e "${COLOR_YELLOW}WARNING: $action${COLOR_RESET}"
    echo -e "${COLOR_YELLOW}Type '$confirm_word' to confirm, or anything else to cancel:${COLOR_RESET}"
    read -r response
    if [ "$response" != "$confirm_word" ]; then
        echo -e "${COLOR_RED}Action cancelled.${COLOR_RESET}"
        return 1
    fi
    return 0
}

# Function to validate branch name
is_valid_branch_name() {
    local branch="$1"
    # Basic validation: no spaces, no leading/trailing slashes, etc.
    [[ "$branch" =~ ^[a-zA-Z0-9._/-]+$ ]] && [[ ! "$branch" =~ ^/ ]] && [[ ! "$branch" =~ /$ ]]
}

# Function to get current branch
get_current_branch() {
    git branch --show-current 2>/dev/null
}

# Function to check if in git repo
is_git_repo() {
    git rev-parse --git-dir >/dev/null 2>&1
}

# Function to get git status summary
get_git_status_summary() {
    local status
    status=$(git status --porcelain 2>/dev/null)
    if [ -z "$status" ]; then
        echo "Clean"
    else
        local staged=$(echo "$status" | grep '^[^?]' | wc -l)
        local unstaged=$(echo "$status" | grep '^.[^?]' | wc -l)
        local untracked=$(echo "$status" | grep '^\?\?' | wc -l)
        echo "Staged: $staged, Unstaged: $unstaged, Untracked: $untracked"
    fi
}

# Function to list branches
list_branches() {
    git branch --list | sed 's/^[* ]*//'
}

# Function to check for conflicts
has_conflicts() {
    git status --porcelain | grep -q '^UU'
}

# Function to get conflicted files
get_conflicted_files() {
    git status --porcelain | grep '^UU' | awk '{print $2}'
}