import fs from 'fs'
import path from 'path'
import { fileURLToPath } from 'url'
import db from 'mime-db'
import mime from 'mime-types'

const extSet = new Set()

for (const [k, v] of Object.entries(db)) {
  if (v.hasOwnProperty('extensions')) {
    v.extensions.map(ext => extSet.add(ext))
  }
}

const map = {}

for (const ext of extSet) {
  map[ext]= mime.lookup(ext)
}
const srcDir = path.join(path.dirname(fileURLToPath(import.meta.url)), '..', 'src')
// fs.writeFileSync(path.join(srcDir, 'ext-mime.json'), JSON.stringify(map))
const m = Object.entries(map)
  .map(([ext, v]) => `("${ext}","${v}")`)
  .join(',\n')
const rs = `
use std::collections::HashMap;
use once_cell::sync::Lazy;
pub static EXT_MIME_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    [${m}].into()
});
`
fs.writeFileSync(path.join(srcDir, 'ext_mime.rs'), rs)
