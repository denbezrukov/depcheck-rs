const {depcheck} = require('../binding');
const binding = require('../binding');

test('adds 1 + 2 to equal 3', () => {
  console.log(binding);
  const result = depcheck("");
  console.log(result);
});
