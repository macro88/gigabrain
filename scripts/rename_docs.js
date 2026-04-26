#!/usr/bin/env node
'use strict';

const fs = require('fs');

function replaceAll(str, from, to) {
  return str.split(from).join(to);
}

function processFile(path, replacements) {
  let content = fs.readFileSync(path, 'utf8');
  for (const [from, to] of replacements) {
    content = replaceAll(content, from, to);
  }
  fs.writeFileSync(path, content, 'utf8');
  return content;
}

// Common replacements applied to most docs files (order matters)
const commonReplacements = [
  // Product name
  ['GigaBrain', 'Quaid'],
  // CLI binary
  ['gbrain ', 'quaid '],
  ['`gbrain`', '`quaid`'],
  ['"gbrain"', '"quaid"'],
  ['gbrain\n', 'quaid\n'],
  ['gigabrain', 'quaid'],  // URLs like macro88/gigabrain
  // Env vars
  ['GBRAIN_DB', 'QUAID_DB'],
  ['GBRAIN_MODEL', 'QUAID_MODEL'],
  ['GBRAIN_CHANNEL', 'QUAID_CHANNEL'],
  ['GBRAIN_NO_PROFILE', 'QUAID_NO_PROFILE'],
  ['GBRAIN_WATCH_DEBOUNCE_MS', 'QUAID_WATCH_DEBOUNCE_MS'],
  ['GBRAIN_QUARANTINE_TTL_DAYS', 'QUAID_QUARANTINE_TTL_DAYS'],
  ['GBRAIN_RAW_IMPORTS_KEEP_ALL', 'QUAID_RAW_IMPORTS_KEEP_ALL'],
  ['GBRAIN_RAW_IMPORTS_KEEP', 'QUAID_RAW_IMPORTS_KEEP'],
  ['GBRAIN_RAW_IMPORTS_TTL_DAYS', 'QUAID_RAW_IMPORTS_TTL_DAYS'],
  ['GBRAIN_FULL_HASH_AUDIT_DAYS', 'QUAID_FULL_HASH_AUDIT_DAYS'],
  ['GBRAIN_INSTALL_DIR', 'QUAID_INSTALL_DIR'],
  ['GBRAIN_VERSION', 'QUAID_VERSION'],
  ['GBRAIN_RELEASE_API_URL', 'QUAID_RELEASE_API_URL'],
  ['GBRAIN_RELEASE_BASE_URL', 'QUAID_RELEASE_BASE_URL'],
  // Default paths
  ['brain.db', 'memory.db'],
  ['~/.gbrain', '~/.quaid'],
  ['.gbrain/', '.quaid/'],
  ['.gbrain"', '.quaid"'],
  // Ignore file
  ['.gbrainignore', '.quaidignore'],
  // MCP tools - specific tool names (order: most specific first)
  ['brain_link_close', 'memory_link_close'],
  ['brain_backlinks', 'memory_backlinks'],
  ['brain_gap_approve', 'memory_gap_approve'],
  ['brain_collections', 'memory_collections'],
  ['brain_timeline', 'memory_timeline'],
  ['brain_search', 'memory_search'],
  ['brain_query', 'memory_query'],
  ['brain_stats', 'memory_stats'],
  ['brain_gaps', 'memory_gaps'],
  ['brain_tags', 'memory_tags'],
  ['brain_graph', 'memory_graph'],
  ['brain_check', 'memory_check'],
  ['brain_link', 'memory_link'],
  ['brain_list', 'memory_list'],
  ['brain_put', 'memory_put'],
  ['brain_get', 'memory_get'],
  ['brain_gap', 'memory_gap'],
  ['brain_raw', 'memory_raw'],
  // Config table name
  ['brain_config', 'quaid_config'],
  // npm package
  ['npm install -g gbrain', 'npm install -g quaid'],
];

