#!/usr/bin/env bash
# Pre-commit hook for running code quality checks.
#
# Usage:
#   ./scripts/pre-commit.sh              Run full check
#   ./scripts/pre-commit.sh --install    Install as .git/hooks/pre-commit
#   ./scripts/pre-commit.sh --check      Dry run (same as default)
#   ./scripts/pre-commit.sh --help       Show this help message
#
# Checks performed:
#   1. cargo fmt --check --all         (code formatting)
#   2. cargo clippy --workspace -- -D warnings  (linting)
#
# The commit is blocked if any check fails.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

usage() {
    cat <<'EOF'
Usage: scripts/pre-commit.sh [OPTION]

Options:
  --check      Run fmt and clippy checks (default).
  --install    Install as .git/hooks/pre-commit.
  -h, --help   Show this help message.
EOF
}

ACTION="check"

while [ "$#" -gt 0 ]; do
    case "$1" in
        --check)
            ACTION="check"
            ;;
        --install)
            ACTION="install"
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "Unknown option: $1" >&2
            usage >&2
            exit 2
            ;;
    esac
    shift
done

if [ "$ACTION" = "install" ]; then
    if [ ! -d "${REPO_ROOT}/.git" ]; then
        echo "ERROR: .git directory not found. Run this from the repository root." >&2
        exit 1
    fi

    HOOK_PATH="${REPO_ROOT}/.git/hooks/pre-commit"

    cat > "${HOOK_PATH}" <<'HOOK'
#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

echo "=========================================="
echo "  Pre-commit: Running quality checks..."
echo "=========================================="
echo ""

# 1. Formatting check
echo "→ Checking code formatting (cargo fmt --check --all)..."
if ! cargo fmt --check --all; then
    echo ""
    echo "ERROR: Code formatting check failed." >&2
    echo "Run 'cargo fmt --all' to fix formatting issues." >&2
    exit 1
fi
echo "✓ Formatting check passed."
echo ""

# 2. Interface sync check (if available)
if [ -f "${REPO_ROOT}/scripts/sync-interfaces.sh" ]; then
    echo "→ Checking interface synchronization..."
    if ! "${REPO_ROOT}/scripts/sync-interfaces.sh" --check 2>/dev/null; then
        echo ""
        echo "ERROR: Interface synchronization check failed." >&2
        exit 1
    fi
    echo "✓ Interface sync check passed."
    echo ""
fi

# 3. Clippy check
echo "→ Running clippy (cargo clippy --workspace --all-targets -- -D warnings)..."
if ! cargo clippy --workspace --all-targets -- -D warnings; then
    echo ""
    echo "ERROR: Clippy check failed." >&2
    echo "Please fix all clippy warnings before committing." >&2
    exit 1
fi
echo "✓ Clippy check passed."
echo ""

echo "=========================================="
echo "  All pre-commit checks passed!"
echo "=========================================="
HOOK

    chmod +x "${HOOK_PATH}"
    echo "✓ Pre-commit hook installed at .git/hooks/pre-commit"
    echo "  It will run fmt and clippy checks before each commit."
    exit 0
fi

# Default: run checks
echo "=========================================="
echo "  Running code quality checks..."
echo "=========================================="
echo ""

# 1. Formatting
echo "→ Checking code formatting..."
if cargo fmt --check --all; then
    echo "✓ Formatting check passed."
else
    echo "✗ Formatting check failed. Run 'cargo fmt --all' to fix."
    exit 1
fi
echo ""

# 2. Clippy
echo "→ Running clippy..."
if cargo clippy --workspace --all-targets -- -D warnings; then
    echo "✓ Clippy check passed."
else
    echo "✗ Clippy check failed. Fix warnings before committing."
    exit 1
fi
echo ""

echo "=========================================="
echo "  All checks passed!"
echo "=========================================="