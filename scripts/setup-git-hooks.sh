#!/bin/bash

# Setup script to install git hooks
# This makes the pre-push hook available to all developers

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
HOOKS_DIR="$REPO_ROOT/.git/hooks"

echo "Setting up git hooks..."

# Create hooks directory if it doesn't exist
mkdir -p "$HOOKS_DIR"

# Copy pre-push hook
if [ -f "$SCRIPT_DIR/pre-push" ]; then
    cp "$SCRIPT_DIR/pre-push" "$HOOKS_DIR/pre-push"
    chmod +x "$HOOKS_DIR/pre-push"
    echo "✓ Installed pre-push hook"
else
    echo "✗ Error: pre-push hook not found in scripts directory"
    exit 1
fi

echo ""
echo "Git hooks installed successfully!"
echo "The pre-push hook will now run tests and checks before each push."
