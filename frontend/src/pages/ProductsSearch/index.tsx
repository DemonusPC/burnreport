import React from "react";
import { Heading, Box, Anchor } from "grommet";
import ProductSearchForm from "../../containers/ProductSearchForm";
import { Product } from "../../util/schema/product";
import { Link } from "react-router-dom";

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
      <Link to="/products/add">
        <Anchor as="span" label="Add Product" key="addproduct" />
      </Link>

      <Box>
        <Heading>Product Search</Heading>
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
