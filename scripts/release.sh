#!/bin/bash
set -euo pipefail

# Release script for stonktop
# Usage: ./scripts/release.sh <version>
# Example: ./scripts/release.sh 0.2.0

VERSION="${1:-}"

if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 0.2.0"
    exit 1
fi

# Validate version format
if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$ ]]; then
    echo "Error: Invalid version format. Use semver (e.g., 0.2.0 or 0.2.0-beta.1)"
    exit 1
fi

TAG="v$VERSION"

echo "==> Preparing release $TAG"

# Check if we're on main branch
BRANCH=$(git branch --show-current)
if [ "$BRANCH" != "main" ] && [ "$BRANCH" != "master" ]; then
    echo "Warning: Not on main/master branch (currently on $BRANCH)"
    read -p "Continue anyway? [y/N] " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
    echo "Error: There are uncommitted changes. Please commit or stash them first."
    exit 1
fi

# Check if tag already exists
if git rev-parse "$TAG" >/dev/null 2>&1; then
    echo "Error: Tag $TAG already exists"
    exit 1
fi

# Update version in Cargo.toml
echo "==> Updating version in Cargo.toml to $VERSION"
sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml
rm -f Cargo.toml.bak

# Update Cargo.lock
echo "==> Updating Cargo.lock"
cargo check --quiet

# Run tests
echo "==> Running tests"
cargo test --quiet

# Run clippy
echo "==> Running clippy"
cargo clippy --quiet -- -D warnings

# Check formatting
echo "==> Checking formatting"
cargo fmt -- --check

# Verify crate can be packaged
echo "==> Verifying crate packaging"
cargo package --list

# Commit version bump
echo "==> Committing version bump"
git add Cargo.toml Cargo.lock
git commit -m "chore: bump version to $VERSION"

# Create tag
echo "==> Creating tag $TAG"
git tag -s "$TAG" -m "Release $TAG"

echo ""
echo "==> Release $TAG prepared successfully!"
echo ""
echo "Next steps:"
echo "  1. Review the commit: git show HEAD"
echo "  2. Review the tag: git show $TAG"
echo "  3. Push changes: git push origin main --follow-tags"
echo ""
echo "The GitHub Actions workflow will automatically:"
echo "  - Build release binaries for all platforms"
echo "  - Create a GitHub release with the binaries"
echo "  - Publish to crates.io (requires CARGO_REGISTRY_TOKEN secret)"
