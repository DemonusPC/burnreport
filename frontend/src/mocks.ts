import { rest } from 'msw'
import { setupServer } from 'msw/node'

export const handlers = [
  // get portions
  rest.get('/api/products/:id/portions', (req, res, ctx) => {
    const { id } = req.params

    return res(
      ctx.json({
        result: [
          {
            id: 2,
            product: id,
            name: 'serving',
            grams: 20.0
          },
          {
            id: 11,
            product: id,
            name: 'portion',
            grams: 50.0
          }
        ]
      })
    )
  }),

  // post portions

  rest.post('/api/products/portions', (req, res, ctx) => {
    return res(
      ctx.json({
        status: 'Added'
      })
    )
  }),


  // Recipies
  rest.get('/api/search', (req, res, ctx) => {
    return res(
      ctx.json({
        result: [
          {
            id: 1,
            text: 'Super Chicken',
            subText: null,
            entity: 'Recipie'
          }
        ]
      })
    )
  }),

  rest.get('/api/recipies/:id', (req, res, ctx) => {
    const { id } = req.params

    return res(
      ctx.json(
        {
          "id": 1,
          "name": "Super Chicken",
          "ingredients": [
            {
              "product": {
                "id": 1,
                "name": "Test Product",
                "nutrients": {
                  "energy": {
                    "kcal": 50.0,
                    "kj": 100.0
                  },
                  "carbohydrates": {
                    "total": 10.0,
                    "sugar": 3.0,
                    "fiber": null,
                    "addedSugar": null,
                    "starch": null
                  },
                  "fat": {
                    "total": 20.0,
                    "saturated": 10.0,
                    "unsaturated": {
                      "mono": {
                        "total": 0.0
                      },
                      "poly": {
                        "total": 0.0
                      }
                    }
                  },
                  "protein": {
                    "total": 5.2
                  },
                  "salt": {
                    "total": 0.01
                  },
                  "vitamins": {
                    "fat": {
                      "a": 0.0,
                      "d": 0.0,
                      "e": 0.0,
                      "k": 0.0
                    },
                    "water": {
                      "b1": 0.12,
                      "b2": 0.0,
                      "b3": 0.0,
                      "b5": 0.0,
                      "b6": 0.0,
                      "b7": 0.0,
                      "b9": 0.0,
                      "b12": 0.0,
                      "c": 0.0
                    }
                  }
                },
                "unit": "Grams"
              },
              "amount": 30.5
            }
          ],
          "total": {
            "energy": {
              "kcal": 50.0,
              "kj": 100.0
            },
            "carbohydrates": {
              "total": 10.0,
              "sugar": 3.0,
              "fiber": null,
              "addedSugar": null,
              "starch": null
            },
            "fat": {
              "total": 20.0,
              "saturated": 11.0,
              "unsaturated": {
                "mono": {
                  "total": 0.0
                },
                "poly": {
                  "total": 0.0
                }
              }
            },
            "protein": {
              "total": 5.2
            },
            "salt": {
              "total": 0.01
            },
            "vitamins": {
              "fat": {
                "a": 0.0,
                "d": 0.0,
                "e": 0.0,
                "k": 0.0
              },
              "water": {
                "b1": 0.12,
                "b2": 0.0,
                "b3": 0.0,
                "b5": 0.0,
                "b6": 0.0,
                "b7": 0.0,
                "b9": 0.0,
                "b12": 0.0,
                "c": 0.0
              }
            }
          },
        }

      )
    )
  }),

]

export const worker = setupServer(...handlers)
