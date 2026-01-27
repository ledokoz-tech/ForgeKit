/**
 * Example usage of ForgeKit npm package
 */

const ForgeKit = require('./src/index.js');

async function main() {
  try {
    // Create ForgeKit instance
    const forgekit = new ForgeKit({
      // You can specify custom forgekit path if needed
      // forgekitPath: '/path/to/forgekit'
    });

    console.log('🔧 ForgeKit npm package example');

    // List available templates
    console.log('\n📋 Available templates:');
    const templates = await forgekit.templates();
    templates.forEach(template => console.log(`  • ${template}`));

    // Search for packages
    console.log('\n🔍 Searching for "http" packages:');
    const packages = await forgekit.search('http');
    packages.slice(0, 5).forEach(pkg => console.log(`  • ${pkg}`));

    // Example of creating a new project (commented out to prevent actual creation)
    /*
    console.log('\n🏗️  Creating new project...');
    await forgekit.new('example-app', { 
      template: 'cli',
      path: './example-app' 
    });
    console.log('✅ Project created successfully!');
    */

    console.log('\n✨ Example completed successfully!');

  } catch (error) {
    console.error('❌ Error:', error.message);
    
    if (error.message.includes('ForgeKit CLI not found')) {
      console.log('\n💡 To fix this issue:');
      console.log('   1. Install ForgeKit CLI: cargo install forgekit');
      console.log('   2. Or set FORGEKIT_PATH environment variable');
    }
  }
}

// Run the example
if (require.main === module) {
  main();
}

module.exports = main;