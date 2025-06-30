const fs = require('fs');
const path = require('path');

const dbPath = path.join(__dirname, '..', 'data');

if (fs.existsSync(dbPath)) {
  console.log('✅ Database directory exists');
  process.exit(0);
} else {
  console.log('❌ Database directory not found');
  process.exit(1);
}
