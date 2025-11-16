#!/bin/bash

# Script to bump version in Cargo.toml
# Usage: ./scripts/bump-version.sh [major|minor|patch|VERSION]
# Examples:
#   ./scripts/bump-version.sh patch    # 0.1.0 -> 0.1.1
#   ./scripts/bump-version.sh minor    # 0.1.0 -> 0.2.0
#   ./scripts/bump-version.sh major    # 0.1.0 -> 1.0.0
#   ./scripts/bump-version.sh 0.2.0    # Set to specific version

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CARGO_TOML="$REPO_ROOT/Cargo.toml"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep "^version = " "$CARGO_TOML" | sed 's/version = "\(.*\)"/\1/')

if [ -z "$CURRENT_VERSION" ]; then
    echo "Error: Could not find version in Cargo.toml"
    exit 1
fi

echo "Current version: $CURRENT_VERSION"

# Determine new version
if [ -z "$1" ]; then
    echo "Usage: $0 [major|minor|patch|VERSION]"
    echo "Current version: $CURRENT_VERSION"
    exit 1
fi

if [[ "$1" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-.*)?$ ]]; then
    # Specific version provided
    NEW_VERSION="$1"
else
    # Parse current version
    IFS='.' read -ra VERSION_PARTS <<< "$CURRENT_VERSION"
    MAJOR="${VERSION_PARTS[0]}"
    MINOR="${VERSION_PARTS[1]}"
    PATCH="${VERSION_PARTS[2]%%-*}"  # Remove any pre-release suffix
    
    case "$1" in
        major)
            MAJOR=$((MAJOR + 1))
            MINOR=0
            PATCH=0
            ;;
        minor)
            MINOR=$((MINOR + 1))
            PATCH=0
            ;;
        patch)
            PATCH=$((PATCH + 1))
            ;;
        *)
            echo "Error: Invalid version bump type: $1"
            echo "Use: major, minor, patch, or a specific version (e.g., 0.2.0)"
            exit 1
            ;;
    esac
    
    NEW_VERSION="$MAJOR.$MINOR.$PATCH"
fi

# Update Cargo.toml
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    sed -i '' "s/^version = \".*\"/version = \"$NEW_VERSION\"/" "$CARGO_TOML"
else
    # Linux
    sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" "$CARGO_TOML"
fi

# Verify the change
UPDATED_VERSION=$(grep "^version = " "$CARGO_TOML" | sed 's/version = "\(.*\)"/\1/')

if [ "$UPDATED_VERSION" != "$NEW_VERSION" ]; then
    echo "Error: Failed to update version"
    exit 1
fi

echo -e "${GREEN}✓${NC} Updated version: $CURRENT_VERSION -> $NEW_VERSION"
echo ""
echo "Next steps:"
echo "  1. Review the changes: git diff Cargo.toml"
echo "  2. Commit: git commit -am \"Bump version to $NEW_VERSION\""
echo "  3. Tag: git tag -a v$NEW_VERSION -m \"Release v$NEW_VERSION\""
echo "  4. Push: git push && git push --tags"
