import React from "react";
import { Heading, Box } from "grommet";
import ProductSearchForm from "../../containers/ProductSearchForm";
import AnchorLink from "../../components/AnchorLink";

const ProductsSearch = (): JSX.Element => {
  return (
    <Box pad="large" gridArea="main">
      <AnchorLink to="/products/add" label="Add Product" />
      <AnchorLink to="/spi/add" label="Add Standard Product Identifier" />
      <Box>
        <Heading size="small">Product Search</Heading>
      </Box>
      <Box
        pad={{
          vertical: "medium",
        }}
        width="large"
      >
        <ProductSearchForm />
      </Box>
    </Box>
  );
};

export default ProductsSearch;
