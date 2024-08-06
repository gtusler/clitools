#!/usr/local/bin/bun

import path from 'path';
import chalk from 'chalk';
import { mkdirSync } from 'fs';


const packages = [
    'clitinfo',
    'countlines',
    'mimers',
    'sysinfo',
    'rot',
];
const importTo = '/home/govi/dev/bin/clit';

async function doTheThing() {
    const importToFile = Bun.file(importTo);

    if (! await importToFile.exists()) {
        console.log(chalk.green('Creating dir `' + importTo + '`'));
        mkdirSync(importTo, { recursive: true });
    }

    for (let i = 0; i < packages.length; i++) {
        const pkg = packages[i];
        const pkgBinPath = path.join('target', 'release', pkg);
        const pkgOutputPath = path.join(importTo, pkg);
        const pkgOutputFile = Bun.file(pkgOutputPath);

        if (! await pkgOutputFile.exists()) {
            console.log(pkgBinPath, '->', pkgOutputPath);
            Bun.spawnSync(['ln', '-s', path.resolve(pkgBinPath), pkgOutputPath]);
        }
    }
}

doTheThing();
