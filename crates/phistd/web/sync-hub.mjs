// Mirrors phinet/docs into the workspace hub (phidoc/docs/phinet) and rewrites
// site-absolute links so they resolve under the hub's /docs/phinet/ prefix.
// Run from phinet/web: `npm run sync-hub`. No dependencies.
import { cpSync, mkdirSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';

const SRC = resolve('../docs');
const DST = resolve('../../phidoc/docs/phinet');
const PUB = resolve('../../phidoc/public/phinet');

rmSync(DST, { recursive: true, force: true });
mkdirSync(DST, { recursive: true });
cpSync(SRC, DST, { recursive: true });
mkdirSync(PUB, { recursive: true });
for (const name of readdirSync(resolve('public'))) if (name.endsWith('.svg')) cpSync(resolve('public', name), join(PUB, name));
cpSync(resolve('public/phinet/3d'), join(PUB, '3d'), { recursive: true });

function walk(dir) {
  for (const name of readdirSync(dir)) {
    const p = join(dir, name);
    if (statSync(p).isDirectory()) walk(p);
    else if (/\.mdx?$/.test(name)) {
      let s = readFileSync(p, 'utf8');
      s = s.replace(/\]\(\/docs\/(?!phinet\/|phidoc\/)/g, '](/docs/phinet/');
      s = s.replace(/\]\(\/([\w-]+\.svg)\)/g, '](/phinet/$1)');
      writeFileSync(p, s);
    }
  }
}
walk(DST);
console.log(`synced ${SRC} -> ${DST}`);
