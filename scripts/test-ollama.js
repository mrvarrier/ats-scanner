const http = require('http');

const options = {
  hostname: 'localhost',
  port: 11434,
  path: '/api/tags',
  method: 'GET'
};

const req = http.request(options, (res) => {
  if (res.statusCode === 200) {
    console.log('✅ Ollama connection successful');
    process.exit(0);
  } else {
    console.log('❌ Ollama connection failed:', res.statusCode);
    process.exit(1);
  }
});

req.on('error', (err) => {
  console.log('❌ Ollama connection failed:', err.message);
  process.exit(1);
});

req.end();
