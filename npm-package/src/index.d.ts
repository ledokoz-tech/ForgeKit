declare class ForgeKit {
  constructor(options?: {
    cwd?: string;
    forgekitPath?: string;
  });

  execute(args: string[]): Promise<{ stdout: string; stderr: string; code: number }>;
  
  new(name: string, options?: { path?: string; template?: string }): Promise<any>;
  build(options?: { path?: string }): Promise<any>;
  package(options?: { path?: string }): Promise<any>;
  buildPackage(options?: { path?: string }): Promise<any>;
  run(options?: { path?: string }): Promise<any>;
  add(packageName: string, options?: { version?: string; path?: string }): Promise<any>;
  remove(packageName: string, options?: { path?: string }): Promise<any>;
  update(options?: { path?: string }): Promise<any>;
  search(query: string): Promise<string[]>;
  templates(): Promise<string[]>;
}

export default ForgeKit;
export { ForgeKit };