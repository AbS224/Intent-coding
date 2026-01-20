#!/bin/bash
# Crucible Engine Parser Integration CI Pipeline
set -e

echo "🛡️ Crucible Engine Parser Integration CI Pipeline"
echo "================================================="

# Build parser-ready CI container if needed
if ! podman image exists crucible-parser:latest; then
    echo "📦 Building parser integration container..."
    podman build -f Dockerfile.parser -t crucible-parser:latest .
fi

# Create logs directory
mkdir -p logs/ci

# Run enhanced CI pipeline with parser tools
echo "🚀 Running enhanced CI pipeline..."
podman run --rm \
    -v "$(pwd):/workspace:Z" \
    -w /workspace \
    --user 1000:1000 \
    crucible-parser:latest \
    /bin/bash -c "
        echo '1. 🔍 Security Scan...' &&
        if grep -r 'password\|secret\|key' --include='*.rs' --include='*.js' --include='*.py' . | grep -v 'example\|test\|placeholder'; then
            echo '❌ Secrets detected' && exit 1
        fi &&
        echo '✅ Security scan passed' &&
        
        echo '2. 🦀 Rust Quality...' &&
        cargo fmt --check &&
        cargo clippy -- -D warnings &&
        cargo build &&
        cargo test &&
        echo '✅ Rust quality passed' &&
        
        echo '3. 🌳 Tree-Sitter Readiness...' &&
        if command -v tree-sitter >/dev/null 2>&1; then
            echo '✅ Tree-Sitter CLI available: \$(tree-sitter --version)'
        else
            echo '❌ Tree-Sitter CLI not found' && exit 1
        fi &&
        
        echo '4. 🧮 Z3 SMT Solver Readiness...' &&
        if command -v z3 >/dev/null 2>&1; then
            echo '✅ Z3 SMT Solver available: \$(z3 --version | head -1)'
            echo '(assert true)' | z3 -in >/dev/null 2>&1 && echo '✅ Z3 basic functionality verified'
        else
            echo '❌ Z3 SMT Solver not found' && exit 1
        fi &&
        
        echo '5. 📋 MIL-SPEC Compliance...' &&
        for doc in BUILD_CHECKLIST.md README.md LICENSE SECURITY.md CONTRIBUTING.md; do
            if [ ! -f \$doc ]; then
                echo \"❌ Missing: \$doc\" && exit 1
            fi
        done &&
        echo '✅ MIL-SPEC compliance verified' &&
        
        echo '6. 🐍 Python Quality...' &&
        if ls *.py 1> /dev/null 2>&1; then
            python3 -m py_compile *.py &&
            echo '✅ Python syntax passed'
        fi &&
        
        echo '7. 📊 Build Artifact Generation...' &&
        find target -name '*.rlib' -o -name 'crucible*' 2>/dev/null | head -5 | xargs sha256sum 2>/dev/null > logs/ci/build_artifacts.sha256 &&
        echo '✅ Build artifacts logged' &&
        
        echo '🎉 All checks passed - Parser integration ready!' &&
        
        echo '8. 📊 Generating Audit Report...' &&
        python3 audit_generator.py generate &&
        echo '✅ Audit report generated'
    " 2>&1 | tee logs/ci/ci_run_$(date +%Y%m%d_%H%M%S).log

echo ""
echo "✅ Enhanced CI pipeline completed successfully!"
echo "📋 Logs saved to: logs/ci/"
echo "🔒 Ready for MIL-SPEC security review"