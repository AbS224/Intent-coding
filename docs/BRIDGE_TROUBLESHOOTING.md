# Windows-WSL Bridge Troubleshooting Guide
**Document ID**: CRU-BRIDGE-1.0-20260120
**Classification**: UNCLASSIFIED
**Prepared By**: Development Team
**Date**: 2026-01-20
**Version**: 1.0

## Distribution List
- Public Documentation
- Developer Community
- Technical Support

## Revision History
| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-01-20 | Development Team | Initial troubleshooting guide |

***

## Executive Summary

This guide provides solutions for common issues when developing across Windows and WSL environments. It covers path mismatches, permission conflicts, and bridge connectivity problems.

## Common Bridge Issues

### 1. Path Mismatch Problems

#### Symptoms
- "File not found" errors when paths exist
- Build failures with path-related errors
- Scripts failing to execute

#### Root Cause
Windows uses `C:\Users\...` while WSL uses `/mnt/c/Users/...`

#### Solutions
```cmd
# Check current path format
echo %CD%                    # Windows format
wsl pwd                      # WSL format

# Convert paths
wsl wslpath -w $(pwd)        # Linux to Windows
wsl wslpath -u "C:\path"     # Windows to Linux
```

#### Best Practices
- Keep projects in WSL filesystem (`/home/user/`) for best performance
- Use WSL for Linux tools, Windows for Windows-specific tasks
- Avoid mixing path formats in scripts

### 2. Permission Lock Conflicts

#### Symptoms
- "Permission denied" errors
- Files appear locked or in use
- Cannot delete or modify files

#### Root Cause
Windows and WSL fighting over file access

#### Solutions
```cmd
# Reset WSL bridge
wsl --shutdown
# Wait 5 seconds, then restart WSL

# Fix file permissions in WSL
wsl chmod +x *.sh
wsl chmod 644 *.md

# Check for file locks
wsl lsof /mnt/c/path/to/project
```

#### Prevention
- Close VS Code before major WSL operations
- Use `wsl --shutdown` between development sessions
- Avoid editing same files simultaneously in Windows and WSL

### 3. VS Code Integration Issues

#### Symptoms
- "Terminal failed to launch"
- WSL extension not connecting
- Build tasks failing

#### Solutions
```cmd
# Reset VS Code WSL server
wsl rm -rf ~/.vscode-server

# Check VS Code WSL extension
# 1. Open VS Code
# 2. Extensions → Remote - WSL → Reinstall

# Verify terminal profile
# Settings → Terminal → Default Profile → Ubuntu (WSL)
```

#### Configuration Check
Verify `.vscode/settings.json`:
```json
{
    "terminal.integrated.defaultProfile.windows": "Ubuntu (WSL)",
    "files.eol": "\n"
}
```

### 4. Line Ending Problems

#### Symptoms
- Scripts fail with "command not found"
- Git shows entire files as changed
- Bash scripts won't execute

#### Root Cause
Windows uses CRLF (`\r\n`), Linux uses LF (`\n`)

#### Solutions
```cmd
# Fix line endings in WSL
wsl dos2unix *.sh

# Configure Git (run once)
git config --global core.autocrlf input

# VS Code: Change CRLF to LF in status bar
```

#### Prevention
- Set VS Code to use LF line endings
- Configure Git properly
- Use `.gitattributes` file for consistent line endings

### 5. Environment Variable Issues

#### Symptoms
- Commands work in one environment but not the other
- Path not found errors
- Tool not available

#### Solutions
```cmd
# Check environment variables
echo $PATH                   # In WSL
echo %PATH%                  # In Windows

# Add Windows tools to WSL PATH
export PATH=$PATH:"/mnt/c/Users/$USER/AppData/Local/Programs/Microsoft VS Code/bin"

# Make permanent (add to ~/.bashrc)
echo 'export PATH=$PATH:"/mnt/c/path/to/tools"' >> ~/.bashrc
```

## Diagnostic Commands

