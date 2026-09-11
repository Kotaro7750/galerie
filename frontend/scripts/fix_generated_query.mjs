import { readFileSync, writeFileSync } from 'node:fs';

// OpenAPI Generator 7.24.0 loses the JSON query's item type and uses CSV encoding.
const path = new URL('../src/api/generated/apis/ContentsApi.ts', import.meta.url);
let source = readFileSync(path, 'utf8');
const replacements = [
  ['condition?: Array;', 'condition?: Array<SearchTerm>;'],
  [
    `queryParameters['condition'] = requestParameters['condition']!.join(runtime.COLLECTION_FORMATS["csv"]);`,
    `queryParameters['condition'] = JSON.stringify(requestParameters['condition']);`,
  ],
];

for (const [before, after] of replacements) {
  if (source.split(before).length !== 2) {
    throw new Error('Generated JSON query code changed; review the generation workaround.');
  }
  source = source.replace(before, after);
}

writeFileSync(path, source);
