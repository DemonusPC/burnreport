import React from "react";
import { SearchSuggestion } from "../../containers/ProductSearchForm";
import AnchorLink from "../AnchorLink";

const ProductCell = ({ id, text }: SearchSuggestion) => {
  return (
    <AnchorLink
      to={encodeURI(`/products/${id}`)}
      margin={{ top: "samll", bottom: "small" }}
      size="large"
      label={text}
    />
  );
};

export default ProductCell;
