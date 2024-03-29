// jest-dom adds custom jest matchers for asserting on DOM nodes.
// allows you to do things like:
// expect(element).toHaveTextContent(/react/i)
// learn more: https://github.com/testing-library/jest-dom
import "@testing-library/jest-dom";
import "whatwg-fetch";
import { setupServer } from "msw/node";
import { handlers } from "./mocks";

// Setup requests interception using the given handlers.
const server = setupServer(...handlers);

// This removes a lot of annying scrollTo errors when running the tests
window.scrollTo = jest.fn();

beforeAll(() => {
  // Enable the mocking in tests.
  server.listen();
});

afterEach(() => {
  // Reset any runtime handlers tests may use.
  server.resetHandlers();
});

afterAll(() => {
  // Clean up once the tests are done.
  server.close();
});
