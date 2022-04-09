const fs = require('fs')
const { promisify } = require('util')

const readdirAsync = promisify(fs.readdir);
const copyFileAsync = promisify(fs.copyFile);

const run = async () => {
    const stat = await readdirAsync('./');

    for (const file of stat) {
        if (/\.node$/.test(file)) {
            await copyFileAsync(file, `./node/src/${file}`);
        }
    }
}

run();
