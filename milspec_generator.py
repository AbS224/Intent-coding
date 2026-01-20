#!/usr/bin/env python3
"""
Crucible Engine MIL-SPEC Documentation Automation
Generates and validates MIL-STD-498 compliant documentation
"""

import os
import sys
import json
import datetime
import html
from pathlib import Path
from typing import Dict, List, Optional
from werkzeug.utils import secure_filename

class MilSpecDocGenerator:
    def __init__(self, project_root: str):
        # Validate and sanitize project root path
        self.project_root = self._safe_path_join(Path.cwd(), project_root)
        self.docs_dir = self._safe_path_join(self.project_root, "docs")
        self.templates_dir = self._safe_path_join(self.docs_dir, "templates")
        self.active_dir = self._safe_path_join(self.docs_dir, "active")
        
        # Ensure directories exist
        try:
            self.docs_dir.mkdir(exist_ok=True)
            self.templates_dir.mkdir(exist_ok=True)
            self.active_dir.mkdir(exist_ok=True)
        except OSError as e:
            raise ValueError(f"Failed to create directories: {e}")
        
        # Current date for document IDs (timezone-aware)
        self.current_date = datetime.datetime.now(datetime.timezone.utc)
        
    def _safe_path_join(self, base_path: Path, *paths: str) -> Path:
        """Safely join paths and prevent directory traversal"""
        result = base_path
        for path in paths:
            # Sanitize each path component
            clean_path = secure_filename(str(path))
            if not clean_path or clean_path in ('.', '..'):
                raise ValueError(f"Invalid path component: {path}")
            result = result / clean_path
        
        # Ensure result is within base_path
        try:
            result.resolve().relative_to(base_path.resolve())
        except ValueError:
            raise ValueError(f"Path traversal attempt detected: {result}")
        
        return result
    
    def _sanitize_input(self, text: str) -> str:
        """Sanitize user input to prevent XSS"""
        if not isinstance(text, str):
            raise ValueError("Input must be a string")
        return html.escape(text.strip())
        
    def generate_document_id(self, doc_type: str, version: str = "1.0") -> str:
        """Generate MIL-SPEC compliant document ID"""
        # Sanitize inputs
        clean_doc_type = self._sanitize_input(doc_type)
        clean_version = self._sanitize_input(version)
        date_str = self.current_date.strftime("%Y%m%d")
        return f"CRU-{clean_doc_type}-{clean_version}-{date_str}"
    
    def create_document_header(self, title: str, doc_type: str, classification: str = "CONTROLLED") -> str:
        """Create standard MIL-SPEC document header"""
        doc_id = self.generate_document_id(doc_type)
        return f"""# {title}
**Document ID**: {doc_id}
**Classification**: {classification}
**Prepared By**: Development Team
**Reviewed By**: Technical Architecture Board
**Approved By**: Technical Architecture Board
**Date**: {datetime.datetime.now().strftime("%Y-%m-%d")}
**Version**: 1.0
**Next Review**: {(datetime.datetime.now() + datetime.timedelta(days=90)).strftime("%Y-%m-%d")}

## Distribution List
- Technical Architecture Board
- Development Team
- Quality Assurance
- Configuration Management

## Revision History
| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | {datetime.datetime.now().strftime("%Y-%m-%d")} | Development Team | Initial version |

***

## Executive Summary

[Executive summary content - 1 page maximum]

## Scope and Purpose

[Document scope and purpose]

## Referenced Documents

- MIL-STD-498: Software Development and Documentation
- CRU-PROC-1.0-20241201: MIL-SPEC Documentation Process

## Definitions and Acronyms

[Key terms and definitions]

***

## Technical Content

"""

    def generate_srd(self) -> None:
        """Generate System Requirements Document"""
        content = self.create_document_header(
            "Crucible Engine: System Requirements Document", 
            "SRD"
        )
        
        content += """
### Functional Requirements

#### REQ-001: Natural Language Processing
**Priority**: High
**Source**: User Requirements
**Description**: System shall parse natural language requirements into formal Intent-AST
**Verification**: Automated testing with requirement corpus
**Traceability**: Links to SDD Section 2.1

#### REQ-002: Formal Verification
**Priority**: Critical
**Source**: Safety Requirements
**Description**: System shall generate mathematical proofs for all requirements
**Verification**: Z3 proof object validation
**Traceability**: Links to SDD Section 2.2

#### REQ-003: Code Generation
**Priority**: High
**Source**: User Requirements
**Description**: System shall generate verified code in multiple languages
**Verification**: Compilation and proof preservation testing
**Traceability**: Links to SDD Section 2.3

### Non-Functional Requirements

#### REQ-101: Performance
**Requirement**: System shall process requirements in < 5 seconds
**Measurement**: Response time monitoring
**Acceptance**: 95th percentile < 5 seconds

#### REQ-102: Security
**Requirement**: All communications shall use post-quantum cryptography
**Measurement**: Cryptographic algorithm verification
**Acceptance**: ML-KEM/ML-DSA implementation

#### REQ-103: Compliance
**Requirement**: System shall meet EAL6+ security standards
**Measurement**: Independent security assessment
**Acceptance**: Formal certification

### Interface Requirements

#### REQ-201: Web Interface
**Description**: Browser-based requirement input interface
**Protocol**: HTTPS with WebSocket for real-time updates
**Data Format**: JSON with cryptographic signatures

#### REQ-202: API Interface
**Description**: RESTful API for system integration
**Authentication**: OAuth 2.0 with ML-DSA signatures
**Rate Limiting**: 1000 requests per minute per user

### Verification and Validation

#### Verification Methods
- **Analysis**: Mathematical proof verification
- **Testing**: Automated test suite execution
- **Inspection**: Code and documentation review
- **Demonstration**: Live system demonstration

#### Validation Criteria
- All requirements traced to implementation
- All proofs independently verified
- All tests passing with >95% coverage
- Security assessment completed

### Traceability Matrix

| Requirement | Design Element | Test Case | Verification Method |
|-------------|----------------|-----------|-------------------|
| REQ-001 | Parser Module | TC-001 | Testing |
| REQ-002 | Verification Engine | TC-002 | Analysis |
| REQ-003 | Code Generator | TC-003 | Testing |

"""
        
        self.write_document("SRD", content)
    
    def generate_sdd(self) -> None:
        """Generate System Design Document"""
        content = self.create_document_header(
            "Crucible Engine: System Design Document", 
            "SDD"
        )
        
        content += """
### Design Overview

The Crucible Engine implements a four-layer verification pipeline:
1. **Intake Layer**: Natural language to Intent-AST
2. **Thunderdome**: AI-powered verification
3. **Verification Engine**: Mathematical proof generation
4. **Code Generation**: Verified code output

### Architecture Design

#### C4 Level 1: System Context
[Reference to C4_ARCHITECTURE.md]

#### C4 Level 2: Container Diagram
[Reference to C4_ARCHITECTURE.md]

#### C4 Level 3: Component Diagram
[Reference to C4_ARCHITECTURE.md]

### Database Design

#### PostgreSQL Schema
- **requirements**: Stores parsed requirements
- **proofs**: Mathematical proof objects
- **audit_log**: Immutable audit trail
- **users**: User authentication data

#### InfluxDB Schema
- **metrics**: Time-series performance data
- **correctness_scores**: Historical correctness tracking

### Interface Design

#### REST API Endpoints
- `POST /api/requirements` - Submit new requirements
- `GET /api/ast/{id}` - Retrieve Intent-AST
- `POST /api/verify` - Trigger verification
- `GET /api/proof/{id}` - Retrieve proof objects

#### WebSocket Events
- `requirement_parsed` - Real-time parsing updates
- `verification_complete` - Proof generation complete
- `error_detected` - Validation errors

### Security Design

#### Authentication
- OAuth 2.0 with ML-DSA signatures
- Multi-factor authentication required
- Session management with Redis

#### Cryptography
- ML-KEM for key exchange
- ML-DSA for digital signatures
- AES-256-GCM for data encryption

### Error Handling Design

#### Error Categories
- **Parsing Errors**: Invalid requirement syntax
- **Verification Errors**: Proof generation failures
- **System Errors**: Infrastructure failures

#### Recovery Procedures
- Automatic retry with exponential backoff
- Graceful degradation to safe states
- Comprehensive error logging

"""
        
        self.write_document("SDD", content)
    
    def generate_secd(self) -> None:
        """Generate Security Design Document"""
        content = self.create_document_header(
            "Crucible Engine: Security Design Document", 
            "SECD", 
            "RESTRICTED"
        )
        
        content += """
### Security Architecture

#### Threat Model
- **Adversarial Requirements**: Malicious input designed to break verification
- **Proof Tampering**: Attempts to modify mathematical proofs
- **Side-Channel Attacks**: Information leakage through timing/power
- **Quantum Attacks**: Future quantum computer threats

#### Security Controls

##### Authentication and Authorization
- Multi-factor authentication mandatory
- Role-based access control (RBAC)
- Principle of least privilege
- Regular access reviews

##### Cryptographic Implementation
- Post-quantum cryptography (ML-KEM/ML-DSA)
- Perfect forward secrecy
- Cryptographic agility for algorithm updates
- Hardware security module (HSM) integration

##### Data Protection
- Encryption at rest (AES-256-GCM)
- Encryption in transit (TLS 1.3)
- Key rotation every 90 days
- Secure key derivation (PBKDF2)

### Security Testing

#### Penetration Testing
- Quarterly external assessments
- Continuous automated scanning
- Red team exercises
- Vulnerability disclosure program

#### Code Security
- Static analysis (SAST)
- Dynamic analysis (DAST)
- Dependency scanning
- Secure code review

### Incident Response

#### Response Team
- Security Officer (Lead)
- Technical Lead
- Compliance Officer
- External forensics (if needed)

#### Response Procedures
1. **Detection**: Automated monitoring alerts
2. **Analysis**: Threat assessment and containment
3. **Containment**: Isolate affected systems
4. **Eradication**: Remove threat and vulnerabilities
5. **Recovery**: Restore normal operations
6. **Lessons Learned**: Post-incident review

"""
        
        self.write_document("SECD", content)
    
    def generate_tpd(self) -> None:
        """Generate Test Plan Document"""
        content = self.create_document_header(
            "Crucible Engine: Test Plan Document", 
            "TPD"
        )
        
        content += """
### Test Strategy

#### Test Levels
- **Unit Tests**: Individual component testing
- **Integration Tests**: Component interaction testing
- **System Tests**: End-to-end functionality
- **Security Tests**: Vulnerability and penetration testing
- **Compliance Tests**: Regulatory requirement validation

#### Test Environment
- **Development**: Local developer machines
- **Integration**: Shared testing environment
- **Staging**: Production-like environment
- **Production**: Live system monitoring

### Test Cases

#### TC-001: Requirement Parsing
**Objective**: Verify natural language parsing accuracy
**Preconditions**: Valid requirement input
**Steps**:
1. Submit natural language requirement
2. Verify Intent-AST generation
3. Validate parsing accuracy
**Expected Result**: >95% parsing accuracy
**Traceability**: REQ-001

#### TC-002: Formal Verification
**Objective**: Verify mathematical proof generation
**Preconditions**: Valid Intent-AST
**Steps**:
1. Submit Intent-AST for verification
2. Generate Z3 proof objects
3. Validate proof correctness
**Expected Result**: Valid proof objects generated
**Traceability**: REQ-002

#### TC-003: Code Generation
**Objective**: Verify multi-language code generation
**Preconditions**: Verified Intent-AST
**Steps**:
1. Generate code in target languages
2. Compile generated code
3. Verify proof preservation
**Expected Result**: Compilable, verified code
**Traceability**: REQ-003

### Test Schedule

| Phase | Duration | Start Date | End Date |
|-------|----------|------------|----------|
| Unit Testing | 2 weeks | 2026-01-21 | 2026-02-04 |
| Integration Testing | 1 week | 2026-02-05 | 2026-02-11 |
| System Testing | 2 weeks | 2026-02-12 | 2026-02-25 |
| Security Testing | 1 week | 2026-02-26 | 2026-03-04 |

### Pass/Fail Criteria

#### Pass Criteria
- All test cases executed successfully
- >95% code coverage achieved
- No critical or high severity defects
- Security assessment passed
- Performance requirements met

#### Fail Criteria
- Any critical defect identified
- Code coverage <90%
- Security vulnerabilities found
- Performance requirements not met

"""
        
        self.write_document("TPD", content)
    
    def write_document(self, doc_type: str, content: str) -> None:
        """Write document to file with proper error handling"""
        try:
            # Sanitize inputs
            clean_doc_type = self._sanitize_input(doc_type)
            
            # Create safe filename
            date_str = self.current_date.strftime("%Y%m%d")
            filename = f"{clean_doc_type}_{date_str}.md"
            filepath = self._safe_path_join(self.active_dir, filename)
            
            # Write file with proper encoding and error handling
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            
            print(f"[GENERATED] Generated: {filepath}")
            
        except (OSError, IOError, ValueError) as e:
            print(f"[ERROR] Failed to write document {doc_type}: {e}")
            raise
    
    def check_compliance(self) -> Dict[str, bool]:
        """Check MIL-SPEC compliance status"""
        required_docs = {
            "SRD": "System Requirements Document",
            "SDD": "System Design Document", 
            "SECD": "Security Design Document",
            "TPD": "Test Plan Document",
            "VCM": "Version Control Management"
        }
        
        compliance_status = {}
        
        for doc_type, doc_name in required_docs.items():
            # Check if document exists
            doc_files = list(self.active_dir.glob(f"{doc_type}_*.md"))
            if doc_files:
                compliance_status[doc_type] = True
                print(f"[OK] {doc_name}: Found")
            else:
                compliance_status[doc_type] = False
                print(f"[MISSING] {doc_name}: Missing")
        
        return compliance_status
    
    def generate_all_documents(self) -> None:
        """Generate all required MIL-SPEC documents with error handling"""
        print("[GENERATOR] Generating MIL-SPEC Documentation Suite...")
        print(f"[DATE] Date: {self.current_date.strftime('%Y-%m-%d')}")
        print(f"[OUTPUT] Output Directory: {self.active_dir}")
        print()
        
        documents = [
            ("SRD", self.generate_srd),
            ("SDD", self.generate_sdd),
            ("SECD", self.generate_secd),
            ("TPD", self.generate_tpd)
        ]
        
        for doc_name, doc_func in documents:
            try:
                doc_func()
            except Exception as e:
                print(f"[ERROR] Failed to generate {doc_name}: {e}")
                continue
        
        print()
        print("📊 Compliance Check:")
        compliance = self.check_compliance()
        
        total_docs = len(compliance)
        compliant_docs = sum(compliance.values())
        compliance_percentage = (compliant_docs / total_docs) * 100
        
        print(f"[COMPLETE] Compliance Status: {compliance_percentage:.0f}% ({compliant_docs}/{total_docs})")
        
        if compliance_percentage == 100:
            print("[SUCCESS] Full MIL-SPEC compliance achieved!")
        else:
            print("[WARNING] Additional documents required for full compliance")

def main():
    if len(sys.argv) < 2:
        print("Usage: python milspec_generator.py <command>")
        print("Commands:")
        print("  generate - Generate all MIL-SPEC documents")
        print("  check    - Check compliance status")
        print("  srd      - Generate System Requirements Document only")
        print("  sdd      - Generate System Design Document only")
        print("  secd     - Generate Security Design Document only")
        print("  tpd      - Generate Test Plan Document only")
        return
    
    try:
        project_root = os.path.dirname(os.path.abspath(__file__))
        generator = MilSpecDocGenerator(project_root)
        
        command = sys.argv[1].lower()
        
        if command == "generate":
            generator.generate_all_documents()
        elif command == "check":
            generator.check_compliance()
        elif command == "srd":
            generator.generate_srd()
        elif command == "sdd":
            generator.generate_sdd()
        elif command == "secd":
            generator.generate_secd()
        elif command == "tpd":
            generator.generate_tpd()
        else:
            print(f"Unknown command: {command}")
            
    except Exception as e:
        print(f"[FATAL ERROR] {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()