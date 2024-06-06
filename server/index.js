import 'dotenv/config.js'
import open from 'open'
import express from 'express';
import querystring from 'querystring';
import { writeSecret } from './writeSecret.js';
import { fetchAuthorizationCode, refreshAccessToken } from './token.js';

const app = express();

const REDIRECT_URI = 'http://localhost:8585/callback';
const AUTH_URL = 'https://accounts.spotify.com/authorize';
const CLIENT_ID = process.env.SPOTIFY_CLIENT_ID;
const PORT = 8585;

const genRandomStr = (length) => {
  const possible = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  let text = '';
  for (let i = 0; i < length; i++) {
    text += possible.charAt(Math.floor(Math.random() * possible.length));
  }
  return text;
}

function auth() {
  const server = app.listen(PORT, () => {
    console.log(`Local authentication server running on port ${PORT}`);
  })

  app.get('/login', (_, res) => {
    const state = genRandomStr(16);
    const scope = 'user-top-read user-read-recently-played playlist-read-private';

    res.redirect(`${AUTH_URL}?${querystring.stringify({
      response_type: 'code',
      client_id: CLIENT_ID,
      redirect_uri: REDIRECT_URI,
      scope,
      state,
    })
      }`)
  })

  app.get('/callback', async (req, res) => {
    const code = req.query.code ?? null;

    const response = await fetchAuthorizationCode(code);
    writeSecrets(response.data);

    res.status(200).send('Authorization successful. You can close this window.');
    server.close();
  });

  app.get('/refresh_token', async (req) => {
    const refresh_token = req.query.refresh_token;

    const response = await refreshAccessToken(refresh_token);

    writeSecrets(response.data);
  })
}

if (process.argv[2] == "refresh") {
  const refresh_token = process.argv[3];
  const response = await refreshAccessToken(refresh_token);
  console.log(response.data)
  writeSecrets(response.data);
} else {
  auth();
  open('http://localhost:8585/login');
}

/**
 * Helper functions
 * to reduce repetition
 */
export function writeSecrets(data) {
  console.log(data);
  const { access_token, refresh_token, expires_in } = data;
  writeSecret('access_token', access_token);
  refresh_token ?? writeSecret('refresh_token', refresh_token);
  writeSecret('expires_in', (Math.floor(Date.now() / 1000) + expires_in).toString());
}
