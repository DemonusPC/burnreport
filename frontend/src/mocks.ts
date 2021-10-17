import { rest } from "msw";
import { setupServer } from "msw/node";

export const handlers = [
  // get portions
  rest.get("/api/products/:id/portions", (req, res, ctx) => {
    const { id } = req.params;

    return res(
      ctx.json({
        result: [
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
        ],
      })
    );
  }),

  // post portions

  rest.post("/api/products/portions", (req, res, ctx) => {
    return res(
      ctx.json({
        status: "Added",
      })
    );
  }),

  // delete portion
];

export const worker = setupServer(...handlers);
