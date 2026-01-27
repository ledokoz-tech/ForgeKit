const { spawn } = require('child_process');
const { promisify } = require('util');

/**
 * ForgeKit Node.js Wrapper
 * Provides programmatic access to ForgeKit CLI functionality
 */
class ForgeKit {
  constructor(options = {}) {
    this.options = {
      cwd: process.cwd(),
      forgekitPath: process.env.FORGEKIT_PATH || 'forgekit',
      ...options
    };
  }

  /**
   * Execute a forgekit command
   */
  async execute(args) {
    return new Promise((resolve, reject) => {
      const child = spawn(this.options.forgekitPath, args, {
        cwd: this.options.cwd,
        stdio: ['pipe', 'pipe', 'pipe']
      });

      let stdout = '';
      let stderr = '';

      child.stdout.on('data', (data) => {
        stdout += data.toString();
      });

      child.stderr.on('data', (data) => {
        stderr += data.toString();
      });

      child.on('error', (err) => {
        if (err.code === 'ENOENT') {
          reject(new Error('ForgeKit CLI not found. Please install ForgeKit: cargo install forgekit'));
        } else {
          reject(err);
        }
      });

      child.on('close', (code) => {
        if (code === 0) {
          resolve({ stdout, stderr, code });
        } else {
          reject(new Error(`ForgeKit command failed with exit code ${code}: ${stderr || stdout}`));
        }
      });
    });
  }

  /**
   * Create a new .mox application
   */
  async new(name, options = {}) {
    const args = ['new', name];
    if (options.path) args.push('--path', options.path);
    if (options.template) args.push('--template', options.template);
    
    return this.execute(args);
  }

  /**
   * Build the current project
   */
  async build(options = {}) {
    const args = ['build'];
    if (options.path) args.push('--path', options.path);
    
    return this.execute(args);
  }

  /**
   * Package the project into a .mox file
   */
  async package(options = {}) {
    const args = ['package'];
    if (options.path) args.push('--path', options.path);
    
    return this.execute(args);
  }

  /**
   * Build and package the project
   */
  async buildPackage(options = {}) {
    const args = ['build-package'];
    if (options.path) args.push('--path', options.path);
    
    return this.execute(args);
  }

  /**
   * Run the project locally
   */
  async run(options = {}) {
    const args = ['run'];
    if (options.path) args.push('--path', options.path);
    
    return this.execute(args);
  }

  /**
   * Add a dependency to the project
   */
  async add(packageName, options = {}) {
    const args = ['add', packageName];
    if (options.version) args.push('--version', options.version);
    if (options.path) args.push('--path', options.path);
    
    return this.execute(args);
  }

  /**
   * Remove a dependency from the project
   */
  async remove(packageName, options = {}) {
    const args = ['remove', packageName];
    if (options.path) args.push('--path', options.path);
    
    return this.execute(args);
  }

  /**
   * Update project dependencies
   */
  async update(options = {}) {
    const args = ['update'];
    if (options.path) args.push('--path', options.path);
    
    return this.execute(args);
  }

  /**
   * Search for available packages
   */
  async search(query) {
    const result = await this.execute(['search', query]);
    // Parse search results from stdout
    const lines = result.stdout.split('\n');
    const packages = lines
      .filter(line => line.trim().startsWith('- ') || line.includes(' - '))
      .map(line => line.trim().replace('- ', ''));
    return packages;
  }

  /**
   * List available templates
   */
  async templates() {
    const result = await this.execute(['templates']);
    // Parse template list from stdout
    const lines = result.stdout.split('\n');
    const templates = lines
      .map(line => {
        const match = line.match(/^\s*(\w+)\s+-\s+(.+)$/);
        return match ? `${match[1]} - ${match[2]}` : null;
      })
      .filter(Boolean);
    return templates;
  }
}

// Export the class
module.exports = ForgeKit;

// Also export a default instance for convenience
module.exports.forgekit = new ForgeKit();