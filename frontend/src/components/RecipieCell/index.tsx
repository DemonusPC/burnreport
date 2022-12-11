import React from "react";
import { SearchSuggestion } from "../../containers/ProductSearchForm";
import AnchorLink from "../AnchorLink";

// TODO: I mean its a bad copy and paste.
// Search should become its own component
const RecipieCell = ({ id, text }: SearchSuggestion) => {
  return (
    <AnchorLink
      to={encodeURI(`/recipies/${id}`)}
      margin={{ top: "samll", bottom: "small" }}
      size="large"
      label={text}
    />
  );
};

export default RecipieCell;
