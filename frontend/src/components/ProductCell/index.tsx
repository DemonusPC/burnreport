import { Anchor } from "grommet";
import React from "react";
import { Link } from "react-router-dom";
import { Product } from "../../util/schema/product";

const ProductCell = ({ id, name }: Product) => {
  return (
    <>
      <Link to={encodeURI(`/products/${id}`)}>
        <Anchor
          margin={{ top: "samll", bottom: "small" }}
          size="large"
          as="div"
          label={name}
        />
      </Link>
    </>
  );
};

export default ProductCell;