// README-specific replacements (applied after common ones)
const readmeSpecific = [
  // Fix title and subtitle (after GigaBrain->Quaid)
  ['# Quaid\n\n> Open-source personal knowledge brain. SQLite + FTS5 + vector embeddings in one file. Thin CLI harness, fat skill files. MCP-ready from day one. Runs anywhere. No API keys, no Docker. Airgapped + online release channels with configurable BGE models in the online build.',
   '# Quaid\n\n> Persistent memory for AI agents. Local-first, MCP-native, no cloud dependency.'],
  // Fix Why section - remove the git scaling paragraph, rewrite opener
  ['## Why\n\nGit doesn\'t scale past ~5,000 markdown files. At that size, a wiki-brain becomes slow to clone, painful to search, and unusable for structured queries. Full-text search requires grep. Semantic search requires an external vector database. Cross-references are just markdown links with no queryable graph.\n\nEvery existing knowledge tool (Obsidian, Notion, RAG frameworks) either requires a GUI, locks your data in a SaaS platform, or needs an internet connection and API keys. Quaid is designed for an agent-first world where your knowledge layer needs to:\n\n- Live in a single file you own completely\n- Do full-text **and** semantic search natively\n- Expose an MCP server for any AI client\n- Work on a plane, in an air-gapped environment, with zero ongoing costs',
   '## Why\n\nEvery existing knowledge tool (Obsidian, Notion, RAG frameworks) either requires a GUI, locks your data in a SaaS platform, or needs an internet connection and API keys. Quaid is designed for an agent-first world where your memory layer needs to:\n\n- Live in a single file you own completely\n- Do full-text **and** semantic search natively\n- Expose an MCP server for any AI client\n- Work offline, in an air-gapped environment, with zero ongoing costs\n- Persist knowledge across agent sessions without external dependencies'],
  // Fix intro paragraph  
  ['Inspired by [Garry Tan\'s GBrain work](https://gist.github.com/garrytan/49c88e83cf8d7ae95e087426368809cb), Quaid adapts the same core concept — a personal knowledge brain with compiled truth and append-only timelines — to a local-first Rust + SQLite architecture built for portable, offline use. No API keys. No internet required. No Docker. One static binary, drop it anywhere.',
   'Quaid gives AI agents a persistent, queryable memory layer backed by a single SQLite file. Every piece of knowledge has two parts: **compiled truth** (always current, rewritten as new evidence arrives) and a **timeline** (append-only evidence base, never rewritten). Agents read and write through 17 MCP tools over stdio — no API keys, no cloud, no Docker. One static binary.\n\nInspired by [Garry Tan\'s GBrain work](https://gist.github.com/garrytan/49c88e83cf8d7ae95e087426368809cb), Quaid adapts the same compiled-knowledge model to a local-first Rust + SQLite architecture built for portable, offline use.'],
  // Fix "create a new brain" usage comment
  ['# Create a new brain\nquaid init ~/memory.db', '# Create a new memory store\nquaid init ~/.quaid/memory.db'],
  ['quaid import /path/to/notes/ --db ~/memory.db', 'quaid import /path/to/notes/ --db ~/.quaid/memory.db'],
  // Fix "Brain stats" comment
  ['# Brain stats\nquaid stats', '# Memory stats\nquaid stats'],
  // Fix "Validate brain integrity" comment
  ['# Validate brain integrity (Phase 3)', '# Validate memory integrity (Phase 3)'],
  // Fix call with memory_stats (brain_stats was already renamed)
  ['quaid call memory_stats', 'quaid call memory_stats'],
  // Fix "memory database" reference (only in context of pipe example)
  ['tool":"memory_search","input":{"query":"machine learning"}}\'', 'tool":"memory_search","input":{"query":"machine learning"}}\''],
  // MCP config key name
  ['"gbrain": {\n      "command": "quaid"', '"quaid": {\n      "command": "quaid"'],
  ['"QUAID_DB": "/path/to/memory.db"', '"QUAID_DB": "/path/to/memory.db"'],
  // Release asset names
  ['quaid-${PLATFORM}-airgapped', 'quaid-${PLATFORM}-airgapped'],
  // Fix "cd quaid" from "cd gigabrain"
  ['cd quaid\n', 'cd quaid\n'],
  // Roadmap table entry
  ['write-through `memory_put`', 'write-through `memory_put`'],
  // Acknowledgements - fix "Same brain"
  ['Same brain, different stack, different deployment story.', 'Same memory model, different stack, different deployment story.'],
  // Skills section
  ['Skills are markdown files that tell agents how to use Quaid. They live in `skills/` and are embedded in the binary by default, extracted to `~/.quaid/skills/` on first run.',
   'Skills are markdown files that tell agents how to use Quaid. They live in `skills/` and are embedded in the binary by default, extracted to `~/.quaid/skills/` on first run.'],
  // Brain Briefing output heading in briefing skill
  ['# Brain Briefing', '# Memory Briefing'],
  // npm package
  ['| `npm install -g quaid`', '| `npm install -g quaid`'],
  // Fix npm note about gbrain package on npm
  ['The `quaid` package name on npm has existing published versions.', 'The `quaid` package name on npm has existing published versions.'],
  // Fix "gbrain serve" in contributing.md context
  ['`quaid serve`, live-sync watcher', '`quaid serve`, live-sync watcher'],
  // Roadmap - binary asset name pattern
  ['quaid-<platform>-<channel>', 'quaid-<platform>-<channel>'],
  // Fix QUAID_MODEL note
  ['`QUAID_MODEL` and `--model` are supported in the `online-model` build. In the default airgapped build they are a warning-only no-op and Quaid continues with embedded `small`.',
   '`QUAID_MODEL` and `--model` are supported in the `online-model` build. In the default airgapped build they are a warning-only no-op and Quaid continues with embedded `small`.'],
];

// Process README.md
console.log('Processing README.md...');
let readmeContent = fs.readFileSync('D:/repos/quaid/README.md', 'utf8');
for (const [from, to] of commonReplacements) {
  readmeContent = readmeContent.split(from).join(to);
}
for (const [from, to] of readmeSpecific) {
  readmeContent = readmeContent.split(from).join(to);
}
fs.writeFileSync('D:/repos/quaid/README.md', readmeContent, 'utf8');
console.log('README.md done');

