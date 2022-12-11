module.exports = {
  env: {
    browser: true,
    es2021: true
  },
  extends: [
    "plugin:prettier/recommended",
    'plugin:react/recommended',
    'standard-with-typescript'
  ],
  overrides: [
  ],
  parserOptions: {
    ecmaVersion: 'latest',
    ecmaFeatures: {
      jsx: true,
    },
    sourceType: 'module',
    project: ['./frontend/tsconfig.json']
  },
  plugins: [
    'react','prettier'
  ],
  rules: {
  }
}
