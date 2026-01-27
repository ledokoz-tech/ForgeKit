#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');

// Simple wrapper that forwards commands to the Rust forgekit CLI
function runForgeKit(args) {
  const forgekitPath = process.env.FORGEKIT_PATH || 'forgekit';
  
  const child = spawn(forgekitPath, args, {
    stdio: 'inherit',
    cwd: process.cwd()
  });

  child.on('error', (err) => {
    if (err.code === 'ENOENT') {
      console.error('Error: ForgeKit CLI not found.');
      console.error('Please install ForgeKit first:');
      console.error('  cargo install forgekit');
      console.error('');
      console.error('Or set FORGEKIT_PATH environment variable to the forgekit binary location.');
    } else {
      console.error('Error running forgekit:', err.message);
    }
    process.exit(1);
  });

  child.on('close', (code) => {
    process.exit(code);
  });
}

// Parse command line arguments
const [, , command, ...args] = process.argv;

// Help text
if (!command || command === 'help' || command === '--help') {
  console.log('ForgeKit CLI for Node.js');
  console.log('Usage: forgekit <command> [options]');
  console.log('');
  console.log('Commands:');
  console.log('  new <name> [options]        Create a new project');
  console.log('  build [options]             Build the project');
  console.log('  package [options]           Package the project');
  console.log('  build-package [options]     Build and package');
  console.log('  run [options]               Run the project');
  console.log('  add <package> [options]     Add dependency');
  console.log('  remove <package> [options]  Remove dependency');
  console.log('  update [options]            Update dependencies');
  console.log('  search <query>              Search packages');
  console.log('  templates                   List templates');
  console.log('');
  console.log('Options:');
  console.log('  --path, -p <path>          Specify project path');
  console.log('  --template, -t <template>  Specify template type');
  console.log('  --version, -v <version>    Specify version');
  console.log('');
  console.log('Examples:');
  console.log('  forgekit new myapp');
  console.log('  forgekit new myapp --template gui');
  console.log('  forgekit build --path ./myapp');
  console.log('  forgekit add serde --version "1.0"');
  process.exit(0);
}

// Version command
if (command === 'version' || command === '--version') {
  console.log('ForgeKit CLI for Node.js v0.1.0');
  console.log('Wrapper for ForgeKit Rust CLI');
  process.exit(0);
}

// Forward all commands to the Rust forgekit CLI
runForgeKit([command, ...args]);