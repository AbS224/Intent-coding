#!/bin/bash
# Crucible Engine WSL Development Environment Setup

set -e

echo "🔥 Crucible Engine WSL Development Setup"
echo "========================================"

# Check if we're in WSL
if [[ ! -f /proc/version ]] || ! grep -qi microsoft /proc/version 2>/dev/null; then
    if [[ ! -f /proc/sys/fs/binfmt_misc/WSLInterop ]]; then
        echo "❌ This script must be run in WSL"
        exit 1
    fi
fi

PROJECT_ROOT="/mnt/l/FinalTry"
cd "$PROJECT_ROOT"

echo "📍 Working directory: $(pwd)"

# Update system packages
echo "📦 Updating system packages..."
sudo apt update -qq

# Install essential development tools
echo "🛠️ Installing development tools..."
sudo apt install -y \
    build-essential \
    curl \
    git \
    python3 \
    python3-pip \
    pkg-config \
    libssl-dev \
    tree \
    jq

# Install Rust if not present
if ! command -v rustc &> /dev/null; then
    echo "🦀 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
else
    echo "✅ Rust already installed: $(rustc --version)"
fi

# Install Node.js if not present
if ! command -v node &> /dev/null; then
    echo "📦 Installing Node.js..."
    curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
    sudo apt install -y nodejs
else
    echo "✅ Node.js already installed: $(node --version)"
fi

# Install Z3 theorem prover
if ! command -v z3 &> /dev/null; then
    echo "🧮 Installing Z3 SMT Solver..."
    sudo apt install -y z3
else
    echo "✅ Z3 already installed: $(z3 --version)"
fi

# Install Tree-sitter CLI
if ! command -v tree-sitter &> /dev/null; then
    echo "🌳 Installing Tree-sitter..."
    npm install -g tree-sitter-cli
else
    echo "✅ Tree-sitter already installed: $(tree-sitter --version)"
fi

# Install Python dependencies for MIL-SPEC tools
echo "🐍 Installing Python dependencies..."
pip3 install --user \
    pyyaml \
    jinja2 \
    markdown \
    cryptography

# Create WSL-specific development script
cat > wsl-dev.sh << 'EOF'
#!/bin/bash
# WSL Development Helper Script

PROJECT_ROOT="/mnt/l/FinalTry"
cd "$PROJECT_ROOT"

case "$1" in
    "build")
        echo "🔨 Building Crucible Engine..."
        cargo build
        ;;
    "test")
        echo "🧪 Running tests..."
        cargo test
        ;;
    "demo")
        echo "🚀 Running demo..."
        cargo run --bin crucible-demo
        ;;
    "milspec")
        echo "📋 Running MIL-SPEC generator..."
        python3 milspec_generator.py "${2:-check}"
        ;;
    "z3")
        echo "🧮 Testing Z3 integration..."
        z3 -version
        echo "(declare-const x Int)" | z3 -in
        ;;
    "tree-sitter")
        echo "🌳 Testing Tree-sitter..."
        tree-sitter --version
        ;;
    "status")
        echo "📊 Development Environment Status:"
        echo "  Rust: $(rustc --version 2>/dev/null || echo 'Not installed')"
        echo "  Node: $(node --version 2>/dev/null || echo 'Not installed')"
        echo "  Z3: $(z3 --version 2>/dev/null || echo 'Not installed')"
        echo "  Tree-sitter: $(tree-sitter --version 2>/dev/null || echo 'Not installed')"
        echo "  Python: $(python3 --version 2>/dev/null || echo 'Not installed')"
        ;;
    "clean")
        echo "🧹 Cleaning build artifacts..."
        cargo clean
        ;;
    *)
        echo "Usage: ./wsl-dev.sh [command]"
        echo ""
        echo "Commands:"
        echo "  build       - Build the project"
        echo "  test        - Run tests"
        echo "  demo        - Run the demo"
        echo "  milspec     - Run MIL-SPEC tools"
        echo "  z3          - Test Z3 integration"
        echo "  tree-sitter - Test Tree-sitter"
        echo "  status      - Show environment status"
        echo "  clean       - Clean build artifacts"
        ;;
esac
EOF

chmod +x wsl-dev.sh

# Test the environment
echo ""
echo "🧪 Testing development environment..."
echo "  Rust: $(rustc --version)"
echo "  Cargo: $(cargo --version)"
echo "  Node: $(node --version)"
echo "  Z3: $(z3 --version)"
echo "  Tree-sitter: $(tree-sitter --version)"
echo "  Python: $(python3 --version)"

# Test Rust build
echo ""
echo "🔨 Testing Rust build..."
if cargo check; then
    echo "✅ Rust build successful"
else
    echo "❌ Rust build failed"
fi

echo ""
echo "✅ WSL Development Environment Setup Complete!"
echo ""
echo "Usage:"
echo "  ./wsl-dev.sh status    - Check environment"
echo "  ./wsl-dev.sh build     - Build project"
echo "  ./wsl-dev.sh demo      - Run demo"
echo "  ./wsl-dev.sh milspec   - Generate MIL-SPEC docs"
echo ""
echo "🚀 Ready for development!"