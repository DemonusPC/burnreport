import React from "react";
import { Heading, Box } from "grommet";
import ProductSearchForm from "../../containers/ProductSearchForm";
import { Product } from "../../util/schema/product";
import AnchorLink from "../../components/AnchorLink";

export const totalMacroInGrams = (product: Product) => {
  const carbs = product.carbohydrates.total;
  const fat = product.fat.total;
  const protein = product.protein.total;

  const total = carbs + fat + protein;

  return total;
};

const ProductsSearch = () => {
  return (
    <Box pad="large" gridArea="main">
      <AnchorLink to="/products/add" label="Add Product" />
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
