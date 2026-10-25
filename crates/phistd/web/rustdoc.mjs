// Builds the crate's rustdoc and places it under public/rustdoc so the docs
// site serves it at /rustdoc/phinet/. Run from phinet/web: `npm run rustdoc`.
import { execSync } from 'node:child_process';
import { cpSync, rmSync } from 'node:fs';
import { resolve } from 'node:path';

execSync('cargo doc --no-deps --release', { cwd: resolve('..'), stdio: 'inherit' });
const dst = resolve('public/rustdoc');
rmSync(dst, { recursive: true, force: true });
cpSync(resolve('../target/doc'), dst, { recursive: true });
console.log(`rustdoc -> ${dst} (served at /rustdoc/phinet/)`);
