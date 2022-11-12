import React from "react";
import { Box, Button, Form, TextInput } from "grommet";
import { Search } from "grommet-icons";
import { getProductSearchSuggestions } from "../../util/data/requests";
import { useHistory } from "react-router";

export interface SearchSuggestion {
  id: number;
  text: string;
  subText?: string;
  entity?: string;
}

interface Sug {
  text: string;
}

interface SearchFormProps {
  getSuggestions: (queryText: string) => Promise<SearchSuggestion[]>;
  initialText?: string;
}

const ProductSearchForm = ({ initialText, getSuggestions = getProductSearchSuggestions }: SearchFormProps): JSX.Element => {
  const history = useHistory();
  const [value, setValue] = React.useState<Sug>({ text: initialText || "" });
  const [suggestions, setSuggestions] = React.useState<Array<string>>([]);
  return (
    <Form
      value={value}
      onChange={(nextValue) => {
        // The line below escapes regular expression special characters:
        // [ \ ^ $ . | ? * + ( )
        const escapedText = nextValue.text.replace(
          /[-\\^$*+?.()|[\]{}]/g,
          "\\$&"
        );
        getSuggestions(escapedText).then(
          (json: Array<SearchSuggestion>) => {
            const newSuggestions = json.map((s: SearchSuggestion) => {
              return s.text;
            });
            setSuggestions(newSuggestions);
          }
        );
        setValue(nextValue);
      }}
      onReset={() => setValue({ text: "" })}
      onSubmit={({ value }) => {
        history.push(`/products/list?p=${encodeURI(value.text)}`);
      }}
    >
      <TextInput name="text" icon={<Search />} suggestions={suggestions} />
      <Box direction="row" gap="medium" margin={{ top: "1em" }}>
        <Button type="submit" primary label="Search" />
      </Box>
    </Form>
  );
};

export default ProductSearchForm;
