import React, { useState } from "react";
import { Heading, Box, TextInput, Button, Anchor, Select } from "grommet";
import styled from "styled-components";
import SearchForm from "../../containers/SearchForm";
import { emptyProduct, Product, ProductSize } from "../../util/schema/product";
import { getProductSearch, deleteProduct, getProductSizesById } from "../../util/data/requests";
import { calculatePer, displayRound } from "../../util/data/calculations";
import NutrientTable from "../../containers/NutrientTable";
import NutrientBar from "../../containers/NutrientBar";

const PerWrapper = styled(Box)`
  align-items: center;
  /* max-width: 15em; */
`;

export const totalMacroInGrams = (product: Product) => {
  const carbs = product.carbohydrates.total;
  const fat = product.fat.total;
  const protein = product.protein.total;

  const total = carbs + fat + protein;

  return total;
};

const calculate = (value: number, per: number, baseUnit: number): number => {
  const result = calculatePer(value, per, baseUnit);
  return displayRound(result);
};

const ProductName = styled(Heading)`
  width: 500px;
`;

const Energy = styled(Heading)`
  font-size: 2em;
`;

const base : ProductSize = {
  id: 0,
  product: 0,
  name: "grams",
  grams: 1,
};

const urlToPortion = (id: number): string => {
  return encodeURI(`/products/${id}/portions/add`);
}

const Products = () => {
  const [state, setState] = useState({
    selected: emptyProduct(),
    per: 100,
    unit: base,
    unitOptions: [base],
  });

  const onChange = (event: { target: { value: any } }) => {
    const {
      target: { value },
    } = event;

    setState({ ...state, per: value });
  };


  return (
    <Box pad="large" align="center">
      <Box>
        <Heading>Product Search</Heading>
      </Box>
      <Box
        pad={{
          vertical: "medium",
        }}
      >
        <SearchForm
          selectedFunction={(product: Product) => {
            // This is really ugly. Forces like 8 rerenders
            // TODO: This needs fixing
            setState({ ...state, selected: product });
            getProductSizesById(product.id).then((result) => setState({...state, selected: product, unitOptions: [base].concat(result)}) );
          }}
          suggestFunction={getProductSearch}
        />
      </Box>
      <Box
        margin={{
          top: "medium",
        }}
      >
        <PerWrapper
          fill={false}
          direction="row"
          alignContent="center"
          justify="center"
        >
          <Heading margin={{ right: "medium" }} level={3}>
            Per{" "}
          </Heading>
          <TextInput placeholder="100" value={state.per} onChange={onChange} />

          <Select
            name="select"
            placeholder="Select"
            labelKey="name"
            value={state.unit}
            options={state.unitOptions}
            onChange={({ option }) => {
              setState({ ...state, unit: option });
            }}
          />
        </PerWrapper>

        <Box>
          <ProductName level={2}>{state.selected.name}</ProductName>
          <NutrientBar
            total={totalMacroInGrams(state.selected)}
            carbohydrates={state.selected.carbohydrates.total}
            fat={state.selected.fat.total}
            protein={state.selected.protein.total}
          />
          <Energy level={4}>
            {" "}
            {calculate(
              state.selected.energy.kcal,
              state.per,
              state.unit.grams
            )}{" "}
            kcal /{" "}
            {calculate(state.selected.energy.kj, state.per, state.unit.grams)}{" "}
            kJ
          </Energy>
          <NutrientTable
            product={state.selected}
            amount={state.per}
            baseUnit={state.unit.grams}
          />
        </Box>

        <Box
          justify="between"
          alignContent="between"
          direction="row"
          margin={{ top: "xlarge" }}
        >
          <Anchor href="/products/add" label="Add Product" key="addproduct" />
          {state.selected.id !== 0 && (
            <>
            <Anchor href={urlToPortion(state.selected.id)} label="Add Portion" key="addproduct" />
            <Button
              fill={false}
              color="status-critical"
              type="button"
              label="Delete Product"
              onClick={() => {
                deleteProduct(state.selected.id).then((status) => {
                  setState({
                    selected: emptyProduct(),
                    per: 100,
                    unit: base,
                    unitOptions: [base],
                  });
                });
              }}
            />
            </>
          )}
        </Box>
      </Box>
    </Box>
  );
};

export default Products;
