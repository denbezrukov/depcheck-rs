const fs = require('fs')
const { promisify } = require('util')

const copyFileAsync = promisify(fs.copyFile);
const existsAsync = promisify(fs.exists);
const mkdirAsync = promisify(fs.mkdir);

const run = async () => {
    const isDistExist = await existsAsync('./dist');
    if (!isDistExist) {
        await mkdirAsync('./dist');
    }
    await copyFileAsync('./node/src/binding.js', './dist/binding.js');
    await copyFileAsync('./node/src/binding.d.ts', './dist/binding.d.ts');
}

run();
