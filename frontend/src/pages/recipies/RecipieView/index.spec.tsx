import React from 'react'
import RecipieView from '.'
import { render, RenderResult } from '@testing-library/react';
import { MemoryRouter, Route } from 'react-router-dom';
import { GetRecipie } from './recipieApi';
import { TEST_100G_NUTRIENTS } from '../../ProductPage/index.spec';
import { nutrientsAreShown } from '../../../containers/NutrientTable/index.spec';

const renderPage = (fetcher: GetRecipie): RenderResult => {
    const result = render(
        <MemoryRouter initialEntries={['/recipies/1']}>
            <Route path="/recipies/:id">
                <RecipieView recipieFetcher={fetcher} />
            </Route>
        </MemoryRouter>
    )

    return result;
}

describe("Given I view the recipie view page", () => {
    it("should display the recipie, with the product list", async () => {

        const exampleRecipie: GetRecipie = () => {
            return {
                recipie: {
                    id: 1,
                    name: "Cool recipie",
                    total: TEST_100G_NUTRIENTS,
                    // Not implemented
                    ingredients: []
                }
            }
        }

        const page = renderPage(exampleRecipie);
        const { findByText } = page;

        expect(await findByText("Cool recipie")).toBeInTheDocument();

        await nutrientsAreShown(page, TEST_100G_NUTRIENTS);



    })
})