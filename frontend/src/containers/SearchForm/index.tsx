import React, { useState } from "react";
import { Box, TextInput } from "grommet";
import { Search } from "grommet-icons";
import { emptyProduct, Product } from "../../util/schema/product";

interface SearchProps {
  selectedFunction: (product: Product) => void;
  suggestFunction: (suggestion: string) => Promise<any>;
}

const buildSuggestionStringArray = (): string[] => {
  return [];
};

const emptyOriginal = (): Product[] => {
  return [];
};

const confirmProduct = (name: string, original: Product[]) => {
  const found =
    original.find((element) => element.name === name) || emptyProduct();
  return found;
};

const SearchForm = ({ selectedFunction, suggestFunction }: SearchProps) => {
  const [state, setState] = useState({
    value: "",
    selectedProduct: emptyProduct(),
  });
  const [sug, setSug] = useState({
    suggestions: buildSuggestionStringArray(),
    original: emptyOriginal(),
  });

  const onChange = (event: { target: { value: any } }) => {
    const {
      target: { value },
    } = event;
    // The line below escapes regular expression special characters:
    // [ \ ^ $ . | ? * + ( )
    const escapedText = value.replace(/[-\\^$*+?.()|[\]{}]/g, "\\$&");

    suggestFunction(escapedText).then((json: Array<Product>) => {
      const suggestions = json.map((product: Product) => {
        return product.name;
      });
      setSug({ suggestions, original: json });
    });

    setState({ ...state, value });
  };

  const onSelect = (event: { suggestion: any }) => {
    const selected = confirmProduct(event.suggestion, sug.original);
    setState({
      ...state,
      value: event.suggestion,
      selectedProduct: selected,
    });
    selectedFunction(selected);
  };

  return (
    <Box width="large">
      <TextInput
        icon={<Search />}
        value={state.value}
        onChange={onChange}
        onSuggestionSelect={onSelect}
        suggestions={sug.suggestions}
      />
    </Box>
  );
};

export default SearchForm;
