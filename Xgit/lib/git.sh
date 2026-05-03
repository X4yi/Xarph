# XGit Git Operations
# Functions for Git operations, with interactive input

source "$(dirname "${BASH_SOURCE[0]}")/utils.sh"
source "$(dirname "${BASH_SOURCE[0]}")/safety.sh"
source "$(dirname "${BASH_SOURCE[0]}")/ui.sh"

# Branch Management Functions

# Create new branch
create_branch() {
    local branch_name base_branch

    read_input "Enter new branch name" branch_name
    read_input "Enter base branch" base_branch "$(get_current_branch)"

    if branch_exists "$branch_name"; then
        show_error "Branch '$branch_name' already exists."
        return 1
    fi

    check_safety "Creating branch '$branch_name' from '$base_branch'" "low"

    if git checkout -b "$branch_name" "$base_branch"; then
        show_success "Branch '$branch_name' created and switched to."
    else
        show_error "Failed to create branch."
    fi
}

# Switch branch
switch_branch() {
    local branches branch_choice branch_name

    branches=$(list_branches)
    echo "Available branches:"
    echo "$branches" | nl
    echo ""

    read_input "Enter branch number or name" branch_choice

    if [[ "$branch_choice" =~ ^[0-9]+$ ]]; then
        branch_name=$(echo "$branches" | sed -n "${branch_choice}p")
    else
        branch_name="$branch_choice"
    fi

    if [ -z "$branch_name" ]; then
        show_error "Invalid branch selection."
        return 1
    fi

    if ! branch_exists "$branch_name"; then
        show_error "Branch '$branch_name' does not exist."
        return 1
    fi

    if ! warn_uncommitted_changes; then
        return 1
    fi

    if git checkout "$branch_name"; then
        show_success "Switched to branch '$branch_name'."
    else
        show_error "Failed to switch branch."
    fi
}

# Delete branch
delete_branch() {
    local branch_name

    read_input "Enter branch name to delete" branch_name

    if ! branch_exists "$branch_name"; then
        show_error "Branch '$branch_name' does not exist."
        return 1
    fi

    if [ "$branch_name" = "$(get_current_branch)" ]; then
        show_error "Cannot delete the current branch."
        return 1
    fi

    confirm_destructive "Delete branch '$branch_name'"

    if git branch -D "$branch_name"; then
        show_success "Branch '$branch_name' deleted."
    else
        show_error "Failed to delete branch."
    fi
}

# List branches
list_branches_menu() {
    echo "Branches:"
    git branch -v
    pause
}

# Sync Operations

# Pull changes
pull_changes() {
    local strategy

    echo "Pull strategies:"
    echo "1. Merge"
    echo "2. Rebase"
    echo ""

    read_input "Choose pull strategy (1 or 2)" strategy

    case "$strategy" in
        1) strategy="merge" ;;
        2) strategy="rebase" ;;
        *) show_error "Invalid choice."; return 1 ;;
    esac

    if ! warn_uncommitted_changes; then
        return 1
    fi

    check_safety "Pulling changes with $strategy strategy" "medium"

    if git pull --$strategy; then
        show_success "Changes pulled successfully."
        handle_conflicts
    else
        show_error "Failed to pull changes."
        handle_conflicts
    fi
}

# Push changes
push_changes() {
    local push_type branch

    branch=$(get_current_branch)
    echo -e "${COLOR_CYAN}Current branch: $branch${COLOR_RESET}"

    read_input "Confirm branch to push" push_branch "$branch"

    echo "Push types:"
    echo "1. Normal push"
    echo "2. Force push"
    echo ""

    read_input "Choose push type (1 or 2)" push_choice

    case "$push_choice" in
        1) push_type="" ;;
        2) push_type="--force" ;;
        *) show_error "Invalid choice."; return 1 ;;
    esac

    if [ -n "$push_type" ]; then
        confirm_destructive "Force push to '$push_branch'"
    fi

    if git push $push_type origin "$push_branch"; then
        show_success "Changes pushed successfully."
    else
        show_error "Failed to push changes."
    fi
}

# Fetch updates
fetch_updates() {
    if git fetch --all; then
        show_success "Updates fetched."
        echo "Fetched branches:"
        git branch -r
    else
        show_error "Failed to fetch updates."
    fi
    pause
}

# Commit System

# Stage and commit
stage_and_commit() {
    local stage_all commit_message

    echo -e "${COLOR_CYAN}Stage all changes? (y/n):${COLOR_RESET}"
    read -r stage_all

    if [[ "$stage_all" =~ ^[Yy]$ ]]; then
        git add .
        show_success "All changes staged."
    else
        echo "Manual staging not implemented yet. Staging all for now."
        git add .
    fi

    read_input "Enter commit message" commit_message

    if [ -z "$commit_message" ]; then
        show_error "Commit message cannot be empty."
        return 1
    fi

    echo -e "${COLOR_CYAN}Commit with message: '$commit_message'${COLOR_RESET}"
    confirm_action "Commit changes" "COMMIT"

    if git commit -m "$commit_message"; then
        show_success "Changes committed."
    else
        show_error "Failed to commit."
    fi
}

