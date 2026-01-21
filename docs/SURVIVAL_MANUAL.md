# CRUCIBLE ENGINE: SURVIVAL MANUAL
**Version:** v0.1.3-alpha
**Last Updated:** January 21, 2026
**Status:** Foundation Phase Complete

## Emergency Recovery Procedures

### Local Tool Directory Setup
For maximum sovereignty and cross-OS consistency:

1. **Create Tool Directory:**
   ```bash
   mkdir -p ~/.kilocode/tools
   ```

2. **WSL-to-Windows Bridge Configuration:**
   - Ensure `~/.kilocode/tools` is accessible from both WSL and Windows
   - Use symlinks for shared binaries
   - Maintain version parity between environments

3. **Essential Tools Inventory:**
   - **Rust/Cargo:** Core compilation toolchain
   - **SPARK-Ada/GNAT:** Formal verification compiler
   - **Z3:** SMT solver for proof generation
   - **Tree-Sitter:** Natural language parsing
   - **Podman:** Local CI/CD container runtime

### Nuclear Safety Integration

#### LOKI Microreactor Invariants
**Critical Safety Thresholds (NUREG-2261 Level 4):**
- Core Temperature: ≤ 1200°C
- Primary Pressure: ≤ 50.0 MPa (Supercritical CO₂)
- Neutron Flux: ≤ 1,000,000 n/cm²/s

#### SPARK-Ada Contract Template
```ada
procedure Execute_SCRAM
with
   Pre => Reactor_Status /= SCRAM,
   Post => Reactor_Status = SCRAM and
           Is_Safe_Temperature(Core_Temperature),
   Contract_Cases => (
     Core_Temperature > 1200 => Reactor_Status = SCRAM,
     others => Reactor_Status = SCRAM
   );
```

#### Bicameral Double Key System
- **UIK (User-Intent Key):** Mission owner authorization
- **LVK (Logic-Verification Key):** Z3-generated proof certificate
- **Ceremony:** Both keys required for state transitions

### Build-in-Public Recovery

#### If Repository State is Lost:
1. **Reinitialize from .kilo:**
   - All mission protocols are in `.kilo`
   - Reconstruct CRs from templates in `docs/templates/`

2. **Security Lock Recovery:**
   ```bash
   # Regenerate security review
   ./review.sh  # For WSL/Linux
   # or
   .\review.bat  # For Windows
   ```

3. **Patent Protection Verification:**
   - Confirm USPTO 63/928,407 markings on all headers
   - Update `.fizz` files with current citation standards

### Development Continuity

#### Daily Workflow:
1. **Status Check:** Review `docs/STATUS.md`
2. **Security Scan:** Run pre-push hooks manually
3. **Documentation First:** Create CR before code changes
4. **Commit Standards:** Use MIL-SPEC format with full traceability

#### Emergency Contacts:
- **Patent Attorney:** For IP protection issues
- **NRC Liaison:** For nuclear safety compliance
- **CISA/DARPA:** For formal methods guidance

---

**"In chaos, the prepared find opportunity." - Crucible Engine Survival Protocol**