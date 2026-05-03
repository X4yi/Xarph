# XGit Safety Functions
# Functions for safety checks and confirmations before risky operations

source "$(dirname "${BASH_SOURCE[0]}")/utils.sh"

# Function to check if operation is safe
# Usage: check_safety "operation_description" [risk_level]
check_safety() {
    local operation="$1"
    local risk_level="${2:-medium}"

    case "$risk_level" in
        "high")
            show_warning "This is a HIGH RISK operation: $operation"
            confirm_action "$operation" "$CONFIRM_YES"
            ;;
        "medium")
            show_warning "This operation may be risky: $operation"
            confirm_action "$operation" "$CONFIRM_YES"
            ;;
        "low")
            echo -e "${COLOR_CYAN}Performing: $operation${COLOR_RESET}"
            ;;
    esac
}

# Function to confirm destructive actions
confirm_destructive() {
    local action="$1"
    show_error "DESTRUCTIVE ACTION: $action"
    echo -e "${COLOR_RED}This action cannot be undone!${COLOR_RESET}"
    confirm_action "$action" "$CONFIRM_DELETE"
}

# Function to check if branch exists
branch_exists() {
    local branch="$1"
    git show-ref --verify --quiet "refs/heads/$branch"
}

# Function to check if remote branch exists
remote_branch_exists() {
    local branch="$1"
    git ls-remote --heads origin "$branch" >/dev/null 2>&1
}

# Function to check if there are uncommitted changes
has_uncommitted_changes() {
    ! git diff --quiet || ! git diff --staged --quiet
}

# Function to warn about uncommitted changes
warn_uncommitted_changes() {
    if has_uncommitted_changes; then
        show_warning "You have uncommitted changes. Consider committing or stashing them first."
        echo -e "${COLOR_CYAN}Continue anyway? (y/n):${COLOR_RESET}"
        read -r response
        if [[ ! "$response" =~ ^[Yy]$ ]]; then
            return 1
        fi
    fi
    return 0
}