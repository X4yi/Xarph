# XGit UI Functions
# Functions for displaying menus, prompts, and user interface elements

source "$(dirname "${BASH_SOURCE[0]}")/utils.sh"

# Function to display main menu
display_main_menu() {
    clear
    echo -e "${COLOR_BLUE}=====================================${COLOR_RESET}"
    echo -e "${COLOR_BLUE}              XGit Perro              ${COLOR_RESET}"
    echo -e "${COLOR_BLUE}=====================================${COLOR_RESET}"
    echo ""

    local current_branch
    current_branch=$(get_current_branch)
    local status_summary
    status_summary=$(get_git_status_summary)

    echo -e "${COLOR_GREEN}Current Branch: ${current_branch}${COLOR_RESET}"
    echo -e "${COLOR_GREEN}Status: ${status_summary}${COLOR_RESET}"
    echo ""

    echo "Choose an option:"
    echo "1. manejame la rama"
    echo "2. Operaciones sync-sosas"
    echo "3. Commits"
    echo "4. Restore & Undo"
    echo "5. Stash Menu"
    echo "6. Exit"
    echo ""
}

# Function to display branch menu
display_branch_menu() {
    echo -e "${COLOR_MAGENTA}manejame la rama:${COLOR_RESET}"
    echo "1. Crea 1 rama"
    echo "2. Cambia de rama"
    echo "3. Borra una rama"
    echo "4. Lista de ramas"
    echo "5. Gogogo menu"
    echo ""
}

# Function to display sync menu
display_sync_menu() {
    echo -e "${COLOR_MAGENTA}Operaciones sync-sosas:${COLOR_RESET}"
    echo "1. Pull"
    echo "2. Push"
    echo "3. Fetch"
    echo "4. Gogogo menu"
    echo ""
}

# Function to display commit menu
display_commit_menu() {
    echo -e "${COLOR_MAGENTA}Commits:${COLOR_RESET}"
    echo "1. Stage y commit"
    echo "2. Gogogo menu"
    echo ""
}

# Function to display restore menu
display_restore_menu() {
    echo -e "${COLOR_MAGENTA}Restaura tus mmadas:${COLOR_RESET}"
    echo "1. Reset Commit"
    echo "2. Discard changes"
    echo "3. gogogo menu"
    echo ""
}

# Function to display stash menu
display_stash_menu() {
    echo -e "${COLOR_MAGENTA}Stash:${COLOR_RESET}"
    echo "1. Cambios"
    echo "2. Aplicame el stash"
    echo "3. List stashes"
    echo "4. Drop stash"
    echo "5. Gogogo menu"
    echo ""
}

# Function to show success message
show_success() {
    echo -e "${COLOR_GREEN}✓ $1${COLOR_RESET}"
}

# Function to show error message
show_error() {
    echo -e "${COLOR_RED}✗ $1${COLOR_RESET}"
}

# Function to show warning
show_warning() {
    echo -e "${COLOR_YELLOW}⚠ $1${COLOR_RESET}"
}

# Function to pause and wait for user
pause() {
    echo ""
    echo -e "${COLOR_CYAN}Press Enter to continue...${COLOR_RESET}"
    read -r
}