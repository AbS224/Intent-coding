# Crucible Engine - Development Setup

## Windows Development Environment

### Quick Start

1. **Run the demo:**
   ```cmd
   dev.bat demo
   ```

2. **Open the frontend:**
   - Open `frontend/index.html` in your browser
   - Enter requirements and see the Intent-AST generation

### Project Structure

```
FinalTry/
├── crucible-core/          # Core types and logic
│   ├── src/
│   │   ├── lib.rs          # Core types (IntentAst, Requirement)
│   │   └── main.rs         # Demo binary
│   └── Cargo.toml
├── frontend/
│   └── index.html          # Web interface
├── docs/                   # Architecture documentation
├── dev.bat                 # Development helper script
└── Cargo.toml             # Workspace configuration
```

### Development Commands

```cmd
# Run the core demo
dev.bat demo

# Build the project
dev.bat build

# Run tests
dev.bat test

# Clean build artifacts
dev.bat clean
```

### Current Features

- ✅ Core Rust workspace setup
- ✅ Basic Intent-AST types
- ✅ Requirement processing
- ✅ JSON serialization
- ✅ Simple web frontend
- ✅ Windows-optimized build

### Next Steps

1. Add Tree-Sitter parser for natural language
2. Integrate Z3 SMT solver for verification
3. Build real-time web interface
4. Add formal verification pipeline

### WSL Bridge (Future)

When ready for Linux tools:
```bash
# In WSL
cd /mnt/l/FinalTry
cargo build
```

The project is designed to work on both Windows and WSL seamlessly.