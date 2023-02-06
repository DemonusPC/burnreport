import React from 'react'
import { Box, Button, Form, TextInput } from 'grommet'
import { Search } from 'grommet-icons'
import { getProductSearchSuggestions } from '../../util/data/requests'
import { useHistory } from 'react-router'

export enum SearchEntity {
  Product = "product",
  Recipie = "recipie",
  Spi = "spi"
}
export interface SearchSuggestion {
  id: number
  text: string
  subText?: string
  entity: SearchEntity
}

interface Sug {
  text: string
}

interface SearchFormProps {
  getSuggestions?: (queryText: string) => Promise<SearchSuggestion[]>;
  getSubmitUrl?: (value: string) => string;
  initialText?: string
}

const productDefault = (value: string) => {
  return `/products/list?p=${encodeURI(value)}`;
}
const ProductSearchForm = ({ initialText, getSuggestions = getProductSearchSuggestions, getSubmitUrl = productDefault }: SearchFormProps): JSX.Element => {
  const history = useHistory()
  const [value, setValue] = React.useState<Sug>({ text: initialText ?? '' })
  const [suggestions, setSuggestions] = React.useState<string[]>([])

  React.useEffect(() => {
    // The line below escapes regular expression special characters:
    // [ \ ^ $ . | ? * + ( )
    const escapedText = value.text.replace(
      /[-\\^$*+?.()|[\]{}]/g,
      '\\$&'
    )
    getSuggestions(escapedText).then(
      (json: SearchSuggestion[]) => {
        const newSuggestions = json.map((s: SearchSuggestion) => {
          return s.text
        })
        setSuggestions(newSuggestions)
      }
    ).catch(() => {
      setSuggestions([])
    })
  }, [])


  return (
    <Form
      value={value}
      onChange={(nextValue) => {
        setValue(nextValue)
      }}
      onReset={() => setValue({ text: '' })}
      onSubmit={({ value }) => {
        history.push(getSubmitUrl(value.text))
      }}
    >
      <TextInput aria-label={'search-input'} name="text" icon={<Search />} suggestions={suggestions} />
      <Box direction="row" gap="medium" margin={{ top: '1em' }}>
        <Button type="submit" primary label="Search" />
      </Box>
    </Form>
  )
}

export default ProductSearchForm
