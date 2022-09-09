/** @type {import('ts-jest/dist/types').InitialOptionsTsJest} */
module.exports = {
  preset: "ts-jest",
  testEnvironment: "jsdom",
  setupFilesAfterEnv: ["./src/setupTests.ts"],
  resetMocks: true,
  globals: {
    "ts-jest": {
      isolatedModules: true,
    },
  },
};