// Check remaining old terms in README
const remainingReadme = (readmeContent.match(/\bGigaBrain\b|\bgbrain\b|\bGBRAIN_|brain\.db|~\/\.gbrain/g) || []);
console.log('README remaining old terms:', remainingReadme.length, JSON.stringify(remainingReadme.slice(0, 20)));

// Process AGENTS.md
console.log('\nProcessing AGENTS.md...');
let agentsContent = fs.readFileSync('D:/repos/quaid/AGENTS.md', 'utf8');
for (const [from, to] of commonReplacements) {
  agentsContent = agentsContent.split(from).join(to);
}
// AGENTS.md specific fixes
agentsContent = agentsContent.split('# Quaid — Agent Instructions').join('# Quaid — Agent Instructions');
agentsContent = agentsContent.split('Personal knowledge brain.').join('Persistent memory for AI agents.');
agentsContent = agentsContent.split('GigaBrain stores your knowledge as structured pages in a single SQLite file (`memory.db`).').join('Quaid stores your knowledge as structured pages in a single SQLite file (`memory.db`).');
agentsContent = agentsContent.split('`memory_put` uses optimistic concurrency (`expected_version`). Re-fetch before writing.').join('`memory_put` uses optimistic concurrency (`expected_version`). Re-fetch before writing.');
agentsContent = agentsContent.split('`memory_gap` always creates gaps with `sensitivity = \'internal\'`. Escalation requires `memory_gap_approve`.').join('`memory_gap` always creates gaps with `sensitivity = \'internal\'`. Escalation requires `memory_gap_approve`.');
agentsContent = agentsContent.split('quaid init ~/memory.db').join('quaid init ~/.quaid/memory.db');
agentsContent = agentsContent.split('quaid import /path/to/notes/').join('quaid import /path/to/notes/');
fs.writeFileSync('D:/repos/quaid/AGENTS.md', agentsContent, 'utf8');
console.log('AGENTS.md done');

// Process CLAUDE.md
console.log('\nProcessing CLAUDE.md...');
let claudeContent = fs.readFileSync('D:/repos/quaid/CLAUDE.md', 'utf8');
for (const [from, to] of commonReplacements) {
  claudeContent = claudeContent.split(from).join(to);
}
// CLAUDE.md specific: fix the title
claudeContent = claudeContent.split('# Quaid\n\nPersonal knowledge brain.').join('# Quaid\n\nPersistent memory for AI agents.');
// Fix the DB diagram
claudeContent = claudeContent.split('memory.db                   — SQLite: pages + FTS5 + vec0 + links + assertions').join('memory.db                  — SQLite: pages + FTS5 + vec0 + links + assertions');
// Fix quaid_config reference
claudeContent = claudeContent.split('`quaid_config` table at `quaid init`').join('`quaid_config` table at `quaid init`');
fs.writeFileSync('D:/repos/quaid/CLAUDE.md', claudeContent, 'utf8');
console.log('CLAUDE.md done');

// Check remaining in CLAUDE.md
const remainingClaude = (claudeContent.match(/\bGigaBrain\b|\bgbrain\b|GBRAIN_|brain\.db|~\/\.gbrain|\bbrain_config\b/g) || []);
console.log('CLAUDE.md remaining old terms:', remainingClaude.length, JSON.stringify(remainingClaude.slice(0, 20)));

// Fix remaining README issues
console.log('\nFixing remaining README issues...');
let readmeContent2 = fs.readFileSync('D:/repos/quaid/README.md', 'utf8');
const readmeFixes = [
  ['so gbrain\n', 'so quaid\n'],
  ['gbrain-install.sh', 'quaid-install.sh'],
  ['.local/bin/gbrain"', '.local/bin/quaid"'],
  ['/usr/local/bin/gbrain', '/usr/local/bin/quaid'],
  ['macro88/quaid', 'quaid-app/quaid'],
  ['macro88/', 'quaid-app/'],
];
for (const [from, to] of readmeFixes) {
  readmeContent2 = readmeContent2.split(from).join(to);
}
// Fix ASSET line with dollar-sign variables
readmeContent2 = readmeContent2.replace('ASSET="gbrain-${PLATFORM}-airgapped"   # or: gbrain-${PLATFORM}-online', 'ASSET="quaid-${PLATFORM}-airgapped"   # or: quaid-${PLATFORM}-online');
fs.writeFileSync('D:/repos/quaid/README.md', readmeContent2, 'utf8');
const remaining2 = (readmeContent2.match(/\bGigaBrain\b|\bgbrain\b|GBRAIN_|brain\.db|~\/\.gbrain|macro88/g) || []);
console.log('README final remaining old terms:', remaining2.length, JSON.stringify(remaining2));

console.log('\nAll done!');
