#!/bin/bash
# Crucible Engine Bridge Troubleshooting Script

echo "🔧 Crucible Engine Bridge Diagnostics"
echo "====================================="

# Check WSL status
echo "📊 WSL Status:"
if command -v wsl.exe &> /dev/null; then
    wsl.exe --list --verbose
else
    echo "❌ WSL not available from this environment"
fi

echo ""
echo "📁 Path Information:"
echo "  Current directory: $(pwd)"
echo "  Windows path: $(wslpath -w "$(pwd)" 2>/dev/null || echo 'N/A')"
# Dynamic path detection instead of hardcoded paths
WIN_PROJECT_PATH="${WIN_PROJECT_PATH:-L:\\FinalTry}"
LINUX_PROJECT_PATH="${LINUX_PROJECT_PATH:-/mnt/l/FinalTry}"
echo "  Linux path: $(wslpath -u "$WIN_PROJECT_PATH" 2>/dev/null || echo 'N/A')"
echo "  Project paths: Windows=$WIN_PROJECT_PATH, Linux=$LINUX_PROJECT_PATH"

echo ""
echo "🛠️ Development Tools:"
echo "  Rust: $(rustc --version 2>/dev/null || echo 'Not installed')"
echo "  Cargo: $(cargo --version 2>/dev/null || echo 'Not installed')"
echo "  Python: $(python3 --version 2>/dev/null || echo 'Not installed')"
echo "  Git: $(git --version 2>/dev/null || echo 'Not installed')"

echo ""
echo "📦 Project Status:"
if [ -f "Cargo.toml" ]; then
    echo "  ✅ Cargo.toml found"
    echo "  📋 Workspace members:"
    grep -A 10 "members" Cargo.toml | grep -E '^\s*"' | sed 's/^/    /'
else
    echo "  ❌ Cargo.toml not found"
fi

echo ""
echo "🔍 Common Issues Check:"

# Check for CRLF line endings
if command -v file &> /dev/null; then
    if file setup-wsl.sh 2>/dev/null | grep -q "CRLF"; then
        echo "  ⚠️  CRLF line endings detected in shell scripts"
        if command -v dos2unix &> /dev/null; then
            echo "     Fix: Run 'dos2unix *.sh'"
        else
            echo "     Fix: Install dos2unix (sudo apt install dos2unix) or change to LF in VS Code"
        fi
    else
        echo "  ✅ Line endings OK"
    fi
else
    echo "  ⚠️  'file' command not available - cannot check line endings"
fi

# Check file permissions
if [ -f "setup-wsl.sh" ]; then
    if [ -x "setup-wsl.sh" ]; then
        echo "  ✅ Shell scripts are executable"
    else
        echo "  ⚠️  Shell scripts not executable"
        echo "     Fix: Run 'chmod +x *.sh'"
    fi
fi

# Check for file locks with availability check
if command -v lsof &> /dev/null; then
    if lsof "$LINUX_PROJECT_PATH" 2>/dev/null | grep -q .; then
        echo "  ⚠️  Files may be locked by Windows processes"
        echo "     Fix: Close VS Code and run 'wsl --shutdown'"
    else
        echo "  ✅ No file locks detected"
    fi
else
    echo "  ⚠️  lsof not available - cannot check file locks"
    echo "     Install: sudo apt install lsof"
fi

echo ""
echo "🚀 Quick Fixes:"
echo "  1. Reset WSL bridge: wsl --shutdown (from PowerShell)"
echo "  2. Fix line endings: dos2unix *.sh"
echo "  3. Fix permissions: chmod +x *.sh"
echo "  4. Reset VS Code server: rm -rf ~/.vscode-server"

echo ""
echo "🔧 Recommended Actions:"
if [ ! -x "setup-wsl.sh" ]; then
    echo "  chmod +x *.sh"
fi

if ! command -v rustc &> /dev/null; then
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
fi

if [ ! -f ".vscode/tasks.json" ]; then
    echo "  VS Code tasks.json missing - check .vscode directory"
fi

echo ""
echo "✅ Diagnostics complete!"