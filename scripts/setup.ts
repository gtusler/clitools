#!/usr/local/bin/bun

import path from 'path';
import chalk from 'chalk';
import { mkdirSync } from 'fs';


const packages = [
    'clitinfo',
    'clit-install',
    'countlines',
    'diffdir',
    'filers',
    'listdirs',
    'mimers',
    'meta',
    'music-meta-renamer',
    'progressbar',
    'rot',
    'sysinfo',
    'timers',
    'treers',
];
const importTo = '/home/govi/dev/bin/clit';
// const importTo = 'D:/dev/bin';

const symlinker = process.platform === 'win32'
    ? symlinkWindows
    : symlinkLinux;

async function doTheThing() {
    const importToFile = Bun.file(importTo);

    if (! await importToFile.exists()) {
        console.log(chalk.green('Creating dir `' + importTo + '`'));
        mkdirSync(importTo, { recursive: true });
    }

    for (let i = 0; i < packages.length; i++) {
        const pkg = packages[i];
        console.log(pkg);
        const pkgBinPath = path.join('target', 'release', pkg);
        const pkgOutputPath = path.join(importTo, pkg);
        const pkgOutputFile = Bun.file(pkgOutputPath);

        if (! await pkgOutputFile.exists()) {
            console.log(pkgBinPath, '->', pkgOutputPath);
            // symlinkSync(pkgBinPath, pkgOutputPath, 'file');
            symlinker(path.resolve(pkgBinPath), pkgOutputPath);
        }
    }
}

doTheThing();

function symlinkLinux(from: string, to: string): void {
    Bun.spawnSync(['ln', '-s', from, to]);
}

function symlinkWindows(from: string, to: string): void {
    Bun.spawnSync(['cmd', '/c', 'mklink', to, from]);
    // Bun.spawnSync(['New-Item', '-Path', to, '-ItemType', 'SymbolicLink', '-Value', from]);
}
