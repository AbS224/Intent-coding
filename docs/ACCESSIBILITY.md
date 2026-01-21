# Accessibility & Inclusion

## Our Commitment

The Crucible Engine is committed to accessibility and inclusion for all developers, regardless of ability. We follow WCAG 2.2 guidelines and prioritize:

- **Vision Accessibility:** Screen reader compatibility, high contrast support, alt text for all images
- **Cognitive Accessibility:** Clear, concise language, logical information flow, reduced cognitive load
- **Motor Accessibility:** Keyboard navigation, sufficient time for interactions
- **Universal Design:** Benefits everyone through better user experience

## Standards We Follow

- **WCAG 2.2 AA** (Web Content Accessibility Guidelines)
- **Section 508** (U.S. Federal accessibility requirements)
- **GitHub Accessibility Guidelines**
- **Inclusive Design Principles**

## Repository Features

### Documentation
- Semantic Markdown structure with proper heading hierarchy
- Alt text for all diagrams and images
- High contrast code examples
- Screen reader friendly navigation

### Code Quality
- Clear, descriptive variable names
- Comprehensive error messages
- Logical code organization
- Automated accessibility linting

### Development Tools
- Keyboard-accessible interfaces
- High contrast themes available
- Screen reader compatible output
- Configurable interaction timeouts

## Automated Accessibility Checks

Our local CI pipeline includes automated accessibility validation:

```bash
# Documentation accessibility
markdownlint --config .markdownlint.json docs/*.md

# Code comment accessibility
cargo doc --document-private-items
# Validates rustdoc accessibility

# Frontend accessibility (when applicable)
axe-core http://localhost:3000
```

## Contributing to Accessibility

We welcome contributions that improve accessibility:

1. **Report Issues:** Use our accessibility issue template
2. **Test Changes:** Run `./ci-local.sh` which includes a11y checks
3. **Follow Guidelines:** Reference this document for standards
4. **Review PRs:** Check for accessibility impact

## Tools & Resources

### Local Development
- **markdownlint:** Documentation accessibility linting
- **axe-core:** Automated web accessibility testing
- **pa11y:** Command-line accessibility auditing
- **textstat:** Readability analysis for documentation

### Browser Extensions
- **WAVE Evaluation Tool**
- **axe DevTools**
- **Accessibility Insights**

## Contact & Feedback

For accessibility questions or feedback:
- **Issues:** Use the "Accessibility" issue template
- **Discussions:** Share experiences in our community discussions
- **Email:** accessibility@crucible-engine.dev

## Accessibility Statement

The Crucible Engine is designed to be usable by everyone. If you encounter accessibility barriers, please let us know so we can improve our tools and documentation for the entire community.