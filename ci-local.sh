#!/bin/bash
# Crucible Engine Local CI with Podman
set -e

echo "🛡️ Crucible Engine Local CI Pipeline (Podman)"
echo "=============================================="

# Build CI container if needed
if ! podman image exists crucible-ci:latest; then
    echo "📦 Building CI container..."
    podman build -f Dockerfile.ci -t crucible-ci:latest .
fi

# Run CI pipeline
echo "🚀 Running CI pipeline..."
podman run --rm \
    -v "$(pwd):/workspace:Z" \
    -w /workspace \
    crucible-ci:latest \
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
        
        echo '3. 📋 MIL-SPEC Compliance...' &&
        for doc in BUILD_CHECKLIST.md README.md LICENSE SECURITY.md CONTRIBUTING.md; do
            if [ ! -f \$doc ]; then
                echo \"❌ Missing: \$doc\" && exit 1
            fi
        done &&
        echo '✅ MIL-SPEC compliance verified' &&
        
        echo '4. 🐍 Python Quality...' &&
        if ls *.py 1> /dev/null 2>&1; then
            python3 -m py_compile *.py &&
            echo '✅ Python syntax passed'
        fi &&
        
        echo '🎉 All checks passed - Ready for push!'
    "

echo ""
echo "✅ Local CI pipeline completed successfully!"
echo "🔒 You can now proceed with MIL-SPEC review process"