# Restore & Undo

# Reset commits
reset_commits() {
    local reset_type

    echo "Reset types:"
    echo "1. Soft reset (keep changes staged)"
    echo "2. Mixed reset (keep changes unstaged)"
    echo "3. Hard reset (discard all changes)"
    echo ""

    read_input "Choose reset type (1-3)" reset_choice

    case "$reset_choice" in
        1) reset_type="--soft" ;;
        2) reset_type="--mixed" ;;
        3) reset_type="--hard" ;;
        *) show_error "Invalid choice."; return 1 ;;
    esac

    if [ "$reset_type" = "--hard" ]; then
        confirm_destructive "Hard reset (this will discard all uncommitted changes)"
    else
        check_safety "Reset commits with $reset_type" "medium"
    fi

    echo -e "${COLOR_CYAN}Reset to HEAD~1? (y/n):${COLOR_RESET}"
    read -r to_head
    if [[ "$to_head" =~ ^[Yy]$ ]]; then
        git reset $reset_type HEAD~1
    else
        echo "Reset cancelled."
    fi
}

# Discard changes
discard_changes() {
    local discard_type file

    echo "Discard options:"
    echo "1. Specific file"
    echo "2. All changes"
    echo ""

    read_input "Choose option (1 or 2)" discard_choice

    case "$discard_choice" in
        1)
            read_input "Enter file path" file
            if [ -f "$file" ]; then
                confirm_destructive "Discard changes to '$file'"
                git checkout -- "$file"
                show_success "Changes to '$file' discarded."
            else
                show_error "File not found."
            fi
            ;;
        2)
            confirm_destructive "Discard all uncommitted changes"
            git reset --hard HEAD
            git clean -fd
            show_success "All changes discarded."
            ;;
        *)
            show_error "Invalid choice."
            ;;
    esac
}

# Conflict handling
handle_conflicts() {
    if ! has_conflicts; then
        return 0
    fi

    show_warning "Merge conflicts detected!"
    local conflicted_files
    conflicted_files=$(get_conflicted_files)

    echo "Conflicted files:"
    echo "$conflicted_files"
    echo ""

    for file in $conflicted_files; do
        echo -e "${COLOR_YELLOW}Resolving conflict in: $file${COLOR_RESET}"
        echo "Options:"
        echo "1. Keep current version"
        echo "2. Accept incoming version"
        echo "3. Open in editor"
        echo "4. Skip for now"
        echo ""

        read_input "Choose option (1-4)" choice

        case "$choice" in
            1) git checkout --ours "$file" ;;
            2) git checkout --theirs "$file" ;;
            3) ${EDITOR:-nano} "$file" ;;
            4) continue ;;
            *) show_error "Invalid choice."; continue ;;
        esac

        git add "$file"
        show_success "Resolved conflict in $file"
    done

    echo -e "${COLOR_CYAN}Continue merge? (y/n):${COLOR_RESET}"
    read -r continue_merge
    if [[ "$continue_merge" =~ ^[Yy]$ ]]; then
        git commit --no-edit
        show_success "Merge completed."
    else
        git merge --abort
        show_warning "Merge aborted."
    fi
}

# Stash Menu Functions

# Stash changes
stash_changes() {
    local message
    read_input "Enter stash message (optional)" message

    if [ -n "$message" ]; then
        git stash push -m "$message"
    else
        git stash push
    fi
    show_success "Changes stashed."
}

# Apply stash
apply_stash() {
    local stashes stash_choice

    stashes=$(git stash list)
    if [ -z "$stashes" ]; then
        show_warning "No stashes available."
        return
    fi

    echo "Available stashes:"
    echo "$stashes"
    echo ""

    read_input "Enter stash number (0 for latest)" stash_num "0"

    if git stash apply "stash@{$stash_num}"; then
        show_success "Stash applied."
    else
        show_error "Failed to apply stash."
    fi
}

# List stashes
list_stashes() {
    git stash list
    pause
}

# Drop stash
drop_stash() {
    local stashes stash_choice

    stashes=$(git stash list)
    if [ -z "$stashes" ]; then
        show_warning "No stashes available."
        return
    fi

    echo "Available stashes:"
    echo "$stashes"
    echo ""

    read_input "Enter stash number to drop" stash_num

    confirm_destructive "Drop stash@{$stash_num}"

    if git stash drop "stash@{$stash_num}"; then
        show_success "Stash dropped."
    else
        show_error "Failed to drop stash."
    fi
}