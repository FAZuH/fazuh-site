#!/bin/bash

# Development helper script
# Usage: ./dev.sh [command1] [command2] ...
#   commands: format | lint | test | build | dev [port] | docs | mail | mail-dump | mail-send | mail-clear | mail-stop | all | help
#   Multiple commands can be specified and will execute left to right

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Constants
readonly MAIL_CONTAINER="fazuh-maildev"
readonly MAIL_IMAGE="axllent/mailpit:latest"

# Function to print colored output
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Help function
show_help() {
    cat << EOF
Development Helper Script

Usage: ./dev.sh [command1] [command2] ...

Commands:
  format    - Format code with "cargo +nightly fmt --all"
  lint      - Run linter with "cargo clippy --all-targets --all-features --no-deps --fix --allow-dirty"
  test      - Run tests with "cargo test --all-features --no-fail-fast"
  build     - Build release with "tailwindcss + dx build --release"
  dev [port] - Run "dx serve --port <port>" (default: 8080)
  docs      - Compile Mermaid diagrams to images
  mail      - Start Mailpit dev mail server (SMTP + IMAP + Web UI)
  mail-dump - Dump all captured messages from Mailpit
  mail-send - Send a test email via Mailpit
  mail-clear - Delete all messages from Mailpit
  mail-stop - Stop and remove the Mailpit container
  all       - Run format, lint, test, and build in sequence
  help      - Show this help message

Multiple commands can be specified and will execute sequentially from left to right.

Examples:
  ./dev.sh format                  # Format code
  ./dev.sh lint                    # Run linter
  ./dev.sh test                    # Run tests
  ./dev.sh build                   # Build release
  ./dev.sh dev                     # Dev server on port 8080
  ./dev.sh dev 8081                # Dev server on port 8081
  ./dev.sh mail                    # Start dev mail server
  ./dev.sh mail mail-dump          # Start server then dump messages
  ./dev.sh format lint             # Format then lint
  ./dev.sh all                     # Run format, lint, test, and build

EOF
}

# Command implementations
cmd_format() {
    print_info "Formatting code..."
    cargo +nightly fmt --all
    print_success "Formatting completed"
}

cmd_lint() {
    print_info "Linting code..."
    cargo clippy --all-targets --all-features --no-deps --fix --allow-dirty
    print_success "Linting completed"
}

cmd_test() {
    print_info "Running tests..."
    cargo test --all-features --no-fail-fast
    print_success "Tests completed"
}

cmd_build() {
    print_info "Building release..."
    tailwindcss -i input.css -o assets/tailwind.css
    dx build --release
    print_success "Release build completed"
}

