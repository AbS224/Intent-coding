# Local CI/CD with Podman

GitHub Actions are now available for continuous testing and workflows. This document describes both the cloud CI/CD (GitHub Actions) and local fallback options using Podman.

## GitHub Actions CI/CD

The repository includes comprehensive GitHub workflows:

### Available Workflows

1. **[`.github/workflows/ci.yml`](.github/workflows/ci.yml)** - Main CI pipeline:
   - Security scanning (secrets, XSS vulnerabilities)
   - Rust quality checks (fmt, clippy, build, test)
   - Code coverage tracking (Codecov, Coveralls)
   - Docker image builds
   - MIL-SPEC compliance validation

2. **[`.github/workflows/release.yml`](.github/workflows/release.yml)** - Automated releases:
   - Triggers on version tags (v*.*.*)
   - Builds all workspace crates
   - Creates GitHub release with artifacts

3. **[`.github/workflows/pages.yml`](.github/workflows/pages.yml)** - Documentation:
   - Deploys Jekyll site to GitHub Pages

4. **[`.github/dependabot.yml`](.github/dependabot.yml)** - Dependency updates:
   - Weekly Cargo dependency updates
   - GitHub Actions updates
   - Monthly Docker dependency updates

### Local Fallback (Podman)

If GitHub Actions runners are unavailable, use the local Podman-based CI.

## Setup

1. **Install Podman in WSL** (if not already installed):

```bash
sudo apt update
sudo apt install podman
```

1. **Make scripts executable**:

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

### GitHub Actions (Primary)

- **Full cloud CI/CD** with no local resource usage
- **Parallel job execution** for faster builds
- **Code coverage integration** with Codecov/Coveralls
- **Automated releases** with semantic versioning
- **Dependabot** for automatic dependency updates

### Podman Local Fallback

- **Same validation** as GitHub Actions
- **Faster feedback** (runs locally)
- **Offline capability**
- **Full MIL-SPEC compliance**
- **Security-first approach**

## Container Details

The CI container includes:

- Rust 1.75 with rustfmt, clippy
- Python 3 with black, flake8, mypy
- Node.js with prettier, eslint
- All security scanning tools

Container is built once and reused for all CI runs.
