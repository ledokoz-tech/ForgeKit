# Contributing to ForgeKit npm Package

Thank you for your interest in contributing to the ForgeKit npm package! This document provides guidelines for contributing.

## 🎯 Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/ForgeKit.git`
3. Navigate to npm package: `cd ForgeKit/npm-package`
4. Install dependencies: `npm install`
5. Create a new branch: `git checkout -b feature/your-feature-name`

## 📦 Development Setup

### Prerequisites

- Node.js >= 14.0.0
- npm or yarn
- ForgeKit CLI installed (`cargo install forgekit`)

### Development Commands

```bash
# Install dependencies
npm install

# Run tests
npm test

# Run linter
npm run lint

# Build the package
npm run build

# Test the CLI locally
node bin/forgekit.js --help
```

## 🛠 Code Structure

```
npm-package/
├── bin/              # CLI executable scripts
│   └── forgekit.js   # Main CLI wrapper
├── src/              # Source code
│   └── index.js      # Main library code
├── __tests__/        # Test files
│   └── forgekit.test.js
├── dist/             # Compiled output (generated)
├── package.json      # Package configuration
├── README.md         # Documentation
└── tsconfig.json     # TypeScript configuration
```

## 📝 Coding Standards

### JavaScript Style

- Use ES6+ features
- Follow Airbnb JavaScript style guide
- Use async/await instead of callbacks
- Write JSDoc comments for public methods

### Error Handling

- Always handle errors gracefully
- Provide meaningful error messages
- Use proper error codes when applicable

### Testing

- Write tests for new functionality
- Ensure all tests pass before submitting PR
- Test both success and failure cases

## 🚀 Submitting Changes

1. **Test your changes** thoroughly
2. **Update documentation** if needed
3. **Follow commit conventions**:
   - `feat:` for new features
   - `fix:` for bug fixes
   - `docs:` for documentation changes
   - `test:` for test additions/modifications
   - `refactor:` for code refactoring

4. **Push to your fork** and submit a Pull Request

## 🐛 Reporting Issues

When reporting issues, please include:

- Clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Environment details (Node.js version, OS, etc.)
- Any relevant error messages

## 🤝 Community Guidelines

- Be respectful and constructive
- Help others learn and grow
- Follow the project's code of conduct
- Welcome newcomers to the community

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

## 🆘 Need Help?

- Check existing issues and documentation
- Join our Discord community (link in README)
- Contact the maintainers directly

Thank you for contributing to ForgeKit! 🚀