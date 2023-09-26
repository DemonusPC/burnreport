import { MemoryRouter, Route } from "react-router-dom";
import ProductPage from ".";
import { RenderResult, render } from "@testing-library/react";
import { GetProduct, GetProductPortions } from "./productApi";
import { Unit } from "../../product/product";
import { Nutrients } from "../../nutrients/nutrients";
import userEvent from "@testing-library/user-event";


const TEST_100G_NUTRIENTS: Nutrients =
{
    energy: {
        kcal: 74,
        kj: 306,
    },
    carbohydrates: {
        total: 52,
        sugar: 12,
        fiber: undefined,
        addedSugar: 2,
        starch: 8
    },
    fat: {
        total: 10,
        saturated: 9
    },
    protein: {
        total: 38
    },
    salt: {
        total: 0.29
    }
};

const TEST_SERVING_NUTRIENTS: Nutrients =
{
    energy: {
        kcal: 740,
        kj: 3060,
    },
    carbohydrates: {
        total: 520,
        sugar: 120,
        fiber: undefined,
        addedSugar: 20,
        starch: 80
    },
    fat: {
        total: 100,
        saturated: 90
    },
    protein: {
        total: 380.
    },
    salt: {
        total: 2.9
    }
};

const renderPage = (productFetcher: GetProduct, portionFetcher: GetProductPortions): RenderResult => {
    const result = render(
        <MemoryRouter initialEntries={['/products/1']}>
            <Route path="/products/:id">
                <ProductPage productFetcher={productFetcher} portionFetcher={portionFetcher} />
            </Route>
        </MemoryRouter>
    )

    return result;
}

const nutrientsAreShown = (page: RenderResult, nutrients: Nutrients) => {

    const { getByText } = page;

    const { energy, carbohydrates, fat, protein, salt } = nutrients;

    const toFind = [
        { label: "Energy", value: `${energy.kj} kJ / ${energy.kcal} kcal` },
        { label: "carbohydrates", value: `${carbohydrates.total || 0} g` },
        { label: "sugar", value: `${carbohydrates.sugar || 0} g` },
        { label: "fiber", value: `${carbohydrates.fiber || 0} g` },
        { label: "addedSugar", value: `${carbohydrates.addedSugar || 0} g` },
        { label: "starch", value: `${carbohydrates.starch || 0} g` },
        { label: "fat", value: `${fat.total || 0} g` },
        { label: "saturated", value: `${fat.saturated || 0} g` },
        { label: "protein", value: `${protein.total} g` },
        { label: "salt", value: `${salt.total} g` },
    ];

    toFind.forEach(({ label, value }) => {
        expect(getByText(label)).toBeInTheDocument();
        expect(getByText(value)).toBeInTheDocument();
    });
}

describe("Given I render the product page", () => {
    describe("when a product exists", () => {
        it("displays the product and allows the calculation per gram and per specified portion", async () => {
            const user = userEvent.setup();
            const exampleProduct: GetProduct = () => {
                return {
                    data: {
                        id: 1,
                        name: "Test Product",
                        unit: Unit.Grams,
                        nutrients: TEST_100G_NUTRIENTS,
                        spi: 8
                    }
                }
            };
            const examplePortions: GetProductPortions = () => {
                return {
                    portions: [
                        {
                            id: 1,
                            product: 1,
                            name: "Serving",
                            grams: 1000
                        }
                    ]
                }
            }


            const page = renderPage(exampleProduct, examplePortions);

            const { findByText, getByLabelText } = page;

            expect(await findByText("Test Product")).toBeInTheDocument();

            await nutrientsAreShown(page, TEST_100G_NUTRIENTS);

            await user.clear(getByLabelText("per-input"));
            await user.type(getByLabelText("per-input"), "1");


            await user.click(getByLabelText("select-unit"));
            await user.click(await findByText("Serving"));

            await nutrientsAreShown(page, TEST_SERVING_NUTRIENTS);

        })
    })
})