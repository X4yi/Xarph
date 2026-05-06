

set -e  # Exit on error

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

source "$SCRIPT_DIR/lib/utils.sh"
source "$SCRIPT_DIR/lib/ui.sh"
source "$SCRIPT_DIR/lib/safety.sh"
source "$SCRIPT_DIR/lib/git.sh"

main() {
    if ! is_git_repo; then
        show_error "Not in a Git repository. Please run this tool from inside a Git project."
        exit 1
    fi

    while true; do
        display_main_menu
        read_input "Enter your choice (1-6)" choice

        case "$choice" in
            1) branch_management ;;
            2) sync_operations ;;
            3) commit_system ;;
            4) restore_undo ;;
            5) stash_menu ;;
            6) 
                echo -e "${COLOR_GREEN}Goodbye!${COLOR_RESET}"
                exit 0
                ;;
            *) 
                show_error "Invalid choice. Please select 1-6."
                pause
                ;;
        esac
    done
}

branch_management() {
    while true; do
        display_main_menu
        display_branch_menu
        read_input "Enter your choice (1-5)" choice

        case "$choice" in
            1) create_branch ;;
            2) switch_branch ;;
            3) delete_branch ;;
            4) list_branches_menu ;;
            5) break ;;
            *) 
                show_error "Invalid choice."
                ;;
        esac
        pause
    done
}

sync_operations() {
    while true; do
        display_main_menu
        display_sync_menu
        read_input "Enter your choice (1-4)" choice

        case "$choice" in
            1) pull_changes ;;
            2) push_changes ;;
            3) fetch_updates ;;
            4) break ;;
            *) 
                show_error "Invalid choice."
                ;;
        esac
        pause
    done
}

commit_system() {
    while true; do
        display_main_menu
        display_commit_menu
        read_input "Enter your choice (1-2)" choice

        case "$choice" in
            1) stage_and_commit ;;
            2) break ;;
            *) 
                show_error "Invalid choice."
                ;;
        esac
        pause
    done
}

restore_undo() {
    while true; do
        display_main_menu
        display_restore_menu
        read_input "Enter your choice (1-3)" choice

        case "$choice" in
            1) reset_commits ;;
            2) discard_changes ;;
            3) break ;;
            *) 
                show_error "Invalid choice."
                ;;
        esac
        pause
    done
}

stash_menu() {
    while true; do
        display_main_menu
        display_stash_menu
        read_input "Enter your choice (1-5)" choice

        case "$choice" in
            1) stash_changes ;;
            2) apply_stash ;;
            3) list_stashes ;;
            4) drop_stash ;;
            5) break ;;
            *) 
                show_error "Invalid choice."
                ;;
        esac
        pause
    done
}

main "$@"