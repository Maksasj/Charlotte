import { defineConfig, type Plugin } from 'vite';
import react from '@vitejs/plugin-react';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import path from 'node:path';

const VAULT_DIR = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../vault');

/**
 * Dev-only, read-only view of the vault's Markdown files for the UI's source view:
 *   GET /vault-src/core/persona/identity.md
 * Only `.md` files inside `vault/` are served.
 */
function vaultSource(): Plugin {
  return {
    name: 'karin-vault-source',
    configureServer(server) {
      server.middlewares.use('/vault-src', async (req, res) => {
        const rel = decodeURIComponent((req.url ?? '').split('?')[0]).replace(/^\/+/, '');
        const file = path.resolve(VAULT_DIR, rel);
        if (!file.startsWith(VAULT_DIR + path.sep) || path.extname(file) !== '.md') {
          res.statusCode = 404;
          res.end();
          return;
        }
        try {
          const text = await readFile(file, 'utf8');
          res.setHeader('Content-Type', 'text/plain; charset=utf-8');
          res.setHeader('Cache-Control', 'no-cache');
          res.end(text);
        } catch {
          res.statusCode = 404;
          res.end();
        }
      });
    },
  };
}

export default defineConfig({
  plugins: [react(), vaultSource()],
  server: {
    port: 5173,
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:8080',
        changeOrigin: true,
      },
    },
  },
});
