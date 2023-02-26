import React from "react";
import { SearchEntity, SearchSuggestion } from "../../containers/ProductSearchForm";
import AnchorLink from "../AnchorLink";

const ProductCell = ({ id, text, entity }: SearchSuggestion) => {

  const urlEntity = entity === SearchEntity.Spi ? "spi" : "products";
  return (
    <AnchorLink
      to={encodeURI(`/${urlEntity}/${id}`)}
      margin={{ top: "samll", bottom: "small" }}
      size="large"
      label={text}
    />
  );
};

export default ProductCell;
