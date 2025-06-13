const database = require('../server/utils/database');
const fs = require('fs');
const path = require('path');

async function setup() {
  try {
    console.log('🔧 Setting up Personal ATS Scanner...');
    
    // Create uploads directory if it doesn't exist
    const uploadsDir = path.join(__dirname, '../uploads');
    if (!fs.existsSync(uploadsDir)) {
      fs.mkdirSync(uploadsDir, { recursive: true });
      console.log('✅ Created uploads directory');
    }
    
    // Initialize database
    await database.initialize();
    console.log('✅ Database initialized successfully');
    
    console.log('🎉 Setup completed successfully!');
    console.log('\nNext steps:');
    console.log('1. Run: npm run dev');
    console.log('2. Open: http://localhost:3000');
    console.log('3. Make sure Ollama is running with required models');
    
    process.exit(0);
  } catch (error) {
    console.error('❌ Setup failed:', error.message);
    process.exit(1);
  }
}

setup();