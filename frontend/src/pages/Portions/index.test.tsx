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

afterEach(() => worker.resetHandlers());
beforeAll(() => {
  worker.listen();
});

afterAll(() => {
  worker.close();
});

describe("Portion Page", () => {
  it("displays a heading", async () => {
    render(
      <MemoryRouter initialEntries={["/products/21/portions"]}>
        <Route path="/products/:id/portions">
          <Portions />
        </Route>
      </MemoryRouter>
    );
    // verify page content for expected route

    await waitFor(() => {
      expect(screen.getByText("Portions")).toBeInTheDocument();
    });

    const heading = screen.getByText("Portions");
    expect(getNodeText(heading)).toMatch("Portions");
    expect(heading.nodeName).toMatch("H1");
  });

  it("has a table of portions, with name and mass in grams", async () => {
    const history = createMemoryHistory();
    history.push("/products/21/portions");

    render(
      <MemoryRouter initialEntries={["/products/21/portions"]}>
        <Route path="/products/:id/portions">
          <Portions />
        </Route>
      </MemoryRouter>
    );
    await waitFor(() => {
      expect(screen.getByText("Portions")).toBeInTheDocument();
    });

    const name = screen.getByText("serving");
    const mass = screen.getByText((_content, node) => {
      const hasText = (node: any) => node.textContent.includes("20");
      const nodeHasText = hasText(node);
      if (!node) {
        return;
      }
      const childrenDontHaveText = Array.from(node.children).every(
        (child) => !hasText(child)
      );

      return nodeHasText && childrenDontHaveText;
    });
    expect(name).toBeInTheDocument();
    expect(mass).toBeInTheDocument();
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
});
