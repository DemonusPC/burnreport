import { Box, Heading } from 'grommet'
import React from 'react'
import AnchorLink from '../../../components/AnchorLink'
import ProductSearchForm, { SearchSuggestion } from '../../../containers/ProductSearchForm'
import { ResultList } from '../../../util/data/requests'
import { GetSearch } from '../../product/productApi'

export const getRecipieSearchSuggestions: GetSearch = async (
  suggestion: string
): Promise<SearchSuggestion[]> => {
  const request = await fetch(
    encodeURI(`/api/search?p=${encodeURI(suggestion)}&e=recipie`)
  )

  const result: ResultList<SearchSuggestion> = await request.json()

  return result.result
}

export const recipieListUrl = (value: string) => {
  return `/recipies/list?p=${encodeURI(value)}`;
}

type RecipieSearchProps = {
  recipieSearch?: GetSearch
}

const RecipieSearch = ({ recipieSearch = getRecipieSearchSuggestions }: RecipieSearchProps): JSX.Element => {
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
        <ProductSearchForm getSuggestions={recipieSearch} getSubmitUrl={recipieListUrl} />
      </Box>
    </Box>
  )
}

export default RecipieSearch
