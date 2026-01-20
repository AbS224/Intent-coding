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
echo "  Windows path: $(wslpath -w $(pwd) 2>/dev/null || echo 'N/A')"
echo "  Linux path: $(wslpath -u 'L:\FinalTry' 2>/dev/null || echo 'N/A')"

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
        echo "     Fix: Run 'dos2unix *.sh' or change to LF in VS Code"
    else
        echo "  ✅ Line endings OK"
    fi
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

# Check for file locks
if lsof /mnt/l/FinalTry 2>/dev/null | grep -q .; then
    echo "  ⚠️  Files may be locked by Windows processes"
    echo "     Fix: Close VS Code and run 'wsl --shutdown'"
else
    echo "  ✅ No file locks detected"
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