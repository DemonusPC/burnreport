import React from 'react'
import { RenderResult, render } from '@testing-library/react'
import useEvent from '@testing-library/user-event'
import RecipieSearch from '.'
import { GetSearch } from '../../product/productApi'
import { MemoryRouter, Route } from 'react-router-dom'
import { SearchEntity } from '../../../containers/ProductSearchForm'
import RecipieList from '../RecipieList'
import { UseRecipieSearch } from '../recipieAPi/recipieApi'

const renderPage = (searchMock: GetSearch, recipieSearch: UseRecipieSearch): RenderResult => {
  const result = render(
    <MemoryRouter initialEntries={['/recipies']}>
      <Route path="/recipies">
        <RecipieSearch recipieSearch={searchMock} />
      </Route>
      <Route path="/recipies/list">
        <RecipieList useRecipie={recipieSearch} />
      </Route>
    </MemoryRouter>
  )

  return result;
}

const mockSearch: GetSearch = async () => {
  return [{ id: 1, text: "Cool recipie", entity: SearchEntity.Recipie }];
}

const mockUseRecipie: UseRecipieSearch = () => {
  return {
    recipie: [
      { id: 1, text: "Cool recipie", entity: SearchEntity.Recipie },
      { id: 2, text: "Cool recipie 2", entity: SearchEntity.Recipie }
    ]
  }
}

describe('Given a user navigates to the Recipie Search Page', () => {
  describe('When they search for a word', () => {
    it('they can select a value and see a list of all possible search values with links to recipies', async () => {
      const user = useEvent.setup()
      const renderResult = renderPage(mockSearch, mockUseRecipie);

      const { findByText, getByLabelText, getByText } = renderResult

      expect(await findByText('Recipie Search')).toBeInTheDocument()

      const searchInput = getByLabelText('search-input')

      await user.type(searchInput, 'Cool')

      const l = await findByText('Cool recipie')
      expect(l).toBeInTheDocument()

      const searchButton = getByText("Search");

      await user.click(searchButton);

      expect(await findByText("Cool recipie")).toBeInTheDocument();

      expect(getByText("Cool recipie 2")).toBeInTheDocument();


    })
  })
})
