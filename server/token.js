import axios from "axios";
import fs from 'fs';
import os from 'os';
import path from 'path';

const REDIRECT_URI = 'http://localhost:8585/callback';
const TOKEN_URL = 'https://accounts.spotify.com/api/token';
const CLIENT_ID = fs.readFileSync(path.join(os.homedir(), '.sprofile', 'client_id.txt'), { encoding: 'utf-8' }).trim();
const CLIENT_SECRET = fs.readFileSync(path.join(os.homedir(), '.sprofile', 'client_secret.txt'), { encoding: 'utf-8' }).trim();
const headers = {
  'content-type': 'application/x-www-form-urlencoded',
  'Authorization': `Basic ${Buffer.from(`${CLIENT_ID}:${CLIENT_SECRET}`).toString('base64')}`
}

export const fetchAuthorizationCode = async (code) => {
  return (await axios.post(
    TOKEN_URL,
    {
      grant_type: 'authorization_code',
      redirect_uri: REDIRECT_URI,
      code,
    },
    { headers }
  ))
}

export const refreshAccessToken = async (refresh_token) => {
  return (await axios.post(
    TOKEN_URL,
    {
      grant_type: 'refresh_token',
      refresh_token,
    },
    { headers }
  ))
}
