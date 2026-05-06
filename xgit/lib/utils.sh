
source "$(dirname "${BASH_SOURCE[0]}")/../config/defaults.sh"

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

        if [[ "$var_name" == *"branch"* ]] && ! is_valid_branch_name "$input"; then
            echo -e "${COLOR_RED}Invalid branch name. Branch names should not contain spaces or special characters.${COLOR_RESET}"
            continue
        fi

        eval "$var_name=\"$input\""
        break
    done
}

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

is_valid_branch_name() {
    local branch="$1"
    [[ "$branch" =~ ^[a-zA-Z0-9._/-]+$ ]] && [[ ! "$branch" =~ ^/ ]] && [[ ! "$branch" =~ /$ ]]
}

get_current_branch() {
    git branch --show-current 2>/dev/null
}

is_git_repo() {
    git rev-parse --git-dir >/dev/null 2>&1
}

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

list_branches() {
    git branch --list | sed 's/^[* ]*//'
}

has_conflicts() {
    git status --porcelain | grep -q '^UU'
}

get_conflicted_files() {
    git status --porcelain | grep '^UU' | awk '{print $2}'
}