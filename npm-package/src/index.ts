#!/usr/bin/env node

import { execa } from 'execa';
import chalk from 'chalk';
import ora from 'ora';
import path from 'path';

// Get __dirname equivalent for ES modules
const __dirname = path.dirname(process.argv[1] || '.');

interface ForgeKitOptions {
  cwd?: string;
  verbose?: boolean;
}

class ForgeKit {
  private options: ForgeKitOptions;
  private spinner: ora.Ora;

  constructor(options: ForgeKitOptions = {}) {
    this.options = {
      cwd: process.cwd(),
      verbose: false,
      ...options
    };
    this.spinner = ora();
  }

  /**
   * Create a new .mox application
   */
  async new(name: string, options: { path?: string; template?: string } = {}): Promise<void> {
    const projectPath = options.path || name;
    const template = options.template || 'basic';
    
    this.spinner.start(`Creating new ForgeKit project: ${name}`);
    
    try {
      const args = ['new', name];
      if (options.path) args.push('--path', options.path);
      if (options.template) args.push('--template', options.template);
      
      await this.executeForgeKit(args);
      this.spinner.succeed(chalk.green(`Project '${name}' created successfully!`));
      
      console.log('\nNext steps:');
      console.log(chalk.blue(`  cd ${projectPath}`));
      console.log(chalk.blue('  forgekit build'));
    } catch (error) {
      this.spinner.fail(chalk.red('Failed to create project'));
      throw error;
    }
  }

  /**
   * Build the current project
   */
  async build(options: { path?: string } = {}): Promise<void> {
    this.spinner.start('Building project');
    
    try {
      const args = ['build'];
      if (options.path) args.push('--path', options.path);
      
      await this.executeForgeKit(args);
      this.spinner.succeed(chalk.green('Build completed successfully!'));
    } catch (error) {
      this.spinner.fail(chalk.red('Build failed'));
      throw error;
    }
  }

  /**
   * Package the project into a .mox file
   */
  async package(options: { path?: string } = {}): Promise<string> {
    this.spinner.start('Packaging project');
    
    try {
      const args = ['package'];
      if (options.path) args.push('--path', options.path);
      
      const result = await this.executeForgeKit(args);
      this.spinner.succeed(chalk.green('Package created successfully!'));
      
      // Extract package path from output
      const packagePath = this.extractPackagePath(result.stdout);
      return packagePath;
    } catch (error) {
      this.spinner.fail(chalk.red('Packaging failed'));
      throw error;
    }
  }

  /**
   * Build and package the project
   */
  async buildPackage(options: { path?: string } = {}): Promise<string> {
    this.spinner.start('Building and packaging project');
    
    try {
      const args = ['build-package'];
      if (options.path) args.push('--path', options.path);
      
      const result = await this.executeForgeKit(args);
      this.spinner.succeed(chalk.green('Build and packaging completed!'));
      
      const packagePath = this.extractPackagePath(result.stdout);
      return packagePath;
    } catch (error) {
      this.spinner.fail(chalk.red('Build and packaging failed'));
      throw error;
    }
  }

  /**
   * Run the project locally
   */
  async run(options: { path?: string } = {}): Promise<void> {
    this.spinner.start('Running project');
    
    try {
      const args = ['run'];
      if (options.path) args.push('--path', options.path);
      
      await this.executeForgeKit(args);
      this.spinner.succeed(chalk.green('Project executed successfully!'));
    } catch (error) {
      this.spinner.fail(chalk.red('Failed to run project'));
      throw error;
    }
  }

  /**
   * Add a dependency to the project
   */
  async add(packageName: string, options: { version?: string; path?: string } = {}): Promise<void> {
    this.spinner.start(`Adding dependency: ${packageName}`);
    
    try {
      const args = ['add', packageName];
      if (options.version) args.push('--version', options.version);
      if (options.path) args.push('--path', options.path);
      
      await this.executeForgeKit(args);
      this.spinner.succeed(chalk.green(`Dependency '${packageName}' added successfully!`));
    } catch (error) {
      this.spinner.fail(chalk.red(`Failed to add dependency '${packageName}'`));
      throw error;
    }
  }

  /**
   * Remove a dependency from the project
   */
  async remove(packageName: string, options: { path?: string } = {}): Promise<void> {
    this.spinner.start(`Removing dependency: ${packageName}`);
    
    try {
      const args = ['remove', packageName];
      if (options.path) args.push('--path', options.path);
      
      await this.executeForgeKit(args);
      this.spinner.succeed(chalk.green(`Dependency '${packageName}' removed successfully!`));
    } catch (error) {
      this.spinner.fail(chalk.red(`Failed to remove dependency '${packageName}'`));
      throw error;
    }
  }

  /**
   * Update project dependencies
   */
  async update(options: { path?: string } = {}): Promise<void> {
    this.spinner.start('Updating dependencies');
    
    try {
      const args = ['update'];
      if (options.path) args.push('--path', options.path);
      
      await this.executeForgeKit(args);
      this.spinner.succeed(chalk.green('Dependencies updated successfully!'));
    } catch (error) {
      this.spinner.fail(chalk.red('Failed to update dependencies'));
      throw error;
    }
  }

