import React, { useState } from "react";
import {
  Heading,
  Box,
  TextInput
} from "grommet";
import SearchForm from "../../containers/SearchForm";
import { emptyProduct, Product } from "../../util/data/product";

const calculatePer = (value: number, per: number): number => {
  const one = value / 100;
  return one * per;
}

const fetchProducts = async (suggestion: string) => {  
  const request =  await fetch(encodeURI(`http://localhost:8080/search?p=${suggestion}`), {
    headers: {
      'origin': 'localhost'
    }
  });
  const json = await request.json();
  return json;
}

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
      <Box pad="large">
        <Box>
          <Heading>Product Search</Heading>
        </Box>
        <Box>
          <SearchForm
            selectedFunction={(product: Product) => {
              console.log(product);
              setState({ ...state, selected: product });
            }}
            suggestFunction={fetchProducts}
          />
        </Box>

        <Box>
          <Heading level={2}>{state.selected.name}</Heading>
          <Box direction="row">
            <Heading level={3}>Per           
          <TextInput
            placeholder="100"
            value={state.per}
            onChange={onChange}
          />g</Heading>

            

          </Box>
          <Box>
            <Heading level={3}>Energy</Heading>
            <div>
              {calculatePer(state.selected.energy.kcal, state.per )} kcal
            </div>
            <div>
              {calculatePer(state.selected.energy.kj, state.per)} kJ
            </div>
          </Box>
          <Box>
            <Heading level={3}>Carbohydrates</Heading>
            <div>
              {calculatePer(state.selected.carbohydrates.total, state.per )} g
            </div>
            <div>
              of which sugar {calculatePer(state.selected.carbohydrates.sugar, state.per )} g
            </div>
            <div>
              added sugar {calculatePer(state.selected.carbohydrates.addedSugar, state.per )} g
            </div>
            <div>
            starch {calculatePer(state.selected.carbohydrates.starch, state.per )} g
            </div>
            <div>
            fiber {calculatePer(state.selected.carbohydrates.fiber, state.per )} g
            </div>
          </Box>
          <Box>
            <Heading level={3}>Fat</Heading>
            <div>
              {calculatePer(state.selected.fat.total, state.per )} g
            </div>
            <div>
              of which saturated {calculatePer(state.selected.fat.saturated, state.per )} g
            </div>
            <div>
              of which monounsaturated {calculatePer(state.selected.fat.monounsaturated, state.per )} g
            </div>
            <div>
              of which trans {calculatePer(state.selected.fat.trans, state.per )} g
            </div>
          </Box>

          <Box>
            <Heading level={3}>Protein</Heading>
            <div>
              {calculatePer(state.selected.protein.total, state.per )} g
            </div>
          </Box>
          <Box>
            <Heading level={3}>Salt</Heading>
            <div>
              {calculatePer(state.selected.salt.total, state.per )} g
            </div>
          </Box>
        </Box>
      </Box>
  );
};

export default Products;
