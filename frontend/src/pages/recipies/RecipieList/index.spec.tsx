import { render, RenderResult } from '@testing-library/react'
import React from 'react'
import { MemoryRouter, Route } from 'react-router-dom'
import RecipieList from '.'

const renderSearchPage = (): RenderResult => {
    const result = render(
        <MemoryRouter initialEntries={['/recipies/list?p=Super Chicken']}>
            <Route path="/recipies/list">
                <RecipieList />
            </Route>
        </MemoryRouter>
    )

    return result;
}

describe("Given I view the Recipie List Page", () => {
    describe("when I am navigated to it with the selected search paramter", () => {
        it("should display me the results of that search", async () => {
            const { findByText } = renderSearchPage();

            const linkElement = await findByText("Super Chicken")

            expect(linkElement.nodeName).toEqual('A');
            expect(linkElement).toHaveAttribute("href", "/recipies/1")
        })
    })
})