  /**
   * Search for available packages
   */
  async search(query: string): Promise<string[]> {
    try {
      const args = ['search', query];
      const result = await this.executeForgeKit(args);
      
      // Parse search results
      const packages = this.parseSearchResults(result.stdout);
      return packages;
    } catch (error) {
      throw new Error(`Failed to search for packages: ${error}`);
    }
  }

  /**
   * List available templates
   */
  async templates(): Promise<string[]> {
    try {
      const args = ['templates'];
      const result = await this.executeForgeKit(args);
      
      // Parse template list
      const templates = this.parseTemplateList(result.stdout);
      return templates;
    } catch (error) {
      throw new Error(`Failed to list templates: ${error}`);
    }
  }

  /**
   * Execute forgekit command
   */
  private async executeForgeKit(args: string[]): Promise<{ stdout: string; stderr: string }> {
    try {
      const result = await execa('forgekit', args, {
        cwd: this.options.cwd,
        reject: false
      });

      if (this.options.verbose) {
        console.log(chalk.gray(`[ForgeKit] Executed: forgekit ${args.join(' ')}`));
        if (result.stdout) console.log(chalk.gray(`[STDOUT] ${result.stdout}`));
        if (result.stderr) console.log(chalk.gray(`[STDERR] ${result.stderr}`));
      }

      if (result.exitCode !== 0) {
        throw new Error(`ForgeKit command failed: ${result.stderr || result.stdout}`);
      }

      return {
        stdout: result.stdout,
        stderr: result.stderr
      };
    } catch (error) {
      if (error instanceof Error && error.message.includes('spawn forgekit ENOENT')) {
        throw new Error('ForgeKit CLI not found. Please install ForgeKit first: cargo install forgekit');
      }
      throw error;
    }
  }

  /**
   * Extract package path from forgekit output
   */
  private extractPackagePath(output: string): string {
    // Look for package path in output
    const match = output.match(/Package created at ([^\s]+)/);
    if (match) {
      return match[1];
    }
    throw new Error('Could not extract package path from output');
  }

  /**
   * Parse search results
   */
  private parseSearchResults(output: string): string[] {
    const lines = output.split('\n');
    const packages: string[] = [];
    
    for (const line of lines) {
      if (line.trim().startsWith('- ') || line.includes(' - ')) {
        packages.push(line.trim().replace('- ', ''));
      }
    }
    
    return packages;
  }

  /**
   * Parse template list
   */
  private parseTemplateList(output: string): string[] {
    const lines = output.split('\n');
    const templates: string[] = [];
    
    for (const line of lines) {
      const match = line.match(/^\s*(\w+)\s+-\s+(.+)$/);
      if (match) {
        templates.push(`${match[1]} - ${match[2]}`);
      }
    }
    
    return templates;
  }
}

// Export the main class
export default ForgeKit;

// If run directly, execute CLI commands
if (import.meta.url === `file://${process.argv[1]}`) {
  const [, , command, ...args] = process.argv;
  
  const forgekit = new ForgeKit({
    verbose: process.env.FORGEKIT_VERBOSE === 'true'
  });

  switch (command) {
    case 'new':
      forgekit.new(args[0], { path: args[1], template: args[2] }).catch(console.error);
      break;
    case 'build':
      forgekit.build({ path: args[0] }).catch(console.error);
      break;
    case 'package':
      forgekit.package({ path: args[0] }).catch(console.error);
      break;
    case 'build-package':
      forgekit.buildPackage({ path: args[0] }).catch(console.error);
      break;
    case 'run':
      forgekit.run({ path: args[0] }).catch(console.error);
      break;
    case 'add':
      forgekit.add(args[0], { version: args[1], path: args[2] }).catch(console.error);
      break;
    case 'remove':
      forgekit.remove(args[0], { path: args[1] }).catch(console.error);
      break;
    case 'update':
      forgekit.update({ path: args[0] }).catch(console.error);
      break;
    case 'search':
      forgekit.search(args[0]).then(results => {
        console.log('Found packages:');
        results.forEach(pkg => console.log(`  - ${pkg}`));
      }).catch(console.error);
      break;
    case 'templates':
      forgekit.templates().then(templates => {
        console.log('Available templates:');
        templates.forEach(template => console.log(`  ${template}`));
      }).catch(console.error);
      break;
    default:
      console.log('ForgeKit CLI for Node.js');
      console.log('Usage: forgekit <command> [options]');
      console.log('\nCommands:');
      console.log('  new <name> [path] [template]    Create a new project');
      console.log('  build [path]                    Build the project');
      console.log('  package [path]                  Package the project');
      console.log('  build-package [path]            Build and package');
      console.log('  run [path]                      Run the project');
      console.log('  add <package> [version] [path]  Add dependency');
      console.log('  remove <package> [path]         Remove dependency');
      console.log('  update [path]                   Update dependencies');
      console.log('  search <query>                  Search packages');
      console.log('  templates                       List templates');
  }
}