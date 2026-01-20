# Local CI/CD with Podman

Since GitHub Actions runners are disabled due to billing issues, we've implemented a local CI/CD system using Podman that provides the same validation.

## Setup

1. **Install Podman in WSL** (if not already installed):
```bash
sudo apt update
sudo apt install podman
```

2. **Make scripts executable**:
```bash
chmod +x ci-local.sh
chmod +x review.sh
```

## Usage

### Windows (from PowerShell/CMD)
```batch
# Run full CI pipeline
dev.bat validate

# Run CI + security review for push
dev.bat push
```

### WSL (from bash)
```bash
# Run CI pipeline directly
./ci-local.sh

# Run security review
./review.sh
```

## Workflow

1. **Local CI Pipeline** (`ci-local.sh`):
   - Security scan (secrets, vulnerabilities)
   - Rust quality (fmt, clippy, build, test)
   - MIL-SPEC compliance check
   - Python syntax validation

2. **Security Review** (`review.sh`):
   - Manual inspection of changes
   - Security scan results review
   - Manual approval required
   - Creates unlock token

3. **Git Push**:
   - Pre-push hook validates unlock token
   - Removes token and allows push
   - Blocks push if no approval

## Benefits

- **Same validation** as GitHub Actions
- **Faster feedback** (runs locally)
- **No billing dependency**
- **Full MIL-SPEC compliance**
- **Security-first approach**

## Container Details

The CI container includes:
- Rust 1.75 with rustfmt, clippy
- Python 3 with black, flake8, mypy
- Node.js with prettier, eslint
- All security scanning tools

Container is built once and reused for all CI runs.