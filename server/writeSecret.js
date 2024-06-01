import { mkdirSync, writeFileSync } from 'fs';
import os from 'os';
import path from 'path';

export const writeSecret = (type, secret) => {
  const filepath = path.join(os.homedir(), '.sprofile', `${type}.txt`);
  mkdirSync(path.dirname(filepath), { recursive: true });
  writeFileSync(filepath, secret, { mode: 0o600 })
}