cmd_docs() {
    print_info "Compiling Mermaid diagrams..."

    # Check if mmdc (Mermaid CLI) is installed
    if ! command -v mmdc &> /dev/null; then
        print_warning "Mermaid CLI not found. Installing..."
        npm install -g @mermaid-js/mermaid-cli
    fi

    # Create output directory
    mkdir -p docs/diagrams

    # Compile each .mmd file to PNG
    print_info "Processing .mmd diagram files..."

    for file in docs/diagrams/*.mmd; do
        if [ -f "$file" ]; then
            filename=$(basename "$file" .mmd)
            print_info "Compiling $filename.mmd..."
            mmdc -i "$file" -o "docs/diagrams/${filename}.png" -b transparent -s 4 --width 3840 --height 2160
        fi
    done

    print_success "Mermaid diagrams compiled to docs/diagrams/"
}

cmd_dev() {
    local port="${1:-8080}"
    print_info "Starting dev server on port ${port}..."
    set -em
    tailwindcss -i input.css -o assets/tailwind.css --watch &>/dev/null &
    TAILWIND_PID=$!
    trap "kill $TAILWIND_PID 2>/dev/null" EXIT
    dx serve --port "${port}"
    print_success "Dev server stopped"
}

cmd_mail() {
    print_info "Starting Mailpit dev mail server..."

    if ! command -v docker &>/dev/null; then
        print_error "Docker is required. Install it first."
        exit 1
    fi

    if ! docker info &>/dev/null; then
        print_error "Docker daemon is not running. Start it first."
        exit 1
    fi

    if docker ps -a --format '{{.Names}}' | grep -q "^${MAIL_CONTAINER}$"; then
        print_warning "Removing existing container..."
        docker rm -f "${MAIL_CONTAINER}" >/dev/null
    fi

    if ! docker image inspect "${MAIL_IMAGE}" &>/dev/null; then
        print_info "Pulling ${MAIL_IMAGE}..."
        docker pull -q "${MAIL_IMAGE}"
    fi

    docker run -d \
        --name "${MAIL_CONTAINER}" \
        -p 1025:1025 \
        -p 1143:1143 \
        -p 8025:8025 \
        "${MAIL_IMAGE}" >/dev/null

    for i in $(seq 1 10); do
        if curl -sf http://localhost:8025 >/dev/null 2>&1; then
            break
        fi
        sleep 1
    done

    echo ""
    echo "  ┌─ Mailpit ready ─────────────────────────────────────┐"
    echo "  │  SMTP:   localhost:1025  (no auth, no TLS)          │"
    echo "  │  IMAP:   localhost:1143  (no auth, no TLS)          │"
    echo "  │  Web UI: http://localhost:8025                       │"
    echo "  │                                                     │"
    echo "  │  Send test:  ./dev.sh mail-send                     │"
    echo "  │  Dump msgs:  ./dev.sh mail-dump                     │"
    echo "  │  Clear all:  ./dev.sh mail-clear                    │"
    echo "  │  Stop:       ./dev.sh mail-stop                     │"
    echo "  └─────────────────────────────────────────────────────┘"
    echo ""
}

cmd_mail_dump() {
    if ! curl -sf http://localhost:8025 >/dev/null 2>&1; then
        print_error "Mailpit not running. Start with: ./dev.sh mail"
        exit 1
    fi

    local json
    json=$(curl -s http://localhost:8025/api/v1/messages)

    python3 -c "
import json, sys, urllib.request

data = json.load(sys.stdin)
msgs = data.get('messages', [])
if not msgs:
    print('No messages.')
    sys.exit(0)
print(f'{len(msgs)} message(s):')
for m in msgs:
    sender = m.get('From', {})
    from_str = sender.get('Address', '?')
    if sender.get('Name'):
        from_str = f'{sender[\"Name\"]} <{from_str}>'
    to_list = m.get('To', [])
    to_str = ', '.join(t.get('Address', '?') for t in to_list)
    print('=' * 60)
    print(f'  ID:      {m[\"ID\"]}')
    print(f'  From:    {from_str}')
    print(f'  To:      {to_str}')
    print(f'  Subject: {m.get(\"Subject\", \"(no subject)\")}')
    print(f'  Date:    {m.get(\"Created\", \"\")}')
    print(f'  Size:    {m.get(\"Size\", 0)} bytes')
    print()
    # Fetch full message details for the text body (preserves newlines)
    detail_url = f'http://localhost:8025/api/v1/message/{m[\"ID\"]}'
    try:
        req = urllib.request.Request(detail_url)
        with urllib.request.urlopen(req) as resp:
            detail = json.loads(resp.read())
        text = detail.get('Text', '') or ''
        if text.strip():
            for line in text.splitlines():
                print(f'  {line}')
        else:
            print('  (no text body)')
    except Exception:
        print('  (could not fetch body)')
    print()
" <<< "$json"
}

cmd_mail_send() {
    if ! curl -sf http://localhost:8025 >/dev/null 2>&1; then
        print_error "Mailpit not running. Start with: ./dev.sh mail"
        exit 1
    fi

    python3 -c "
import smtplib
from email.message import EmailMessage

msg = EmailMessage()
msg['From'] = 'Dev Test <dev@localhost>'
msg['To'] = 'dev@localhost'
msg['Subject'] = 'Test email from dev.sh'
msg.set_content('Test email sent via local Mailpit dev server.\n\nSMTP: localhost:1025\nIMAP: localhost:1143\nWeb:  http://localhost:8025')

with smtplib.SMTP('127.0.0.1', 1025) as s:
    s.send_message(msg)
"

    print_success "Test email sent. View at http://localhost:8025"
}

cmd_mail_clear() {
    if ! curl -sf http://localhost:8025 >/dev/null 2>&1; then
        print_error "Mailpit not running."
        exit 1
    fi
    curl -s -X DELETE http://localhost:8025/api/v1/messages >/dev/null
    print_success "All messages cleared."
}

cmd_mail_stop() {
    if docker ps -a --format '{{.Names}}' | grep -q "^${MAIL_CONTAINER}$"; then
        docker stop "${MAIL_CONTAINER}" >/dev/null
        docker rm "${MAIL_CONTAINER}" >/dev/null
        print_success "Mailpit stopped and removed."
    else
        print_info "Mailpit not running."
    fi
}

cmd_all() {
    print_info "Running all tasks..."
    cmd_format
    cmd_lint
    cmd_test
    cmd_build
    print_success "All tasks completed"
}

# Execute a single command
execute_command() {
    local command="$1"

    case "$command" in
        format)
            cmd_format
            ;;
        lint)
            cmd_lint
            ;;
        test)
            cmd_test
            ;;
        build)
            cmd_build
            ;;
        docs)
            cmd_docs
            ;;
        dev)
            cmd_dev
            ;;
        mail)
            cmd_mail
            ;;
        mail-dump)
            cmd_mail_dump
            ;;
        mail-send)
            cmd_mail_send
            ;;
        mail-clear)
            cmd_mail_clear
            ;;
        mail-stop)
            cmd_mail_stop
            ;;
        all)
            cmd_all
            ;;
        help)
            show_help
            ;;
        *)
            print_error "Unknown command: $command"
            show_help
            exit 1
            ;;
    esac
}

# Main execution
if [ $# -eq 0 ]; then
    show_help
    exit 0
fi

while [ $# -gt 0 ]; do
    case "$1" in
        dev)
            shift
            if [ $# -gt 0 ] && [[ "$1" =~ ^[0-9]+$ ]]; then
                cmd_dev "$1"
                shift
            else
                cmd_dev
            fi
            ;;
        *)
            execute_command "$1"
            shift
            ;;
    esac
done
