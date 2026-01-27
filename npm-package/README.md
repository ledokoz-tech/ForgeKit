# @ledokoz/forgekit

Node.js wrapper for [ForgeKit](https://github.com/ledokoz-tech/ForgeKit) - A modern Rust framework for building `.mox` applications for Ledokoz OS.

[![npm version](https://img.shields.io/npm/v/@ledokoz/forgekit)](https://www.npmjs.com/package/@ledokoz/forgekit)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Build Status](https://img.shields.io/github/workflow/status/ledokoz-tech/ForgeKit/CI)](https://github.com/ledokoz-tech/ForgeKit/actions)

## 🚀 Features

- **Full CLI Compatibility**: Complete wrapper for all ForgeKit CLI commands
- **Programmatic API**: JavaScript/Node.js API for integrating ForgeKit into your tools
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Easy Installation**: Simple npm install with automatic ForgeKit CLI detection
- **TypeScript Support**: Full TypeScript definitions included

## 📦 Installation

First, install the npm package:

```bash
npm install @ledokoz/forgekit
```

You'll also need the ForgeKit CLI installed:

```bash
# Install ForgeKit CLI via Cargo
cargo install forgekit
```

## 🛠 Usage

### CLI Usage

After installation, you can use forgekit commands directly:

```bash
# Create a new project
npx forgekit new myapp

# Build a project
npx forgekit build

# Package a project
npx forgekit package

# Add a dependency
npx forgekit add serde --version "1.0"
```

### Programmatic Usage

```javascript
const ForgeKit = require('@ledokoz/forgekit');

// Create a ForgeKit instance
const forgekit = new ForgeKit();

// Create a new project
await forgekit.new('myapp', { template: 'gui' });

// Build the project
await forgekit.build({ path: './myapp' });

// Add a dependency
await forgekit.add('serde', { version: '1.0', path: './myapp' });

// Search for packages
const packages = await forgekit.search('http');
console.log('Found packages:', packages);

// List available templates
const templates = await forgekit.templates();
console.log('Available templates:', templates);
```

### TypeScript Usage

```typescript
import ForgeKit from '@ledokoz/forgekit';

const forgekit = new ForgeKit();

// All methods return promises
await forgekit.new('my-typescript-app', { template: 'cli' });
```

## 📚 API Reference

### Constructor

```javascript
const forgekit = new ForgeKit({
  cwd: process.cwd(),           // Working directory
  forgekitPath: 'forgekit'      // Path to forgekit binary
});
```

### Methods

All methods return `Promise<{ stdout: string, stderr: string, code: number }>`:

- `new(name, options)` - Create a new project
- `build(options)` - Build the project
- `package(options)` - Package the project
- `buildPackage(options)` - Build and package
- `run(options)` - Run the project
- `add(packageName, options)` - Add dependency
- `remove(packageName, options)` - Remove dependency
- `update(options)` - Update dependencies
- `search(query)` - Search packages (returns array of strings)
- `templates()` - List templates (returns array of strings)

### Options

Common options for most methods:
- `path` - Project directory path
- `template` - Template type (new command)
- `version` - Package version (add command)

## 🎯 Commands

| Command | Description | Example |
|---------|-------------|---------|
| `new <name>` | Create new project | `forgekit new myapp` |
| `build` | Build project | `forgekit build` |
| `package` | Create .mox package | `forgekit package` |
| `build-package` | Build and package | `forgekit build-package` |
| `run` | Run project locally | `forgekit run` |
| `add <package>` | Add dependency | `forgekit add serde` |
| `remove <package>` | Remove dependency | `forgekit remove serde` |
| `update` | Update dependencies | `forgekit update` |
| `search <query>` | Search packages | `forgekit search http` |
| `templates` | List templates | `forgekit templates` |

## ⚙️ Configuration

### Environment Variables

- `FORGEKIT_PATH` - Path to the forgekit binary (if not in PATH)
- `FORGEKIT_VERBOSE` - Enable verbose output

### Custom Binary Path

```javascript
const forgekit = new ForgeKit({
  forgekitPath: '/custom/path/to/forgekit'
});
```

## 🤝 Requirements

- Node.js >= 14.0.0
- ForgeKit CLI installed (via `cargo install forgekit`)
- Rust toolchain for building projects

## 📖 Documentation

For detailed documentation about ForgeKit and `.mox` applications, visit:
- [ForgeKit Documentation](https://ledokoz.com/forgekit/docs)
- [Ledokoz OS](https://os.ledokoz.com)

## 🐛 Troubleshooting

### "ForgeKit CLI not found" Error

Make sure ForgeKit CLI is installed:

```bash
cargo install forgekit
```

Or set the path manually:

```bash
export FORGEKIT_PATH=/path/to/forgekit
```

### Permission Issues

On Unix systems, you might need to make the binary executable:

```bash
chmod +x $(which forgekit)
```

## 🤝 Contributing

Contributions are welcome! Please see the [contributing guidelines](CONTRIBUTING.md).

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [GitHub Repository](https://github.com/ledokoz-tech/ForgeKit)
- [ForgeKit Documentation](https://ledokoz.com/forgekit/docs)
- [Ledokoz Main Site](https://ledokoz.com)
- [Ledokoz OS](https://os.ledokoz.com)