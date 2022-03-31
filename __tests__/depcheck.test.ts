const {depcheck} = require('../index.js');

test('should not failed', () => {
  const result = depcheck('./crates/core/tests/fake_modules/bad');
  expect(result).toMatchSnapshot();
});
