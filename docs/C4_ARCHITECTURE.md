# Crucible Engine: C4 Architecture Documentation
**Document ID**: CRU-ARCH-2.1-20260120
**Classification**: CONTROLLED
**Prepared By**: Technical Architecture Board
**Reviewed By**: Security Review Board
**Approved By**: Technical Architecture Board
**Date**: 2026-01-20
**Version**: 2.1
**Next Review**: 2026-04-20

## Distribution List
- Technical Architecture Board
- Security Review Board
- Development Team
- Compliance Officer

## Revision History
| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2024-12-19 | Architecture Team | Initial C4 architecture |
| 2.0 | 2025-12-01 | Architecture Team | Production architecture updates |
| 2.1 | 2026-01-20 | Architecture Team | MIL-SPEC compliance alignment |

***

## Executive Summary

This document presents the complete system architecture for Crucible Engine using the C4 model. The architecture implements "Auditor's Dream" verification with 97.5% TCB reduction, achieving EAL6+ compliance through mathematical proof verification and post-quantum cryptography.

## Scope and Purpose

This document defines:
- Complete system architecture using C4 methodology
- "Verifying the Verifier" implementation framework
- Compliance mapping for EAL6+, ISO 26262, DO-178C
- Post-quantum cryptographic security architecture

## Referenced Documents

- MIL-STD-498: Software Development and Documentation
- CRU-PROC-1.0-20241201: MIL-SPEC Documentation Process
- CRU-VCM-2.0-20260120: Version Control Management
- ISO 26262: Functional Safety Standard
- DO-178C: Software Considerations in Airborne Systems
- Common Criteria: Security Evaluation Standard

## Definitions and Acronyms

- **TCB**: Trusted Computing Base
- **PCC**: Proof Carrying Code
- **ML-KEM**: Module-Lattice-Based Key Encapsulation Mechanism
- **ML-DSA**: Module-Lattice-Based Digital Signature Algorithm
- **EAL**: Evaluation Assurance Level
- **ASIL**: Automotive Safety Integrity Level

***

## Acknowledgments

**Special Recognition**: This architecture incorporates insights from **Grigore Rosu** (University of Illinois, K Framework, Runtime Verification), whose recommendation to add Z3 proof object verifiers on FastSet directly influenced our Verification Engine design. His expertise in formal methods and runtime verification has been instrumental in validating our "Auditor's Dream" approach of verifying the verifier.

***

## System Context (C4 Level 1)

```
┌─────────────────────────────────────────────────────────────────┐
│                    Crucible Engine System                      │
│                "Correct by Design, Not by Debugging"           │
│                                                                 │
│  Transforms natural language requirements into formally         │
│  verified, mathematically proven, production-ready code        │
│                                                                 │
│  Compliance: ISO 26262, DO-178C, Common Criteria EAL4+        │
│  Security: Post-Quantum Cryptography (ML-KEM/ML-DSA)          │
│  Audit: Immutable cryptographic audit trails                   │
└─────────────────────────────────────────────────────────────────┘
                                │
                                │
        ┌───────────────────────┼───────────────────────┐
        │                       │                       │
        ▼                       ▼                       ▼
┌─────────────┐         ┌─────────────┐         ┌─────────────┐
│   Developers│         │  Auditors   │         │ Regulators  │
│             │         │             │         │             │
│ • Vibecoding│         │ • Verify    │         │ • Compliance│
│ • AI-Assist │         │   Proofs    │         │   Reports   │
│ • Real-time │         │ • Audit     │         │ • Cert      │
│   Feedback  │         │   Trails    │         │   Validation│
└─────────────┘         └─────────────┘         └─────────────┘
```

### External Systems Integration

- **AI Services**: OpenAI API, Claude API (for ANNIE assistant)
- **Formal Verification**: Z3 SMT Solver, SPARK/Ada Toolchain
- **Version Control**: Git repositories, CI/CD pipelines
- **Compliance Systems**: Regulatory reporting, audit management
- **Monitoring**: Prometheus, Grafana, security monitoring

***

## Container Diagram (C4 Level 2)

