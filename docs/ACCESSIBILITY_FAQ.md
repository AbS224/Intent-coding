# ACCESSIBILITY & INCLUSION FAQ
**Context:** Engineering in high-rigor environments with Screen Readers (SR).

### Q1: Why does this repo prioritize the WSL Bridge?
**A:** Graphical Git clients and complex IDE overlays can be noisy for Screen Readers. By using the WSL (Windows Subsystem for Linux) Bridge, we ensure that all engineering output is streamed as clean, structured text that SR software (like NVDA or Jaws) can parse efficiently.

### Q2: How are the documents structured for SR navigation?
**A:** We follow a strict Markdown heading hierarchy (H1 > H2 > H3). This allows SR users to jump between "Intent," "Verification," and "Traceability" sections using standard heading-jump commands (e.g., the 'H' key).

### Q3: Why is ASCII art forbidden in this repository?
**A:** ASCII diagrams (boxes and arrows made of text) are often unintelligible when read aloud. We use structured Mermaid.js code blocks or descriptive text lists. If a visual is required, we provide a "Text-Based Architectural Narrative" alongside it.

### Q4: How do I handle the "Passphrase Fatigue" in the terminal?
**A:** Use the `ssh-agent`. Running `eval "$(ssh-agent -s)" && ssh-add ~/.ssh/id_ed25519` ensures you only have to interact with the passphrase prompt once per session, reducing repetitive audio-entry tasks.

### Q5: Can I use the agentic coding features with a Screen Reader?
**A:** Yes. The `.kilo` protocol is designed to be "Command-Centric." You can instruct the agent to "Read the CR and summarize the Proof section," which provides a clean audio summary of the engineering state without needing to scan thousands of lines of code manually.