# depcheck-rs

Depcheck is a tool for analyzing the dependencies in a project to see: how each dependency is used, which dependencies are useless, and which dependencies are missing from `package.json`.
It is a port of original [depcheck](https://github.com/depcheck/depcheck). It uses [swc](https://github.com/swc-project/swc) for parsing.

## Installation

Install with npm:
```
npm install --save-dev @depcheck-rs-node/core
```
Install with yarn:
```
yarn add --dev @depcheck-rs-node/core
```

Or use rust to install depcheck-rs cli:

```
cargo install depcheck-rs-cli
```

## Syntax Support

Depcheck not only recognizes the dependencies in JavaScript files, but also supports these syntaxes:

- JavaScript (ES5, ES6 and ES7)
- [React JSX](http://facebook.github.io/react/docs/jsx-in-depth.html)
- [Typescript](http://www.typescriptlang.org/)

## Usage

```
USAGE:
    depcheck-rs [OPTIONS]

OPTIONS:
    -d, --directory <DIRECTORY>
            The directory argument is the root directory of your project [default: .]

    -h, --help
            Print help information

        --ignore-bin-package
            A flag to indicate if depcheck ignores the packages containing bin entry

        --ignore-path <IGNORE_PATH>
            Path to a file with patterns describing files to ignore

        --ignore-patterns <IGNORE_PATTERNS>
            Comma separated patterns describing files or directories to ignore

        --ignore_matches <IGNORE_MATCHES>
            A comma separated array containing package names to ignore

    -q, --quiet
            Less output per occurrence

        --skip-missing
            A flag to indicate if depcheck skips calculation of missing dependencies

    -v, --verbose
            More output per occurrence

    -V, --version
            Print version information

```

## API

```js

import {depcheck} from "@depcheck-rs-node/core";

const options = {

  ignoreBinPackage: false, // ignore the packages with bin entry

  skipMissing: false, // skip calculation of missing dependencies

  ignorePatterns: [

    // files matching these patterns will be ignored

    'sandbox',

    'dist',

    'bower_components',

  ],

  ignoreMatches: [

    // ignore dependencies that matches these globs

    'grunt-*',

  ],

  ignorePath: '/path/to/your/.depcheckignore',
};

depcheck('/path/to/your/project', options).then((result) => {

  console.log(result.unusedDependencies); // an array containing the unused dependencies

  console.log(result.unusedDevDependencies); // an array containing the unused devDependencies

  console.log(result.missingDependencies); // a lookup containing the dependencies missing in `package.json` and where they are used

  console.log(result.usingDependencies); // a lookup indicating each dependency is used by which files

});

```

## License

MIT License.