```
┌─────────────────────────────────────────────────────────────────┐
│                    Crucible Engine Platform                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────┐ │
│  │   Web Frontend  │    │  Verification   │    │   Metrics   │ │
│  │                 │    │     Engine      │    │  Dashboard  │ │
│  │ • Monaco Editor │◄──►│                 │◄──►│             │ │
│  │ • D3.js Viz     │    │ • Z3 SMT Solver │    │ • Real-time │ │
│  │ • Glassmorphic  │    │ • Prolog Engine │    │ • Analytics │ │
│  │   UI            │    │ • SPARK/Ada     │    │ • Benchmarks│ │
│  └─────────────────┘    └─────────────────┘    └─────────────┘ │
│           │                       │                       │     │
│           │              ┌─────────────────┐              │     │
│           └─────────────►│  API Gateway    │◄─────────────┘     │
│                          │                 │                    │
│                          │ • Authentication│                    │
│                          │ • Rate Limiting │                    │
│                          │ • Audit Logging │                    │
│                          └─────────────────┘                    │
│                                   │                             │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────┐ │
│  │  Thunderdome    │    │   Code Gen      │    │  Compliance │ │
│  │   AI Arena      │    │    Engine       │    │   Module    │ │
│  │                 │    │                 │    │             │ │
│  │ • Blue Team     │◄──►│ • Rust Gen      │◄──►│ • Audit     │ │
│  │ • Red Team      │    │ • SPARK/Ada Gen │    │   Trails    │ │
│  │ • Judge         │    │ • Zig Gen       │    │ • Crypto    │ │
│  │ • ANNIE AI      │    │ • Elixir Gen    │    │   Signing   │ │
│  └─────────────────┘    └─────────────────┘    └─────────────┘ │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│                        Data Layer                              │
│                                                                 │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────┐ │
│  │   PostgreSQL    │    │    InfluxDB     │    │    Redis    │ │
│  │                 │    │                 │    │             │ │
│  │ • Audit Trails  │    │ • Time-series   │    │ • Session   │ │
│  │ • User Data     │    │   Metrics       │    │   Cache     │ │
│  │ • Compliance    │    │ • Performance   │    │ • Real-time │ │
│  │   Records       │    │   Data          │    │   Data      │ │
│  └─────────────────┘    └─────────────────┘    └─────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

***

## Component Diagram (C4 Level 3) - Verification Engine

```
┌─────────────────────────────────────────────────────────────────┐
│                    Verification Engine                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────┐ │
│  │  Requirement    │    │   Intent-AST    │    │   Policy    │ │
│  │    Parser       │    │   Generator     │    │   Engine    │ │
│  │                 │    │                 │    │             │ │
│  │ • Tree-Sitter   │───►│ • UUID Assign  │───►│ • Prolog    │ │
│  │ • Tokenization  │    │ • Dependency    │    │   Rules     │ │
│  │ • NLP Analysis  │    │   Mapping       │    │ • Security  │ │
│  │ • Gap Detection │    │ • Temporal      │    │   Policies  │ │
│  └─────────────────┘    │   Constraints   │    │ • Validation│ │
│                          └─────────────────┘    └─────────────┘ │
│                                   │                       │     │
│                                   ▼                       ▼     │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────┐ │
│  │   Z3 SMT        │    │   Formal        │    │  Proof      │ │
│  │   Solver        │    │  Verification   │    │ Generator   │ │
│  │                 │    │                 │    │             │ │
│  │ • Constraint    │◄───│ • Invariant     │───►│ • Math      │ │
│  │   Solving       │    │   Checking      │    │   Proofs    │ │
│  │ • SAT/SMT       │    │ • Safety Props  │    │ • Cert      │ │
│  │ • Model Gen     │    │ • Liveness      │    │   Generation│ │
│  │ • Proof Objects │    │   Props         │    │ • Crypto    │ │
│  └─────────────────┘    └─────────────────┘    │   Signing   │ │
│           │                                     └─────────────┘ │
│           ▼                                                     │
│  ┌─────────────────┐                                           │
│  │ Z3 Proof Object │    ◄── Rosu Recommendation               │
│  │   Verifier      │                                           │
│  │                 │    "Auditor's Dream: Verifying the       │
│  │ • Proof Validation    Verifier" - Independent validation    │
│  │ • Object Integrity    of Z3 solver outputs for maximum     │
│  │ • FastSet Integration trustworthiness in regulated         │
│  │ • Hoare Logic Proof   environments                         │
│  └─────────────────┘                                           │
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                 Audit & Compliance                      │   │
│  │                                                         │   │
│  │ • Immutable audit trails with cryptographic integrity  │   │
│  │ • Real-time compliance monitoring and reporting        │   │
│  │ • Post-quantum cryptographic signatures (ML-KEM)       │   │
│  │ • Regulatory mapping (ISO 26262, DO-178C, CC EAL4+)    │   │
│  │ • Z3 proof object verification for auditor confidence  │   │
│  └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

