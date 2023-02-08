import React, { useState } from "react";
import { Box, TextInput } from "grommet";
import { SearchSuggestion } from "../ProductSearchForm";

interface SearchProps {
  suggestFunction: (suggestion: string) => Promise<any>;
  fieldName?: string;
}

const buildSuggestionStringArray = (): string[] => {
  return [];
};

const emptyOriginal = (): SearchSuggestion[] => {
  return [];
};

const confirmProduct = (name: string, original: SearchSuggestion[]) => {
  const found = original.find((element) => element.text === name) || undefined;
  return found;
};

type SearchFormState = {
  value: string;
  selectedProduct: SearchSuggestion | undefined;
};
const SpiSelect = ({
  suggestFunction,
  fieldName = "search",
}: SearchProps) => {
  const [state, setState] = useState<SearchFormState>({
    value: "",
    selectedProduct: undefined,
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

    suggestFunction(escapedText).then((json: Array<SearchSuggestion>) => {
      const suggestions = json.map((product: SearchSuggestion) => {
        return product.text;
      });
      setSug({ suggestions, original: json });
    });

    setState({ ...state, value });
  };

  const onSelect = (event: { suggestion: any }) => {
    const selected = confirmProduct(event.suggestion, sug.original);
    if (selected) {
      setState({
        ...state,
        value: `${selected.id}::${selected.text}`,
        selectedProduct: selected,
      });
    }
  };

  return (
    <Box width="large">
      <TextInput
        plain
        name={fieldName}
        value={state.value}
        onChange={onChange}
        onSuggestionSelect={onSelect}
        suggestions={sug.suggestions}
      />
    </Box>
  );
};

export default SpiSelect;
