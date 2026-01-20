#!/bin/bash
echo "🔍 MIL-SPEC Security Review"
echo "=========================="

echo ""
echo "Recent changes:"
git diff --name-only HEAD~1 HEAD

echo ""
echo "Security scan results:"
echo "----------------------"
grep -r "password\|secret" --include="*.py" --include="*.js" . 2>/dev/null && echo "SECRETS FOUND" || echo "Secrets: CLEAN"
grep -r "eval(\|innerHTML" --include="*.js" . 2>/dev/null && echo "XSS RISKS FOUND" || echo "XSS: CLEAN"

echo ""
read -p "Review complete? Type 'APPROVED' to unlock: " confirm
if [ "$confirm" = "APPROVED" ]; then
    echo "REVIEWED" > .milspec-unlock
    echo "✅ Push unlocked - run 'git push origin main'"
else
    echo "❌ Review not approved"
fi