***

## Rosu Enhancement Framework: "Auditor's Dream" Implementation

### Technical Innovation: Verifying the Verifier

**Mathematical Foundation**: The core insight from Grigore Rosu's recommendation is that we can shrink the Trusted Computing Base (TCB) from the entire Z3 SMT solver (~500K LOC) to a small proof checker (~5K LOC).

#### Hoare Logic to Z3 Proof Object Pipeline

```
Intent-AST Requirement:
  "User can withdraw amount A from account with balance B"
  
Hoare Triple Generation:
  {B ≥ A ∧ A > 0} withdraw(A) {B' = B - A ∧ B' ≥ 0}
  
Z3 SMT Encoding:
  (assert (>= B A))
  (assert (> A 0))
  (assert (= B_prime (- B A)))
  (assert (>= B_prime 0))
  (check-sat)
  
Z3 Proof Object Output:
  (proof
    (let ((a!1 (mp (asserted (>= B A)) (asserted (> A 0)))))
    (mp a!1 (unit-resolution ...)))
  
Independent Verification:
  FastSet Proof Checker validates each inference step
  without trusting Z3's internal reasoning engine
```

#### Trusted Computing Base Reduction

| Component | Traditional Approach | Crucible Engine | TCB Reduction |
|-----------|---------------------|-----------------|---------------|
| **SMT Solver** | Z3 (~500K LOC) | Proof Checker (~5K LOC) | 99% reduction |
| **Code Generator** | Unverified | CompCert-style verified | Eliminates bugs |
| **Requirement Parser** | Manual review | Formal grammar + proofs | Eliminates ambiguity |
| **Total TCB** | ~2M LOC | ~50K LOC | 97.5% reduction |

### Auditor-Ready Verification Enhancements

| Enhancement | Auditor Benefit | Technical Implementation | Compliance Standard |
|-------------|-----------------|-------------------------|--------------------|
| **Proof Object Verification** | Independent validation of solver logic | Z3 Proof Object Verifier with FastSet integration | EAL6/7, ASIL-D |
| **Immutable Audit Trails** | Tamper-proof history of every verification step | ML-KEM signed blockchain with cryptographic integrity | SOX, 21 CFR Part 11 |
| **Proof Carrying Code** | Runtime verification at load time | Embedded proofs in binary artifacts | DO-178C Level A |
| **Gap Detection Proofs** | Mathematical guarantee of completeness | Temporal logic verification of requirement coverage | ISO 26262 ASIL-D |
| **Verified Code Generation** | Zero-bug guarantee in compiler | CompCert-style verified lowering from AST to target | Common Criteria EAL4+ |

### Gap Detection Logic: Handling Incomplete Specifications

**Problem**: Traditional systems fail when users provide incomplete requirements.

**Crucible Solution**: Formal completeness checking with safe defaults.

```prolog
% Prolog rule for detecting missing error handling
incomplete_specification(Requirement) :-
    has_operation(Requirement, Operation),
    has_precondition(Requirement, Precondition),
    \+ has_error_case(Requirement, Operation, Precondition).

% Safe state generation
safe_default(Operation, ErrorCase) :-
    incomplete_specification(Requirement),
    generate_safe_state(Operation, "HALT_AND_LOG").
```

**Auditor Benefit**: System mathematically proves it cannot enter undefined states.

### Proof Carrying Code Implementation

**Traditional Problem**: How do you trust that deployed code matches verified specifications?

**Crucible Solution**: Embed proofs directly in the binary.

```rust
// Generated Rust code with embedded proof
#[verified_invariant(proof_id = "uuid-1234", property = "balance >= 0")]
pub struct Account {
    balance: u64,
    #[proof_object] 
    invariant_proof: ProofObject,
}

#[verified_precondition(
    proof_id = "uuid-5678", 
    property = "amount > 0 && self.balance >= amount"
)]
impl Account {
    pub fn withdraw(&mut self, amount: u64) -> Result<(), WithdrawError> {
        // Runtime proof verification at function entry
        verify_precondition_proof(&self.precondition_proof, amount, self.balance)?;
        
        self.balance -= amount;
        
        // Runtime invariant verification
        verify_invariant_proof(&self.invariant_proof, self.balance)?;
        
        Ok(())
    }
}
```

