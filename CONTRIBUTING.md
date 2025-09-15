# Contributing to Stamp Album Generator

Thank you for your interest in contributing to StampAlbumGen2! This document provides guidelines and information for contributors.

## 🚀 Quick Start

### Development Setup
```bash
# Clone the repository
git clone <repository-url>
cd stamp-album-gen2

# Build in debug mode
cargo build

# Run tests
cargo test

# Build optimized release
cargo build --release
```

### Development Workflow
1. **Fork** the repository
2. **Create** a feature branch: `git checkout -b feature/your-feature-name`
3. **Make** your changes
4. **Test** thoroughly: `cargo test`
5. **Commit** with clear messages: `git commit -m "Add: brief description"`
6. **Push** to your fork: `git push origin feature/your-feature-name`
7. **Create** a Pull Request

## 📋 Development Guidelines

### Code Style
- Follow Rust standard formatting: `cargo fmt`
- Run clippy for linting: `cargo clippy`
- Use descriptive variable and function names
- Add documentation comments for public APIs

### Commit Messages
Use clear, descriptive commit messages:
```
type: Brief description of changes

- Detailed explanation of what was changed
- Why the change was necessary
- Any breaking changes or migration notes
```

**Types:**
- `Add:` New features or files
- `Fix:` Bug fixes
- `Update:` Improvements to existing code
- `Refactor:` Code restructuring without functional changes
- `Docs:` Documentation updates
- `Test:` Test additions or fixes

### Testing
- Write unit tests for new functions
- Test with different stamp series (L/M/N)
- Verify album output generation
- Test edge cases (missing images, invalid data)

## 🏗️ Architecture Overview

### Core Components
- **`album_globals.rs`**: Main album generation logic
- **`image_utils.rs`**: Image processing and embedded reference handling
- **`sample_images/`**: Series-specific scaling system
- **`main.rs`**: CLI interface and application entry point

### Key Design Principles
- **Embedded Assets**: No external file dependencies
- **Series-Specific Scaling**: Precise control per stamp type
- **Error Resilience**: Graceful handling of missing data
- **Performance Focus**: Optimized for large stamp collections

## 🔧 Adding New Features

### New Stamp Series Support
1. Add scaling factors to `sample_images/mod.rs`
2. Update series detection logic
3. Add reference images to appropriate subdirectories
4. Test with real stamp data

### Configuration Options
1. Add fields to configuration structures
2. Update `config.toml` parsing
3. Implement feature toggles
4. Document new options in README

### Database Schema Changes
1. Update database query logic
2. Maintain backward compatibility
3. Add migration documentation
4. Test with existing databases

## 🐛 Bug Reports

### Reporting Issues
Please include:
- **Version**: StampAlbumGen2 version and Rust version
- **Platform**: OS and architecture
- **Steps to reproduce**: Clear reproduction steps
- **Expected vs Actual**: What should happen vs what does happen
- **Logs**: Relevant error messages or debug output

### Debugging
Enable debug logging:
```bash
RUST_LOG=debug stamp_album_gen2 [options]
```

## 📚 Documentation

### Code Documentation
- Use `///` for public API documentation
- Include examples in doc comments
- Document parameters, return values, and error conditions
- Keep documentation up to date with code changes

### User Documentation
- Update README.md for new features
- Add examples for common use cases
- Document configuration options
- Include troubleshooting guides

## 🔍 Code Review Process

### Review Checklist
- [ ] Code compiles without warnings
- [ ] Tests pass
- [ ] Documentation is updated
- [ ] No breaking changes without migration path
- [ ] Performance impact assessed
- [ ] Security implications reviewed

### Review Comments
- Be constructive and specific
- Suggest improvements, don't just point out problems
- Consider the bigger picture and long-term maintainability
- Acknowledge good practices and clever solutions

## 🚨 Breaking Changes

### When to Introduce Breaking Changes
- Major version updates (v1.0 → v2.0)
- Significant API redesigns
- Removal of deprecated features
- Fundamental architecture changes

### Migration Guide
- Provide clear migration instructions
- Offer backward compatibility where possible
- Give advance notice in release notes
- Support old versions for a reasonable period

## 📊 Performance Considerations

### Profiling
```bash
# Use cargo flamegraph for performance analysis
cargo install flamegraph
cargo flamegraph --bin stamp_album_gen2
```

### Optimization Areas
- Image processing bottlenecks
- Database query performance
- Memory usage with large collections
- I/O operations for album generation

## 🤝 Community

### Communication
- Use GitHub Issues for bug reports and feature requests
- Use GitHub Discussions for general questions
- Be respectful and constructive in all interactions
- Help other contributors when possible

### Recognition
Contributors are recognized in:
- Git commit history
- CHANGELOG entries
- Release notes acknowledgments
- Project documentation

## 📄 License

By contributing to this project, you agree that your contributions will be licensed under the same MIT License that covers the project.

## 🙏 Acknowledgments

Thank you for contributing to StampAlbumGen2! Your efforts help make stamp album generation better for collectors worldwide.

---

**Last Updated**: September 11, 2025</content>
<parameter name="filePath">/media/ray/program/Rust/StampAlbumGen2/CONTRIBUTING.md
