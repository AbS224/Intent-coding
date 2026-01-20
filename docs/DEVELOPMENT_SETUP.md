# Development Environment Setup Guide
**Document ID**: CRU-SETUP-1.0-20260120
**Classification**: UNCLASSIFIED
**Prepared By**: Development Team
**Date**: 2026-01-20
**Version**: 1.0

## Distribution List
- Public Documentation
- Developer Community
- Open Source Contributors

## Revision History
| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-01-20 | Development Team | Initial public setup guide |

***

## Executive Summary

This document provides step-by-step instructions for setting up a development environment for Crucible Engine. The guide supports both Windows-only and Windows+WSL configurations, allowing developers to choose their preferred workflow.

## Prerequisites

### System Requirements
- **Operating System**: Windows 10/11 (64-bit)
- **Memory**: 8GB RAM minimum, 16GB recommended
- **Storage**: 10GB free space
- **Network**: Internet connection for package downloads

### Required Software
- **Git**: Version control system
- **Rust**: Programming language and toolchain
- **VS Code**: Recommended IDE (optional but recommended)
- **WSL2**: Windows Subsystem for Linux (optional but recommended)

## Setup Options

Choose one of the following setup paths based on your preferences:

### Option A: Windows-Only Setup (Simpler)
Best for: Beginners, Windows-focused development, resource-constrained systems

### Option B: Windows + WSL Setup (Recommended)
Best for: Advanced users, cross-platform development, access to Linux tools

---

## Option A: Windows-Only Setup

### Step 1: Install Git
1. Download Git from https://git-scm.com/download/windows
2. Run installer with default settings
3. Verify installation:
   ```cmd
   git --version
   ```

### Step 2: Install Rust
1. Visit https://rustup.rs/
2. Download and run `rustup-init.exe`
3. Follow prompts (choose default installation)
4. Restart command prompt
5. Verify installation:
   ```cmd
   rustc --version
   cargo --version
   ```

### Step 3: Clone Project
```cmd
git clone https://github.com/YourOrg/crucible-engine.git
cd crucible-engine
```

### Step 4: Test Build
```cmd
cargo check
```

### Step 5: Run Demo
```cmd
dev.bat demo
```

---

## Option B: Windows + WSL Setup

### Step 1: Enable WSL2
1. Open PowerShell as Administrator
2. Run:
   ```powershell
   wsl --install
   ```
3. Restart computer when prompted
4. Set up Ubuntu user account when prompted

### Step 2: Install Windows Tools
Follow Steps 1-3 from Option A (Git, Rust, Clone Project)

### Step 3: Configure WSL Environment
1. Open WSL terminal (Ubuntu)
2. Update system:
   ```bash
   sudo apt update && sudo apt upgrade -y
   ```
3. Install development tools:
   ```bash
   sudo apt install -y build-essential curl python3 python3-pip
   ```

### Step 4: Install Rust in WSL
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Step 5: Test WSL Bridge
```cmd
# From Windows command prompt in project directory
bridge.bat status
```

### Step 6: Install Advanced Tools (Optional)
```bash
# In WSL terminal
sudo apt install -y z3 tree-sitter-cli
```

---

## VS Code Setup (Recommended)

### Step 1: Install VS Code
1. Download from https://code.visualstudio.com/
2. Install with default settings

### Step 2: Install Extensions
Required extensions:
- **Rust Analyzer**: Rust language support
- **Remote - WSL**: WSL integration (if using WSL)

Optional extensions:
- **GitLens**: Enhanced Git integration
- **Markdown All in One**: Documentation editing

### Step 3: Open Project
1. Open VS Code
2. File → Open Folder → Select project directory
3. If using WSL: Click "Reopen in WSL" when prompted

### Step 4: Test Build Tasks
1. Press `Ctrl+Shift+P`
2. Type "Tasks: Run Task"
3. Select "Crucible: Build (WSL)" or "Crucible: Build (Windows)"

---

## Verification Steps

### Basic Functionality Test
```cmd
# Test core build
cargo check

# Run demo
dev.bat demo

# Generate documentation
milspec.bat check
```

### WSL Bridge Test (If Using WSL)
```cmd
# Check WSL status
bridge.bat status

# Test WSL build
wsl -d Ubuntu --cd /mnt/c/path/to/project -- cargo check
```

### VS Code Integration Test
1. Open project in VS Code
2. Press `Ctrl+Shift+P`
3. Run "Tasks: Run Task" → "Crucible: Demo"
4. Verify output in terminal

---

## Troubleshooting

### Common Issues

#### "Command not found" errors
**Solution**: Restart terminal/command prompt after installations

#### WSL connection issues
**Solution**: 
```cmd
bridge.bat reset
```

#### Rust compilation errors
**Solution**:
```cmd
cargo clean
cargo check
```

#### Permission denied (WSL)
**Solution**:
```bash
chmod +x *.sh
```

#### Line ending issues
**Solution**: In VS Code, change CRLF to LF in bottom-right status bar

### Getting Help

1. **Check Status**: Run `bridge.bat diagnose` for automated diagnostics
2. **Documentation**: Review project README.md
3. **Issues**: Create GitHub issue with error details
4. **Community**: Join project discussions

---

## Development Workflow

### Daily Development
```cmd
# Start development session
bridge.bat status

# Make changes to code
# ... edit files ...

# Test changes
dev.bat build
dev.bat test

# Run demo
dev.bat demo
```

### Documentation Updates
```cmd
# Generate MIL-SPEC documents
milspec.bat generate

# Check compliance
milspec.bat check

# Review documents
milspec.bat review
```

### Before Committing
```cmd
# Clean build
cargo clean
cargo check

# Run tests
cargo test

# Check documentation
milspec.bat check
```

---

## Environment Variables

### Optional Configuration
Add to your system environment variables:

```
RUST_LOG=info
CRUCIBLE_ENV=development
```

### WSL Path Configuration
If using WSL, ensure Windows paths are accessible:
```bash
# Add to ~/.bashrc
export PATH=$PATH:"/mnt/c/Users/$USER/AppData/Local/Programs/Microsoft VS Code/bin"
```

---

## Performance Optimization

### For 8GB Systems
- Close unnecessary applications during builds
- Use Windows-only setup if WSL is too resource-intensive
- Consider increasing virtual memory

### For 16GB+ Systems
- Use WSL setup for best development experience
- Enable all optional tools
- Consider running multiple development environments

---

## Security Considerations

### Development Security
- Keep Rust toolchain updated: `rustup update`
- Regularly update WSL: `sudo apt update && sudo apt upgrade`
- Use official package sources only

### Project Security
- Never commit sensitive information
- Use `.gitignore` for local configuration files
- Review all dependencies before adding

---

## Next Steps

After successful setup:

1. **Explore Codebase**: Review `src/` directories
2. **Read Documentation**: Check `docs/` folder
3. **Run Examples**: Try different demo modes
4. **Contribute**: See CONTRIBUTING.md for guidelines

---

**Setup Principle**: "Simple to start, powerful when needed, secure by default."