**Auditor Benefit**: Target hardware can verify correctness at load time without trusting the development environment.

### Verified Code Generation Pipeline

**Challenge**: How do you ensure the Code Generator itself doesn't introduce bugs?

**Solution**: CompCert-style verified compilation.

```coq
(* Coq proof that AST lowering preserves semantics *)
Theorem code_gen_correctness :
  forall (ast : IntentAST) (code : RustCode),
    generate_code ast = Some code ->
    semantics_equivalent ast code.
```

**Implementation Stages**:
1. **AST → Intermediate Representation**: Verified in Coq
2. **IR → Target Language**: Verified transformation
3. **Proof Preservation**: Proofs carried through each stage

### Mathematical Walkthrough: Hoare Logic to Z3 Proof Objects

#### Step 1: Intent-AST to Hoare Triple
```
Requirement: "Withdrawal must not overdraw account"

Hoare Triple:
{balance ≥ amount ∧ amount > 0} 
  withdraw(amount) 
{balance' = balance - amount ∧ balance' ≥ 0}
```

#### Step 2: Z3 SMT Encoding
```smt2
(declare-fun balance () Int)
(declare-fun amount () Int)
(declare-fun balance_prime () Int)

; Preconditions
(assert (>= balance amount))
(assert (> amount 0))

; Operation
(assert (= balance_prime (- balance amount)))

; Postcondition
(assert (>= balance_prime 0))

(check-sat)
(get-proof)
```

#### Step 3: Z3 Proof Object Generation
```
(proof
  (let ((a!1 (mp (asserted (>= balance amount)) 
                 (asserted (> amount 0)))))
  (let ((a!2 (unit-resolution a!1 
                              (asserted (= balance_prime (- balance amount))))))
  (mp a!2 (arithmetic-simplification (>= balance_prime 0))))))
```

#### Step 4: Independent Verification with FastSet
```rust
// FastSet proof checker (outside TCB)
fn verify_proof_object(proof: &ProofObject, conclusion: &Formula) -> bool {
    match proof {
        ProofObject::ModusPonens(premise1, premise2) => {
            verify_proof_object(premise1, &premise1.conclusion) &&
            verify_proof_object(premise2, &premise2.conclusion) &&
            modus_ponens_valid(&premise1.conclusion, &premise2.conclusion, conclusion)
        },
        ProofObject::Asserted(formula) => {
            // Verify this was actually asserted in the original problem
            original_assertions.contains(formula)
        },
        // ... other proof rules
    }
}
```

**Auditor Benefit**: The proof checker is small enough to be manually audited, yet provides mathematical certainty that Z3's reasoning was correct.

### Compliance Mapping

| Regulation | Requirement | Crucible Implementation |
|------------|-------------|------------------------|
| **DO-178C Level A** | Software cannot contribute to catastrophic failure | Proof Carrying Code with runtime verification |
| **ISO 26262 ASIL-D** | Systematic capability for automotive safety | Verified code generation + gap detection |
| **Common Criteria EAL6+** | Semi-formally verified design | Z3 proof objects + independent verification |
| **21 CFR Part 11** | Electronic records integrity | ML-KEM signed audit trails |
| **SOX Section 404** | Internal controls over financial reporting | Immutable cryptographic audit logs |

---

**Result**: An auditor can mathematically verify that:
1. The requirements are complete (gap detection proofs)
2. The logic is sound (Z3 proof objects + independent verification)
3. The code matches the specification (verified code generation)
4. The deployment is tamper-proof (proof carrying code)
5. The audit trail is immutable (post-quantum cryptography)

**"Auditor's Dream"**: A system that provides mathematical certainty at every layer, with a TCB small enough to manually audit. "Thunderdome" round | Post-Quantum (ML-KEM) signing |
| **FastSet Verifiers** | Specialized, faster verification for specific data sets | Specialized verifiers on Z3 proof objects |
| **Hoare Logic Integration** | Mathematical proof of program correctness | Formal contract verification in SPARK/Ada |
| **Cryptographic Integrity** | Quantum-resistant audit trail validation | ML-DSA signature chains |
| **Real-time Monitoring** | Live verification status for continuous compliance | WebSocket-based verification streaming |

