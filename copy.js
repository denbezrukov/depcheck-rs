var copy = require('copy-files');

copy({
    files: {
        'binding.js': './node/src/binding.js',
        'binding.d.ts': './node/src/binding.d.ts'
    },
    dest: 'dist/',
}, function (err) {
});
