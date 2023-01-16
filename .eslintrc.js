module.exports = {
  root: true,
  env: {
    browser: true,
    es2015: true,
    node: true,
  },
  extends: 'standard-with-typescript',
  overrides: [
    {
      files: ['.eslintrc.js'],
      parserOptions: {
        parser: '@typescript-eslint/parser',
        project: './tsconfig.json',
      },
    },
  ],
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module',
    parser: '@typescript-eslint/parser',
    project: './tsconfig.json',
  },
  rules: {
    'comma-dangle': [2, 'always-multiline'],
    '@typescript-eslint/comma-dangle': [2, 'always-multiline'],
    semi: [2, 'always'],
    '@typescript-eslint/explicit-function-return-type': 0,
    '@typescript-eslint/space-before-function-paren': 0,
    '@typescript-eslint/semi': 0,
    '@typescript-eslint/member-delimiter-style': [
      2,
      {
        multiline: {
          delimiter: 'semi',
          requireLast: true,
        },
        singleline: {
          delimiter: 'semi',
          requireLast: true,
        },
        multilineDetection: 'brackets',
      },
    ],
  },
};
