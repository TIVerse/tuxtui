# GitHub Configuration for tuxtui

This directory contains GitHub-specific configuration files for the tuxtui project.

## 📁 Contents

### Workflows (`workflows/`)

- **`ci.yml`** - Continuous Integration pipeline
  - Format checking (`cargo fmt`)
  - Linting (`cargo clippy`)
  - Testing on Linux, macOS, Windows
  - Documentation building
  - Code coverage (codecov)
  - MSRV verification (Rust 1.85.0)

### Issue Templates (`ISSUE_TEMPLATE/`)

- **`bug_report.md`** - Template for bug reports
- **`feature_request.md`** - Template for feature requests
- **`documentation.md`** - Template for documentation improvements
- **`question.md`** - Template for general questions
- **`config.yml`** - Issue template configuration with community links

### Pull Request Template

- **`pull_request_template.md`** - Standardized PR template with checklist

### Dependency Management

- **`dependabot.yml`** - Automated dependency updates
  - Weekly Cargo dependency updates
  - Weekly GitHub Actions updates
  - Grouped patch and minor updates
  - Automatic labeling

## 🔧 CI Pipeline

The CI pipeline runs on every push and pull request:

```yaml
Jobs:
├── Format Check      (rustfmt)
├── Clippy Linting    (clippy with -D warnings)
├── Tests             (Linux, macOS, Windows + stable & MSRV)
├── Documentation     (cargo doc with -D warnings)
├── Coverage          (tarpaulin + codecov)
└── MSRV Check        (Rust 1.85.0)
```

## 📝 Using Issue Templates

When creating an issue, you'll see options for:

1. **🐛 Bug Report** - Report bugs or unexpected behavior
2. **✨ Feature Request** - Suggest new features or enhancements
3. **📚 Documentation** - Improve documentation
4. **❓ Question** - Ask questions (consider Discussions first)

## 🔄 Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests locally:
   ```bash
   cargo test --workspace
   cargo clippy --all-targets --all-features
   cargo fmt --check
   ```
5. Push to your fork
6. Open a PR using the template
7. Wait for CI checks to pass
8. Address review feedback

## 🤖 Dependabot

Dependabot automatically:
- Checks for dependency updates weekly (Mondays)
- Creates PRs with grouped updates
- Labels PRs appropriately (`dependencies`, `rust`, `ci`)
- Limits open PRs (10 for Cargo, 5 for Actions)

## 📊 Code Coverage

Coverage reports are automatically uploaded to Codecov on CI runs. Add a badge to your README:

```markdown
[![codecov](https://codecov.io/gh/TIVerse/tuxtui/branch/master/graph/badge.svg)](https://codecov.io/gh/TIVerse/tuxtui)
```

## 🔗 Related Files

- [`/CONTRIBUTING.md`](../CONTRIBUTING.md) - Contribution guidelines
- [`/README.md`](../README.md) - Project documentation
- [`/MISSING_FEATURES.md`](../MISSING_FEATURES.md) - Feature roadmap

## 🆘 Support

- **Bugs**: Open an issue using the bug report template
- **Features**: Open an issue using the feature request template
- **Questions**: Use [GitHub Discussions](https://github.com/TIVerse/tuxtui/discussions)
- **Security**: See [SECURITY.md](../SECURITY.md) (if exists)

---

**Maintained by:** Tonmoy Infrastructure & Vision  
**License:** MIT
