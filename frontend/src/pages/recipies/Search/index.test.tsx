import React from 'react'
import { render } from '@testing-library/react'
import useEvent from '@testing-library/user-event'
import RecipieSearch from '.'

describe('Given a user navigates to the Recipie Search Page', () => {
  describe('When they search for a word', () => {
    it('they can select a value and see a list of all possible search values with links to recipies', async () => {
      const user = useEvent.setup()
      const renderResult = render(<RecipieSearch />)

      const { findByText, getByLabelText } = renderResult

      expect(await findByText('Recipie Search')).toBeInTheDocument()

      const searchInput = getByLabelText('search-input')

      await user.type(searchInput, 'Super')

      const l = await findByText('Super Chicken')
      expect(l).toBeInTheDocument()
    })
  })
})
