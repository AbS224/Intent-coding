#!/bin/bash
echo "🔍 MIL-SPEC Security Review"
echo "=========================="

echo ""
echo "Recent changes:"
git diff --name-only HEAD~1 HEAD 2>/dev/null || git diff --name-only --cached

echo ""
echo "Security scan results:"
echo "----------------------"
grep -r "password\|secret\|token" --include="*.py" --include="*.js" --include="*.rs" . 2>/dev/null && echo "SECRETS FOUND" || echo "Secrets: CLEAN"
grep -r "eval(\|innerHTML" --include="*.js" . 2>/dev/null && echo "XSS RISKS FOUND" || echo "XSS: CLEAN"
grep -r "\.\./\|C:\\\\" --include="*.py" --include="*.js" . 2>/dev/null && echo "HARDCODED PATHS FOUND" || echo "Paths: CLEAN"

echo ""
echo "Build status:"
if command -v cargo >/dev/null 2>&1; then
    if cargo check >/dev/null 2>&1; then
        echo "Build: PASSED"
    else
        echo "Build: FAILED"
    fi
else
    echo "Build: CARGO NOT AVAILABLE"
fi

echo ""
echo "Files to be pushed:"
git status --porcelain 2>/dev/null || echo "No staged changes"

echo ""
read -p "Review complete? Type 'APPROVED' to unlock: " confirm
if [ "$confirm" = "APPROVED" ]; then
    echo "REVIEWED" > .push_token
    echo "✅ Push unlocked - run 'git push origin main'"
    echo ""
    echo "🔓 MIL-SPEC security lock removed"
    echo "⏰ Token expires in 1 hour"
else
    echo "❌ Review not approved - push remains blocked"
fi