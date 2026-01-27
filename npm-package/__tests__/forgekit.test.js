const ForgeKit = require('../src/index.js');

// Mock child_process.spawn for testing
jest.mock('child_process', () => ({
  spawn: jest.fn()
}));

const { spawn } = require('child_process');

describe('ForgeKit', () => {
  let forgekit;
  let mockChildProcess;

  beforeEach(() => {
    forgekit = new ForgeKit({
      forgekitPath: 'forgekit-test'
    });
    
    // Mock child process events
    mockChildProcess = {
      on: jest.fn((event, callback) => {
        if (event === 'close') {
          // Simulate successful completion
          setTimeout(() => callback(0), 10);
        }
        return mockChildProcess;
      }),
      stdout: {
        on: jest.fn()
      },
      stderr: {
        on: jest.fn()
      }
    };
    
    spawn.mockReturnValue(mockChildProcess);
  });

  afterEach(() => {
    jest.clearAllMocks();
  });

  test('should create ForgeKit instance', () => {
    expect(forgekit).toBeInstanceOf(ForgeKit);
    expect(forgekit.options.cwd).toBe(process.cwd());
    expect(forgekit.options.forgekitPath).toBe('forgekit-test');
  });

  test('should have all required methods', () => {
    expect(typeof forgekit.new).toBe('function');
    expect(typeof forgekit.build).toBe('function');
    expect(typeof forgekit.package).toBe('function');
    expect(typeof forgekit.buildPackage).toBe('function');
    expect(typeof forgekit.run).toBe('function');
    expect(typeof forgekit.add).toBe('function');
    expect(typeof forgekit.remove).toBe('function');
    expect(typeof forgekit.update).toBe('function');
    expect(typeof forgekit.search).toBe('function');
    expect(typeof forgekit.templates).toBe('function');
  });

  test('should call spawn with correct arguments for new command', async () => {
    await forgekit.new('test-project', { template: 'cli' });
    
    expect(spawn).toHaveBeenCalledWith(
      'forgekit-test',
      ['new', 'test-project', '--template', 'cli'],
      expect.objectContaining({
        cwd: process.cwd(),
        stdio: ['pipe', 'pipe', 'pipe']
      })
    );
  });

  test('should handle missing forgekit gracefully', async () => {
    const errorMock = new Error('spawn ENOENT');
    errorMock.code = 'ENOENT';
    
    mockChildProcess.on = jest.fn((event, callback) => {
      if (event === 'error') {
        setTimeout(() => callback(errorMock), 10);
      }
      return mockChildProcess;
    });
    
    await expect(forgekit.new('test')).rejects.toThrow('ForgeKit CLI not found');
  });
});