### Quick Health Check
```cmd
# Windows side
wsl --list --verbose
wsl --status

# WSL side
wsl uname -a
wsl df -h
wsl ps aux | grep -i vscode
```

### Detailed Diagnostics
```cmd
# Check WSL integration
wsl --list --online
wsl --list --verbose

# Check file system access
wsl ls -la /mnt/c/
dir \\wsl$\Ubuntu\home\

# Check network connectivity
wsl ping google.com
ping google.com
```

### Performance Check
```cmd
# File system performance test
wsl time ls -la /mnt/c/large-directory/
wsl time ls -la ~/large-directory/

# Memory usage
wsl free -h
wsl top -n 1
```

## Automated Fixes

### Reset Script (Windows)
```batch
@echo off
echo Resetting WSL Bridge...
wsl --shutdown
timeout /t 5 /nobreak >nul
wsl --distribution Ubuntu --exec echo "WSL Ready"
echo Bridge reset complete
```

### Permission Fix Script (WSL)
```bash
#!/bin/bash
echo "Fixing permissions..."
find . -name "*.sh" -exec chmod +x {} \;
find . -name "*.md" -exec chmod 644 {} \;
echo "Permissions fixed"
```

### Line Ending Fix Script (WSL)
```bash
#!/bin/bash
echo "Fixing line endings..."
find . -name "*.sh" -exec dos2unix {} \;
find . -name "*.bat" -exec unix2dos {} \;
echo "Line endings fixed"
```

## Prevention Strategies

### Development Workflow
1. **Start Clean**: Run `wsl --shutdown` before starting work
2. **Single Environment**: Use either Windows OR WSL for a task, not both
3. **Regular Cleanup**: Shutdown WSL between major changes
4. **Consistent Tools**: Use same tools in same environment

### Project Setup
1. **Choose Primary Environment**: Windows for simple, WSL for advanced
2. **Configure Git**: Set line ending handling
3. **Document Paths**: Use relative paths in scripts
4. **Test Both**: Verify functionality in both environments

### VS Code Configuration
1. **Use Remote-WSL**: For WSL-based projects
2. **Configure Tasks**: Specify shell explicitly
3. **Set Line Endings**: Default to LF
4. **Install Extensions**: In both Windows and WSL contexts

## Error Code Reference

| Error | Environment | Cause | Solution |
|-------|-------------|-------|----------|
| `E_FAIL (0x80004005)` | Windows | WSL service hung | Restart WSL service |
| `Permission denied` | WSL | File lock conflict | `wsl --shutdown` |
| `Command not found` | WSL | PATH not set | Check environment variables |
| `CRLF will be replaced` | Git | Line ending mismatch | Configure `core.autocrlf` |
| `Terminal failed to launch` | VS Code | WSL server corrupted | Delete `~/.vscode-server` |

## Performance Optimization

### For Better Performance
- Keep frequently accessed files in WSL filesystem
- Use Windows filesystem for large binary files
- Minimize cross-filesystem operations
- Use WSL 2 (not WSL 1)

### Memory Management
- Close unused applications during builds
- Monitor WSL memory usage: `wsl --shutdown` to free memory
- Increase WSL memory limit in `.wslconfig` if needed

### Network Optimization
- Use WSL 2 for better network performance
- Configure Windows Defender exclusions for project directories
- Use local package caches when possible

## Advanced Troubleshooting

### WSL Configuration File
Create `%USERPROFILE%\.wslconfig`:
```ini
[wsl2]
memory=4GB
processors=2
swap=2GB
```

### Network Issues
```cmd
# Reset network stack
wsl --shutdown
netsh winsock reset
netsh int ip reset
# Restart computer
```

### File System Issues
```cmd
# Check file system integrity
wsl fsck /dev/sdc
# Or from Windows
wsl --shutdown
wsl --unregister Ubuntu
wsl --install -d Ubuntu
```

---

**Troubleshooting Principle**: "Isolate, identify, fix, prevent."