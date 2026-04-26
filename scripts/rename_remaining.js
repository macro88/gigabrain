#!/usr/bin/env node
'use strict';

const fs = require('fs');

function replaceAll(str, from, to) {
  return str.split(from).join(to);
}

function processFile(path, extraReplacements) {
  let content = fs.readFileSync(path, 'utf8');
  for (const [from, to] of commonReplacements) {
    content = replaceAll(content, from, to);
  }
  if (extraReplacements) {
    for (const [from, to] of extraReplacements) {
      content = replaceAll(content, from, to);
    }
  }
  fs.writeFileSync(path, content, 'utf8');
  const remaining = (content.match(/GigaBrain|gbrain|GBRAIN_|brain\.db|~\/\.gbrain|macro88/g) || []);
  console.log(path.split('/').pop() + ' done. Remaining: ' + remaining.length + ' ' + JSON.stringify(remaining.slice(0,10)));
  return content;
}

const commonReplacements = [
  ['GigaBrain', 'Quaid'],
  ['gbrain ', 'quaid '],
  ['`gbrain`', '`quaid`'],
  ['"gbrain"', '"quaid"'],
  ['gbrain\n', 'quaid\n'],
  ['gbrain-', 'quaid-'],
  ['gigabrain', 'quaid'],
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
  ['brain.db', 'memory.db'],
  ['~/.gbrain', '~/.quaid'],
  ['.gbrain/', '.quaid/'],
  ['.gbrain"', '.quaid"'],
  ['.gbrainignore', '.quaidignore'],
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
  ['brain_config', 'quaid_config'],
  ['npm install -g gbrain', 'npm install -g quaid'],
  ['macro88/quaid', 'quaid-app/quaid'],
  ['macro88/', 'quaid-app/'],
  ['/usr/local/bin/gbrain', '/usr/local/bin/quaid'],
  ['.local/bin/gbrain', '.local/bin/quaid'],
];

const base = 'D:/repos/quaid';

// docs/spec.md
processFile(base + '/docs/spec.md', [
  ['# Brain stats', '# Memory stats'],
  ['# Brain Briefing', '# Memory Briefing'],
  ['# Create a new brain', '# Create a new memory store'],
  ['quaid init ~/memory.db', 'quaid init ~/.quaid/memory.db'],
]);

// docs/getting-started.md
processFile(base + '/docs/getting-started.md', [
  ['Your first brain', 'Your first memory store'],
  ['# Brain Briefing', '# Memory Briefing'],
  ['# Brain stats', '# Memory stats'],
  ['# Create a new brain', '# Create a new memory store'],
  ['quaid init ~/memory.db', 'quaid init ~/.quaid/memory.db'],
]);

// docs/contributing.md
processFile(base + '/docs/contributing.md', [
  ['quaid-<platform>-<channel>', 'quaid-<platform>-<channel>'],
]);

// docs/roadmap.md
processFile(base + '/docs/roadmap.md', [
  ['# Brain stats', '# Memory stats'],
  ['# Brain Briefing', '# Memory Briefing'],
]);

// docs/gigabrain-vs-qmd-friction-analysis.md
processFile(base + '/docs/gigabrain-vs-qmd-friction-analysis.md', []);

// phase2_progress.md
processFile(base + '/phase2_progress.md', []);

// skills
const skills = [
  'ingest', 'query', 'maintain', 'briefing', 'research', 'alerts', 'upgrade', 'enrich'
];

for (const skill of skills) {
  const extra = [];
  if (skill === 'briefing') {
    extra.push(['# Brain Briefing', '# Memory Briefing']);
  }
  if (skill === 'upgrade') {
    extra.push(['gbrain 0.2.0', 'quaid 0.2.0']);
  }
  processFile(base + '/skills/' + skill + '/SKILL.md', extra);
}

console.log('\nAll remaining files done!');
