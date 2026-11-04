const http = require('http');
const fs = require('fs');
const path = require('path');
const os = require('os');
const { exec, spawn } = require('child_process');

const PORT = 49201;
const DIST_DIR = path.join(__dirname, 'frontend', 'dist');
const HOME_DIR = os.homedir();
const CONFIG_DIR = path.join(os.homedir(), 'AppData', 'Roaming', 'GitSwitch');

// Ensure config dir exists
try {
  if (!fs.existsSync(CONFIG_DIR)) {
    fs.mkdirSync(CONFIG_DIR, { recursive: true });
  }
} catch (e) {}

const CONFIG_FILE = path.join(CONFIG_DIR, 'config.json');

const server = http.createServer((req, res) => {
  // Enable CORS
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'GET, POST, OPTIONS');
  res.setHeader('Access-Control-Allow-Headers', 'Content-Type');

  if (req.method === 'OPTIONS') {
    res.writeHead(200);
    res.end();
    return;
  }

  const url = new URL(req.url, `http://${req.headers.host}`);

  // Static File Serving
  let filePath = path.join(DIST_DIR, url.pathname === '/' ? 'index.html' : url.pathname);
  if (!fs.existsSync(filePath)) {
    filePath = path.join(DIST_DIR, 'index.html');
  }

  fs.readFile(filePath, (err, data) => {
    if (err) {
      res.writeHead(404, { 'Content-Type': 'text/plain' });
      res.end('Not Found');
      return;
    }

    let contentType = 'text/html';
    if (filePath.endsWith('.js')) contentType = 'application/javascript';
    else if (filePath.endsWith('.css')) contentType = 'text/css';
    else if (filePath.endsWith('.json')) contentType = 'application/json';
    else if (filePath.endsWith('.svg')) contentType = 'image/svg+xml';

    res.writeHead(200, { 'Content-Type': contentType });
    res.end(data);
  });
});

server.listen(PORT, '127.0.0.1', () => {
  console.log(`\n======================================================`);
  console.log(`  ⚡ GitSwitch v1.8.0 - Desktop Daemon Active`);
  console.log(`  Local Endpoint: http://127.0.0.1:${PORT}`);
  console.log(`  Host Config:    ~/.gitconfig | ~/.ssh/config`);
  console.log(`======================================================\n`);

  // Launch Desktop Window via Edge App mode
  const edgePath = "C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe";
  const userDataDir = path.join(os.tmpdir(), 'gitswitch-webview-profile');

  if (fs.existsSync(edgePath)) {
    spawn(edgePath, [
      `--app=http://127.0.0.1:${PORT}`,
      '--window-size=1200,860',
      `--user-data-dir=${userDataDir}`
    ], { detached: true, stdio: 'ignore' }).unref();
  } else {
    // Fallback to start
    exec(`start http://127.0.0.1:${PORT}`);
  }
});
