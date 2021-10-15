import React from "react";
import {
  render,
  getNodeText,
  waitFor,
  screen,
  fireEvent,
} from "@testing-library/react";
import { MemoryRouter, Route } from "react-router-dom";
import { createMemoryHistory } from "history";
import Portions from "./index";
import { worker } from "../../mocks";

import { rest } from "msw";

beforeAll(() => {
  // we're using fake timers because we don't want to

  // wait a full second for this test to run.

  jest.useFakeTimers();
});

afterAll(() => {
  jest.useRealTimers();
});

test("heading is displayed", async () => {
  const { getByText } = render(<Portions />, {
    wrapper: MemoryRouter,
  });
  // verify page content for expected route

  await waitFor(() => {
    expect(screen.getByText("Portions")).toBeInTheDocument();
  });

  const heading = getByText("Portions");
  expect(getNodeText(heading)).toMatch("Portions");
  expect(heading.nodeName).toMatch("H1");
});

it("has a table of portions, with name and mass in grams", async () => {
  const { container, getByText } = render(<Portions />, {
    wrapper: MemoryRouter,
  });

  await waitFor(() => {
    expect(screen.getByText("Name")).toBeInTheDocument();
  });

  const name = getByText("Name");
  const mass = getByText("Mass (grams)");

  expect(name.parentElement?.nodeName).toMatch("TH");
  expect(mass.parentElement?.nodeName).toMatch("TH");
});

it("updates the table after a correct rest call", async () => {
  const history = createMemoryHistory();
  history.push("/products/21/portions");

  const { container, getByText } = render(
    <MemoryRouter initialEntries={["/products/21/portions"]}>
      <Route path="/products/:id/portions">
        <Portions />
      </Route>
    </MemoryRouter>
  );

  await waitFor(() => {
    expect(getByText("portion")).toBeInTheDocument();
  });

  const name = getByText("portion");
  const mass = getByText("50");
  expect(name.parentElement?.nodeName).toMatch("TD");
  expect(mass.parentElement?.nodeName).toMatch("TD");
});

test("clicking the add portion button reveals the add portion form", async () => {
  const history = createMemoryHistory();
  history.push("/products/21/portions");

  const { findByText, getByText } = render(
    <MemoryRouter initialEntries={["/products/21/portions"]}>
      <Route path="/products/:id/portions">
        <Portions />
      </Route>
    </MemoryRouter>
  );

  fireEvent.click(getByText("Add portion"));

  const addButton = await findByText("Add");
  const portionField = await findByText("Portion (in grams)");

  expect(addButton.parentElement?.nodeName).toStrictEqual("BUTTON");
  expect(portionField.nodeName).toStrictEqual("LABEL");
});

test("not filling out the form prevents sending", async () => {
  const history = createMemoryHistory();
  history.push("/products/21/portions");

  const { getByText, findAllByText } = render(
    <MemoryRouter initialEntries={["/products/21/portions"]}>
      <Route path="/products/:id/portions">
        <Portions />
      </Route>
    </MemoryRouter>
  );

  fireEvent.click(getByText("Add portion"));

  await waitFor(() => {
    expect(getByText("Add")).toBeInTheDocument();
  });

  fireEvent.click(getByText("Add"));

  const required_labels = await findAllByText("required");

  expect(required_labels).toHaveLength(2);
});

test("sending the form creates a new element", async () => {
  const history = createMemoryHistory();
  history.push("/products/21/portions");

  const { getByText, findAllByText } = render(
    <MemoryRouter initialEntries={["/products/21/portions"]}>
      <Route path="/products/:id/portions">
        <Portions />
      </Route>
    </MemoryRouter>
  );

  fireEvent.click(getByText("Add portion"));

  await waitFor(() => {
    expect(getByText("Add")).toBeInTheDocument();
  });

  fireEvent.click(getByText("Add"));

  const required_labels = await findAllByText("required");

  expect(required_labels).toHaveLength(2);

  worker.use(
    rest.get("/api/products/:id/portions", (req, res, ctx) => {
      const { id } = req.params;

      return res(
        ctx.json([
          {
            id: 2,
            product: id,
            name: "serving",
            grams: 20.0,
          },
          {
            id: 11,
            product: id,
            name: "portion",
            grams: 50.0,
          },
          {
            id: 11,
            product: id,
            name: "bar",
            grams: 100.0,
          },
        ])
      );
    })
  );
});