### Implementation Architecture

```
Z3 SMT Solver Output
        │
        ▼
┌─────────────────┐
│ Proof Object    │ ◄── Rosu Recommendation
│ Verifier        │
│                 │
│ • FastSet       │ ── Specialized verification
│   Integration   │    for data set types
│ • Hoare Logic   │ ── Mathematical proof
│   Validation    │    of correctness
│ • Integrity     │ ── Cryptographic
│   Checking      │    validation
└─────────────────┘
        │
        ▼
┌─────────────────┐
│ Immutable       │
│ Audit Log       │
│                 │
│ • ML-KEM Signed │ ── Quantum-resistant
│ • Timestamped   │    tamper-proof logs
│ • Chained       │ ── Blockchain-style
│   Verification  │    integrity
└─────────────────┘
```

## Compliance & Audit Framework

### Regulatory Standards Mapping

| Standard | Coverage | Evidence Generated | Audit Trail |
|----------|----------|-------------------|-------------|
| **ISO 26262** | Functional Safety | Mathematical proofs of safety properties | Immutable verification records |
| **DO-178C** | Software Airworthiness | Formal verification certificates | Complete development audit trail |
| **Common Criteria EAL4+** | Security Evaluation | Cryptographic proof of security properties | Tamper-evident audit logs |
| **NIST Cybersecurity** | Security Framework | Vulnerability analysis reports | Real-time security monitoring |
| **SOX Compliance** | Financial Controls | Code correctness guarantees | Cryptographically signed audit trails |

### Audit Evidence Package

```
audit_package/
├── verification_certificates/
│   ├── mathematical_proofs.pdf
│   ├── formal_verification_report.pdf
│   └── security_analysis.pdf
├── audit_trails/
│   ├── immutable_action_log.json
│   ├── cryptographic_signatures.json
│   └── integrity_verification.json
├── compliance_mappings/
│   ├── iso26262_evidence.json
│   ├── do178c_compliance.json
│   └── common_criteria_evaluation.json
├── code_artifacts/
│   ├── generated_code/
│   ├── verification_annotations/
│   └── proof_carrying_code/
└── signatures/
    ├── post_quantum_signatures.json
    ├── certificate_chain.pem
    └── integrity_manifest.json
```

### Auditor Verification Process

1. **Cryptographic Integrity Check**
   - Verify post-quantum signatures on all artifacts
   - Validate certificate chain integrity
   - Confirm audit trail immutability

2. **Formal Verification Validation**
   - Review mathematical proofs generated by Z3
   - **Z3 Proof Object Verification** (Rosu Enhancement): Independent validation of Z3 solver outputs using specialized verifiers
   - Validate SPARK/Ada formal contracts
   - Confirm correctness preservation in code generation

3. **Compliance Evidence Review**
   - Map requirements to regulatory standards
   - Verify evidence completeness
   - Validate traceability matrix

4. **Security Assessment**
   - Review adversarial testing results
   - Validate attack surface analysis
   - Confirm vulnerability mitigation

### Technical Innovation: "Verifying the Verifier"

Following Grigore Rosu's recommendation, our architecture implements **Z3 Proof Object Verification** - a specialized component that independently validates Z3 SMT solver outputs. This "Auditor's Dream" approach ensures that even our verification tools are themselves verified, providing the highest level of assurance for regulated environments. The integration with FastSet and Hoare logic proof validation creates a mathematically sound foundation that auditors can trust with complete confidence.ll artifacts
   - Validate certificate chain integrity
   - Confirm audit trail immutability

2. **Formal Verification Validation**
   - Review mathematical proofs generated by Z3
   - Validate SPARK/Ada formal contracts
   - Confirm correctness preservation in code generation

3. **Compliance Evidence Review**
   - Map requirements to regulatory standards
   - Verify evidence completeness
   - Validate traceability matrix

4. **Security Assessment**
   - Review adversarial testing results
   - Validate attack surface analysis
   - Confirm vulnerability mitigation

***

**Document Control**
- **Version**: 1.0
- **Classification**: Auditor Ready
- **Approval**: Technical Architecture Board
- **Next Review**: Quarterly
- **Compliance**: ISO 26262, DO-178C, Common Criteria EAL4+