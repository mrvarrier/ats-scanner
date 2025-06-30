const fs = require('fs');
const path = require('path');

const dataDir = path.join(__dirname, '..', 'data');

if (!fs.existsSync(dataDir)) {
  fs.mkdirSync(dataDir, { recursive: true });
  console.log('✅ Database directory created');
} else {
  console.log('✅ Database directory already exists');
}

console.log('Database will be initialized automatically when the application starts');
