#!/usr/bin/env node
import * as path from 'path';
import * as yargs from 'yargs';
import { readFile, writeFile } from 'fs';
import { promisify } from 'util';
import makeDir from 'make-dir';
import { diff } from './dist/node_lcs_img_diff';

const read = promisify(readFile);
const write = promisify(writeFile);

// tslint:disable-next-line:no-var-requires
const version = require(path.resolve(__dirname, './package.json')).version as string;

function createOptions() {
  yargs
    .usage('Usage: $0 [options] <command>')
    .help()
    .option('h', { alias: 'help', group: 'Global Options:' })
    .option('p', {
      alias: 'prefix',
      desc: 'Output filename prefix.',
      default: 'marked_',
      group: 'Global Options:',
    })
    .option('d', {
      alias: 'dist',
      desc: 'Output director=y.',
      default: './',
      group: 'Global Options:',
    })
    .option('version', { desc: 'Print version number.', group: 'Global Options:' })
    .version(version);
  const { prefix = 'marked', dist = './' } = yargs.argv;
  const before = yargs.argv._[0];
  const after = yargs.argv._[1];
  if (!before || !after) {
    // tslint:disable-next-line:no-console
    console.error('Please specify before and after images(e.g lcs-img-diff foo.png bar.png');
    process.exit(1);
  }
  return {
    before,
    after,
    prefix,
    dist,
  };
}

async function cli() {
  const options = createOptions();
  const [before, after] = await Promise.all([read(options.before), read(options.after)]);
  const res = JSON.parse(diff(before, after));
  await makeDir(options.dist);
  const beforeFilename = options.prefix + '_' + path.basename(options.before, path.extname(options.before)) + '.png';
  const afterFilename = options.prefix + '_' + path.basename(options.after, path.extname(options.after)) + '.png';
  await write(path.join(options.dist, beforeFilename), new Buffer(res.before));
  await write(path.join(options.dist, afterFilename), new Buffer(res.after));
}

cli()
  .then(() => process.exit(0))
  .catch((why: any) => {
    // tslint:disable-next-line:no-console
    console.error(why);
    process.exit(1);
  });
