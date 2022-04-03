# depcheck-rs
Work in progress.

Depcheck is a tool for analyzing the dependencies in a project to see: how each dependency is used, which dependencies are useless, and which dependencies are missing from `package.json`.
It is a port of original [depcheck](https://github.com/depcheck/depcheck). It uses [swc](https://github.com/swc-project/swc) for parsing.

## Installation

```
npm install @depcheck-rs-node/core
```

## Syntax Support

Depcheck not only recognizes the dependencies in JavaScript files, but also supports these syntaxes:

- JavaScript (ES5, ES6 and ES7)
- [React JSX](http://facebook.github.io/react/docs/jsx-in-depth.html)
- [Typescript](http://www.typescriptlang.org/)

[comment]: <> (## Usage)

[comment]: <> (```)

[comment]: <> (depcheck [directory] [arguments])

[comment]: <> (```)

[comment]: <> (The `directory` argument is the root directory of your project &#40;where the `package.json` file is&#41;. If unspecified, defaults to current directory.)

[comment]: <> (All of the arguments are optional:)

[comment]: <> (`--ignore-bin-package=[true|false]`: A flag to indicate if depcheck ignores the packages containing bin entry. The default value is `false`.)

[comment]: <> (`--skip-missing=[true|false]`: A flag to indicate if depcheck skips calculation of missing dependencies. The default value is `false`.)

[comment]: <> (`--json`: Output results in JSON. When not specified, depcheck outputs in human friendly format.)

[comment]: <> (`--oneline`: Output results as space separated string. Useful for copy/paste.)

[comment]: <> (`--ignores`: A comma separated array containing package names to ignore. It can be glob expressions. Example, `--ignores="eslint,babel-*"`.)

[comment]: <> (`--ignore-dirs`: DEPRECATED, use ignore-patterns instead. A comma separated array containing directory names to ignore. Example, `--ignore-dirs=dist,coverage`.)

[comment]: <> (`--ignore-path`: Path to a file with patterns describing files to ignore. Files must match the .gitignore [spec]&#40;http://git-scm.com/docs/gitignore&#41;. Example, `--ignore-path=.eslintignore`.)

[comment]: <> (`--ignore-patterns`: Comma separated patterns describing files to ignore. Patterns must match the .gitignore [spec]&#40;http://git-scm.com/docs/gitignore&#41;. Example, `--ignore-patterns=build/Release,dist,coverage,*.log`.)

[comment]: <> (`--help`: Show the help message.)

[comment]: <> (`--parsers`, `--detectors` and `--specials`: These arguments are for advanced usage. They provide an easy way to customize the file parser and dependency detection. Check [the pluggable design document]&#40;https://github.com/depcheck/depcheck/blob/master/doc/pluggable-design.md&#41; for more information.)

[comment]: <> (`--config=[filename]`: An external configuration file &#40;see below&#41;.)

[comment]: <> (## Usage with a configuration file)

[comment]: <> (Depcheck can be used with an rc configuration file. In order to do so, create a .depcheckrc file in your project's package.json folder, and set the CLI keys in YAML, JSON, and Javascript formats.)

[comment]: <> (For example, the CLI arguments `--ignores="eslint,babel-*" --skip-missing=true` would turn into:)

[comment]: <> (**_.depcheckrc_**)

[comment]: <> (```)

[comment]: <> (ignores: ["eslint", "babel-*"])

[comment]: <> (skip-missing: true)

[comment]: <> (```)

[comment]: <> (**Important:** if provided CLI arguments conflict with configuration file ones, the CLI ones will take precedence over the rc file ones.)

[comment]: <> (The rc configuration file can also contain the following extensions: `.json`, `.yaml`, `.yml`.)

[comment]: <> (## API)

[comment]: <> (Similar options are provided to `depcheck` function for programming:)

[comment]: <> (```js)

[comment]: <> (import depcheck from 'depcheck';)

[comment]: <> (const options = {)

[comment]: <> (  ignoreBinPackage: false, // ignore the packages with bin entry)

[comment]: <> (  skipMissing: false, // skip calculation of missing dependencies)

[comment]: <> (  ignorePatterns: [)

[comment]: <> (    // files matching these patterns will be ignored)

[comment]: <> (    'sandbox',)

[comment]: <> (    'dist',)

[comment]: <> (    'bower_components',)

[comment]: <> (  ],)

[comment]: <> (  ignoreMatches: [)

[comment]: <> (    // ignore dependencies that matches these globs)

[comment]: <> (    'grunt-*',)

[comment]: <> (  ],)

[comment]: <> (  parsers: {)

[comment]: <> (    // the target parsers)

[comment]: <> (    '**/*.js': depcheck.parser.es6,)

[comment]: <> (    '**/*.jsx': depcheck.parser.jsx,)

[comment]: <> (  },)

[comment]: <> (  detectors: [)

[comment]: <> (    // the target detectors)

[comment]: <> (    depcheck.detector.requireCallExpression,)

[comment]: <> (    depcheck.detector.importDeclaration,)

[comment]: <> (  ],)

[comment]: <> (  specials: [)

[comment]: <> (    // the target special parsers)

[comment]: <> (    depcheck.special.eslint,)

[comment]: <> (    depcheck.special.webpack,)

[comment]: <> (  ],)

[comment]: <> (  package: {)

[comment]: <> (    // may specify dependencies instead of parsing package.json)

[comment]: <> (    dependencies: {)

[comment]: <> (      lodash: '^4.17.15',)

[comment]: <> (    },)

[comment]: <> (    devDependencies: {)

[comment]: <> (      eslint: '^6.6.0',)

[comment]: <> (    },)

[comment]: <> (    peerDependencies: {},)

[comment]: <> (    optionalDependencies: {},)

[comment]: <> (  },)

[comment]: <> (};)

[comment]: <> (depcheck&#40;'/path/to/your/project', options&#41;.then&#40;&#40;unused&#41; => {)

[comment]: <> (  console.log&#40;unused.dependencies&#41;; // an array containing the unused dependencies)

[comment]: <> (  console.log&#40;unused.devDependencies&#41;; // an array containing the unused devDependencies)

[comment]: <> (  console.log&#40;unused.missing&#41;; // a lookup containing the dependencies missing in `package.json` and where they are used)

[comment]: <> (  console.log&#40;unused.using&#41;; // a lookup indicating each dependency is used by which files)

[comment]: <> (  console.log&#40;unused.invalidFiles&#41;; // files that cannot access or parse)

[comment]: <> (  console.log&#40;unused.invalidDirs&#41;; // directories that cannot access)

[comment]: <> (}&#41;;)

[comment]: <> (```)

[comment]: <> (## Example)

[comment]: <> (The following example checks the dependencies under `/path/to/my/project` folder:)

[comment]: <> (```sh)

[comment]: <> ($> depcheck /path/to/my/project)

[comment]: <> (Unused dependencies)

[comment]: <> (* underscore)

[comment]: <> (Unused devDependencies)

[comment]: <> (* jasmine)

[comment]: <> (Missing dependencies)

[comment]: <> (* lodash)

[comment]: <> (```)

[comment]: <> (It figures out:)

[comment]: <> (- The dependency `underscore` is declared in the `package.json` file, but not used by any code.)

[comment]: <> (- The devDependency `jasmine` is declared in the `package.json` file, but not used by any code.)

[comment]: <> (- The dependency `lodash` is used somewhere in the code, but not declared in the `package.json` file.)

[comment]: <> (Please note that, if a subfolder has a `package.json` file, it is considered another project and should be checked with another depcheck command.)

[comment]: <> (The following example checks the same project, however, outputs as a JSON blob. Depcheck's JSON output is in one single line for easy pipe and computation. The [`json`]&#40;https://www.npmjs.com/package/json&#41; command after the pipe is a node.js program to beautify the output.)

[comment]: <> (```js)

[comment]: <> ($> depcheck /path/to/my/project --json | json)

[comment]: <> ({)

[comment]: <> (  "dependencies": [)

[comment]: <> (    "underscore")

[comment]: <> (  ],)

[comment]: <> (  "devDependencies": [)

[comment]: <> (    "jasmine")

[comment]: <> (  ],)

[comment]: <> (  "missing": {)

[comment]: <> (    "lodash": [)

[comment]: <> (      "/path/to/my/project/file.using.lodash.js")

[comment]: <> (    ])

[comment]: <> (  },)

[comment]: <> (  "using": {)

[comment]: <> (    "react": [)

[comment]: <> (      "/path/to/my/project/file.using.react.jsx",)

[comment]: <> (      "/path/to/my/project/another.file.using.react.jsx")

[comment]: <> (    ],)

[comment]: <> (    "lodash": [)

[comment]: <> (      "/path/to/my/project/file.using.lodash.js")

[comment]: <> (    ])

[comment]: <> (  },)

[comment]: <> (  "invalidFiles": {)

[comment]: <> (    "/path/to/my/project/file.having.syntax.error.js": "SyntaxError: <call stack here>")

[comment]: <> (  },)

[comment]: <> (  "invalidDirs": {)

[comment]: <> (    "/path/to/my/project/folder/without/permission": "Error: EACCES, <call stack here>")

[comment]: <> (  })

[comment]: <> (})

[comment]: <> (```)

[comment]: <> (- The `dependencies`, `devDependencies` and `missing` properties have the same meanings in the previous example.)

[comment]: <> (- The `using` property is a lookup indicating each dependency is used by which files.)

[comment]: <> (- The value of `missing` and `using` lookup is an array. It means the dependency may be used by many files.)

[comment]: <> (- The `invalidFiles` property contains the files having syntax error or permission error. The value is the error details. However, only one error is stored in the lookup.)

[comment]: <> (- The `invalidDirs` property contains the directories having permission error. The value is the error details.)

## License

MIT License.
