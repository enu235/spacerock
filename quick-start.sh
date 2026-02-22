#!/bin/bash

# SpaceRock Quick Start Script
# This script will download, build, and run the SpaceRock game

set -e  # Exit on any error

echo "🚀 SpaceRock Quick Start Script 🌌"
echo "=================================="
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check for Rust installation
check_rust() {
    print_status "Checking for Rust installation..."
    if command_exists rustc && command_exists cargo; then
        local rust_version=$(rustc --version)
        print_success "Rust found: $rust_version"
        return 0
    else
        print_error "Rust is not installed!"
        echo
        echo "Please install Rust by running:"
        echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        echo
        echo "Or visit: https://rustup.rs/"
        echo
        echo "After installation, restart your terminal and run this script again."
        return 1
    fi
}

# Check for wasm-pack installation
check_wasm_pack() {
    print_status "Checking for wasm-pack installation..."
    if command_exists wasm-pack; then
        local wasm_version=$(wasm-pack --version)
        print_success "wasm-pack found: $wasm_version"
        return 0
    else
        print_warning "wasm-pack is not installed. Installing now..."
        if command_exists curl; then
            curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
            if [ $? -eq 0 ]; then
                print_success "wasm-pack installed successfully!"
                return 0
            else
                print_error "Failed to install wasm-pack automatically."
                echo "Please install manually: https://rustwasm.github.io/wasm-pack/installer/"
                return 1
            fi
        else
            print_error "curl is not available. Please install wasm-pack manually."
            echo "Visit: https://rustwasm.github.io/wasm-pack/installer/"
            return 1
        fi
    fi
}

# Check for git
check_git() {
    print_status "Checking for Git installation..."
    if command_exists git; then
        print_success "Git is available"
        return 0
    else
        print_error "Git is not installed!"
        echo "Please install Git first: https://git-scm.com/downloads"
        return 1
    fi
}

# Find available Python command
find_python() {
    if command_exists python3; then
        echo "python3"
    elif command_exists python; then
        # Check if it's Python 3
        if python -c "import sys; exit(0 if sys.version_info[0] == 3 else 1)" 2>/dev/null; then
            echo "python"
        else
            return 1
        fi
    else
        return 1
    fi
}

# Check for web server capability
check_web_server() {
    print_status "Checking for web server capability..." >&2

    # Priority: Python 3 (most common) > PHP (often pre-installed) > Node.js (optional)
    local python_cmd=$(find_python)
    if [ $? -eq 0 ]; then
        print_success "Python 3 found: will use built-in HTTP server" >&2
        echo "$python_cmd"
        return 0
    elif command_exists php; then
        print_success "PHP found: will use built-in server" >&2
        echo "php"
        return 0
    elif command_exists npx; then
        print_success "Node.js/npx found: will use http-server (optional)" >&2
        echo "npx"
        return 0
    else
        print_warning "No suitable web server found." >&2
        print_warning "Please install Python 3 or PHP to run the local server." >&2
        print_warning "Node.js is optional but can also provide a web server." >&2
        echo "none"
        return 1
    fi
}

# Main execution
main() {
    echo "Performing dependency checks..."
    echo

    # Check all dependencies
    local deps_ok=true
    
    if ! check_rust; then
        deps_ok=false
    fi
    
    if ! check_wasm_pack; then
        deps_ok=false
    fi
    
    if ! check_git; then
        deps_ok=false
    fi
    
    local server_cmd=$(check_web_server)
    if [ "$server_cmd" = "none" ]; then
        deps_ok=false
    fi
    
    if [ "$deps_ok" = false ]; then
        print_error "Some dependencies are missing. Please install them and try again."
        exit 1
    fi
    
    echo
    print_success "All dependencies are available!"
    echo
    
    # Clone or update repository
    local repo_dir="spacerock"
    if [ -d "$repo_dir" ]; then
        print_status "Repository directory exists. Updating..."
        cd "$repo_dir"
        git pull origin main
    else
        print_status "Cloning SpaceRock repository..."
        git clone https://github.com/enu235/spacerock.git "$repo_dir"
        cd "$repo_dir"
    fi
    
    print_success "Repository ready!"
    echo
    
    # Build the project
    print_status "Building WebAssembly package..."
    wasm-pack build --target web
    
    if [ $? -eq 0 ]; then
        print_success "Build completed successfully!"
    else
        print_error "Build failed!"
        exit 1
    fi
    
    echo
    print_success "🎮 SpaceRock is ready to play!"
    echo
    
    # Start web server
    local port=8080
    print_status "Starting web server on port $port..."
    echo
    print_warning "The game will open in your browser shortly..."
    print_warning "Press Ctrl+C to stop the server when you're done playing."
    echo
    
    # Open browser after a short delay
    (sleep 3 && print_status "Opening browser..." && (command_exists xdg-open && xdg-open "http://localhost:$port") || (command_exists open && open "http://localhost:$port") || echo "Please open http://localhost:$port in your browser") &
    
    # Start appropriate server
    case "$server_cmd" in
        python3|python)
            $server_cmd -m http.server $port
            ;;
        npx)
            npx http-server -p $port -o
            ;;
        php)
            php -S localhost:$port
            ;;
        *)
            print_error "No web server available"
            exit 1
            ;;
    esac
}

# Run main function
main "$@" 