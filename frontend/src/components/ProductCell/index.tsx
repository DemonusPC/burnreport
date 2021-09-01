import React from "react";
import { Product } from "../../util/schema/product";
import AnchorLink from "../AnchorLink";

const ProductCell = ({ id, name }: Product) => {
  return (
    <AnchorLink
      to={encodeURI(`/products/${id}`)}
      margin={{ top: "samll", bottom: "small" }}
      size="large"
      label={name}
    />
  );
};

export default ProductCell;
