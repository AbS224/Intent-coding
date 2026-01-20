#!/bin/bash
# Crucible Engine MIL-SPEC Push Gatekeeper
# "Trust, but Verify" - Token-based push authorization

set -e

TOKEN_FILE=".push_token"
LOG_DIR="logs/ci"
AUDIT_LOG="$LOG_DIR/push_audit.log"

# Ensure log directory exists
mkdir -p "$LOG_DIR"

# Function to log audit events
audit_log() {
    echo "[$(date -Iseconds)] $1" >> "$AUDIT_LOG"
}

echo "🛡️ MIL-SPEC Push Gatekeeper"
echo "=========================="

# Check if token exists
if [ ! -f "$TOKEN_FILE" ]; then
    echo "🛑 PUSH LOCKED: No verification token found"
    echo ""
    echo "To unlock push:"
    echo "1. Run: ./dev.bat validate (Windows) or ./ci-local.sh (WSL)"
    echo "2. Run: ./review.bat (Windows) or ./review.sh (WSL)"
    echo "3. Type 'APPROVED' to generate token"
    echo "4. Retry push within 1 hour"
    
    audit_log "PUSH_BLOCKED: No token found"
    exit 1
fi

# Validate token age (expires in 1 hour)
if [[ $(find "$TOKEN_FILE" -mmin +60 2>/dev/null) ]]; then
    echo "⚠️ TOKEN EXPIRED: Verification token is older than 1 hour"
    echo ""
    echo "Security policy requires fresh validation for each push."
    echo "Re-run validation and review process."
    
    rm "$TOKEN_FILE"
    audit_log "PUSH_BLOCKED: Token expired"
    exit 1
fi

# Validate token content
if [ ! -s "$TOKEN_FILE" ] || ! grep -q "REVIEWED" "$TOKEN_FILE"; then
    echo "❌ INVALID TOKEN: Token file corrupted or invalid"
    echo ""
    echo "Re-run security review process to generate valid token."
    
    rm "$TOKEN_FILE"
    audit_log "PUSH_BLOCKED: Invalid token"
    exit 1
fi

# Token is valid - proceed with push
echo "🔓 TOKEN VALID: Proceeding with MIL-SPEC verified push"
echo ""

# Log successful authorization
audit_log "PUSH_AUTHORIZED: Token validated, push proceeding"

# Generate build artifact hash for audit trail
if command -v sha256sum >/dev/null 2>&1; then
    BUILD_HASH=$(find target -name "*.rlib" -o -name "crucible*" 2>/dev/null | head -5 | xargs sha256sum 2>/dev/null | sha256sum | cut -d' ' -f1)
    if [ -n "$BUILD_HASH" ]; then
        echo "📊 Build Artifact Hash: $BUILD_HASH"
        audit_log "BUILD_HASH: $BUILD_HASH"
    fi
fi

# Clean up token (single use)
rm "$TOKEN_FILE"
audit_log "TOKEN_CONSUMED: Single-use token removed"

echo "✅ Push authorization complete"
echo "📋 Audit trail: $AUDIT_LOG"

exit 0