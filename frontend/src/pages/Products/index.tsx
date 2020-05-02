import React, { useState } from "react";
import {
  Heading,
  Box,
  TextInput
} from "grommet";
import styled from 'styled-components';
import SearchForm from "../../containers/SearchForm";
import { emptyProduct, Product } from "../../util/schema/product";
import { getProductSearch } from "../../util/data/requests";
import { calculatePer, displayRound } from "../../util/data/calculations";
import NutrientTable from "../../containers/NutrientTable";
import NutrientBar from "../../containers/NutrientBar";

const PerWrapper = styled(Box)`
  align-items: center;
  max-width: 15em;
`;

const totalMacroInGrams = (product: Product) => {
  const carbs = product.carbohydrates.total;
  const fat = product.fat.total;
  const protein = product.protein.total;

  const total = carbs + fat + protein;

  return total;
}

const calculate = (value: number, per: number): number => {
  const result = calculatePer(value, per);
  return displayRound(result);
} 

const ProductName = styled(Heading)`
  width: 500px;
`;

const Energy = styled(Heading)`
  font-size: 2em;
`;

const Products = () => {
  const [state, setState] = useState({
    selected: emptyProduct(),
    per: 100
  });

  const onChange = (event: { target: { value: any; }; }) => {
    const {
      target: { value }
    } = event;

    setState({ ...state, per: value });
  };
  return (
      <Box pad="large" align="center" >
        <Box>
          <Heading>Product Search</Heading>
        </Box>
        <Box pad={{
          vertical: "medium"
        }} >
          <SearchForm
            selectedFunction={(product: Product) => {
              console.log(product);
              setState({ ...state, selected: product });
            }}
            suggestFunction={getProductSearch}
          />
        </Box>
        <Box margin = {{
          top: "medium"
        }}>
          <PerWrapper fill={false} direction="row" alignContent="center" justify="center">
            <Heading margin={{right: "medium"}} level={3}>Per </Heading>           
          <TextInput
            placeholder="100"
            value={state.per}
            onChange={onChange}
          />
           <Heading margin={{left: "small"}} level={3}>g</Heading>
          </PerWrapper>
          <Box>
            <ProductName level={2}>{state.selected.name}</ProductName>
            <NutrientBar total={totalMacroInGrams(state.selected)} carbohydrates={state.selected.carbohydrates.total} fat={state.selected.fat.total} protein={state.selected.protein.total}  />
            <Energy level={4}> {calculate(state.selected.energy.kcal, state.per )} kcal / {calculate(state.selected.energy.kj, state.per)} kJ</Energy>
            <NutrientTable product={state.selected} amount={state.per} />
          </Box>
            

        </Box>

      </Box>
  );
};

export default Products;
