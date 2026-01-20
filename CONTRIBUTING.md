# Contributing to Crucible Engine

Welcome to the Crucible Engine project! We're building the future of formally verified, correct-by-construction software development.

## 🛡️ Requirements

All contributions must meet these standards:
- **Formal verification** where applicable (logic must be mathematically proven)
- **MIL-SPEC documentation** standards (MIL-STD-498 compliance)
- **Security-first development** approach
- **Zero-trust architecture** principles

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+
- Node.js 18+
- Python 3.11+
- Docker & Docker Compose
- Git with GPG signing configured

### Development Setup
```bash
# Clone and setup
git clone https://github.com/AbS224/Intent-coding.git
cd Intent-coding
./dev.bat setup    # Windows
./setup-wsl.sh     # WSL/Linux
```

## 📋 Contribution Process

### 1. Fork & Branch
```bash
git checkout -b feature/amazing-feature
```

### 2. Development Standards

#### Code Quality
- **Rust**: `cargo fmt` + `cargo clippy` (zero warnings)
- **Python**: `black` + `flake8` + `mypy`
- **JavaScript**: `prettier` + `eslint`
- **All**: Pass security scans and build verification

#### Documentation
- Update relevant MIL-SPEC documents in `docs/`
- Add inline documentation for complex logic
- Update `BUILD_CHECKLIST.md` if adding milestones

#### Testing
- Unit tests for all new functionality
- Integration tests for system interactions
- Formal verification proofs for critical logic
- Security testing for input validation

### 3. Pre-Commit Validation
```bash
# Run MIL-SPEC validation
./dev.bat validate    # Windows
./validate.sh         # WSL/Linux
```

### 4. Submit Pull Request

#### PR Requirements
- [ ] All CI checks passing
- [ ] Security scan clean
- [ ] Documentation updated
- [ ] Tests added/updated
- [ ] Formal verification evidence (if applicable)

#### PR Template
```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation update
- [ ] Security improvement

## Verification
- [ ] Code builds successfully
- [ ] Tests pass
- [ ] Security scan clean
- [ ] Documentation updated

## Formal Verification
- [ ] Logic formally verified (if applicable)
- [ ] Proof objects generated
- [ ] Verification evidence attached
```

## 🏛️ Areas We Need Help

### Formal Methods
- Z3 SMT solver constraint generation
- SPARK/Ada integration
- Proof visualization and validation
- Theorem prover bindings

### AI/ML
- Adversarial testing algorithms
- Natural language requirement analysis
- Intent-AST optimization
- ANNIE AI enhancement

### Frontend/UX
- React/TypeScript components
- D3.js visualizations
- Real-time parsing feedback
- Mobile-responsive design

### Security & Compliance
- Post-quantum cryptography implementation
- Compliance automation (ISO 26262, DO-178C)
- Security audit tooling
- Penetration testing

## 📖 Code of Conduct

### Our Standards
- **Technical Excellence**: All code must pass formal verification where applicable
- **Security First**: Every change is security-reviewed
- **Documentation**: Follow MIL-STD-498 standards
- **Respectful Community**: Inclusive and professional interactions
- **Transparency**: Build in public with clear communication

### Enforcement
- Code review required for all changes
- Security review for sensitive modifications
- Community guidelines enforced consistently
- Appeals process available for disputes

## 🎯 Current Focus Areas

### Phase 1: Foundation ✅ COMPLETE
- Development environment and tooling
- Basic Intent-AST structures
- MIL-SPEC documentation framework
- Security hardening (25 vulnerabilities fixed)

### Phase 2: Parser Integration (Current)
- Tree-Sitter natural language parsing
- Real-time requirement validation
- Enhanced error detection
- Z3 SMT solver integration

### Phase 3: Formal Verification (Next)
- Mathematical proof generation
- Verification pipeline completion
- Proof visualization
- Compliance certification

## 📞 Getting Help

- **Discussions**: Use GitHub Discussions for questions
- **Issues**: Report bugs via GitHub Issues
- **Security**: Private vulnerability reporting
- **Email**: abs224@users.noreply.github.com

## 🏆 Recognition

Contributors will be recognized in:
- Release notes and changelogs
- Academic publications (with permission)
- Conference presentations
- Project documentation

---

**"Correct by Design, Not by Debugging. Better by Measurement. Built in Public."**

Thank you for helping build the future of verified software development!