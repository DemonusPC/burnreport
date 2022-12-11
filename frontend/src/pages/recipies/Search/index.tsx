import { Box, Heading } from 'grommet'
import React from 'react'
import AnchorLink from '../../../components/AnchorLink'
import ProductSearchForm, { SearchSuggestion } from '../../../containers/ProductSearchForm'
import { ResultList } from '../../../util/data/requests'

export const getRecipieSearchSuggestions = async (
  suggestion: string
): Promise<SearchSuggestion[]> => {
  const request = await fetch(
    encodeURI(`/api/search/recipie/suggestions?p=${encodeURI(suggestion)}`)
  )

  const result: ResultList<SearchSuggestion> = await request.json()

  return result.result
}

export const recipieListUrl = (value: string) => {
  return `/recipies/list?p=${encodeURI(value)}`;
}

const RecipieSearch = (): JSX.Element => {
  return (
    <Box pad="large" gridArea='main'>
      <AnchorLink to="/recipies/add" label="Add Recipies" />
      <Box>
        <Heading size='small'>Recipie Search</Heading>
      </Box>
      <Box
        pad={{
          vertical: 'medium'
        }}
        width="large"
      >
        <ProductSearchForm getSuggestions={getRecipieSearchSuggestions} getSubmitUrl={recipieListUrl} />
      </Box>
    </Box>
  )
}

export default RecipieSearch
