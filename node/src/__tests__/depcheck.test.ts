const {depcheck} = require('../index');

test('should not failed', () => {
  const result = depcheck('./crates/core/tests/fake_modules/bad');
  expect(result).toMatchInlineSnapshot(`
Object {
  "missingDependencies": Object {},
  "unusedDependencies": Array [
    "optimist",
  ],
  "unusedDevDependencies": Array [],
  "usingDependencies": Object {},
}
`);
});
