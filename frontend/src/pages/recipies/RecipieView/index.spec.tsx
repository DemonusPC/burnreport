import React from 'react'
import RecipieView from '.'


import { fireEvent, render, RenderResult } from '@testing-library/react';
import { MemoryRouter, Route } from 'react-router-dom';

const renderPage = (): RenderResult => {
    const result = render(
        <MemoryRouter initialEntries={['/recipies/1']}>
            <Route path="/recipies/:id">
                <RecipieView />
            </Route>
        </MemoryRouter>
    )

    return result;
}

describe("Given I view the recipie view page", () => {
    it("should display the recipie, with the product list", async () => {

        const { findByText, getByText } = renderPage();

        expect(await findByText("Super Chicken")).toBeInTheDocument();

        const toFind = [
            { label: "carbohydrates", value: "10 g" },
            { label: "sugar", value: "3 g" },
            { label: "fat", value: "20 g" },
            { label: "saturated", value: "11 g" },
            { label: "protein", value: "5.2 g" },
            { label: "salt", value: "0.01 g" },
        ]

        toFind.forEach(({ label, value }) => {
            expect(getByText(label)).toBeInTheDocument();
            expect(getByText(value)).toBeInTheDocument();
        });

        fireEvent.click(getByText("Vitamins"));

        expect(await findByText("b1")).toBeInTheDocument();
        expect(await findByText("0.12 mg")).toBeInTheDocument();


    })
})