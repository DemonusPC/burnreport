import { Anchor, Box, Button, Heading, Select, TextInput } from "grommet";
import React from "react";
import { useParams } from "react-router";
import styled from "styled-components";
import useSWR from "swr";
import NutrientBar from "../../containers/NutrientBar";
import NutrientTable from "../../containers/NutrientTable";
import { calculatePer, displayRound } from "../../util/data/calculations";
import { deleteProduct, ResultList } from "../../util/data/requests";
import { Product, ProductSize } from "../../util/schema/product";
import {Return} from 'grommet-icons'; 

export const totalMacroInGrams = (product: Product) => {
  const carbs = product.carbohydrates.total;
  const fat = product.fat.total;
  const protein = product.protein.total;

  const total = carbs + fat + protein;

  return total;
};

const PerWrapper = styled(Box)`
  align-items: center;
  /* max-width: 15em; */
`;

const urlToPortion = (id: number): string => {
  return encodeURI(`/products/${id}/portions`);
};

const calculate = (value: number, per: number, baseUnit: number): number => {
  const result = calculatePer(value, per, baseUnit);
  return displayRound(result);
};

const base: ProductSize = {
  id: 0,
  product: 0,
  name: "grams",
  grams: 1,
};

interface ProductParams {
  id: string;
}

const ProductPage = () => {
  const params: ProductParams = useParams<ProductParams>();
  const parsed = Number.parseInt(params.id);
  const [state, setState] = React.useState({
    per: 100,
    unit: base,
    unitOptions: [base],
  });

  const { data, error } = useSWR<Product>(encodeURI(`/api/products/${parsed}`));
  const portions = useSWR<ResultList<ProductSize>>(
    encodeURI(`/api/products/${parsed}/portions`)
  );

  if (error || portions.error) return <div>Error</div>;
  if (!data || !portions.data) return <div>loading...</div>;

  const availablePortions = [base].concat(portions.data.result);

  return (
    <Box pad="large" align="center">
    <Box direction="row" align="center" alignContent="between" justify="around" gap="large">
        <Button type="button" label="Back" icon={<Return />} onClick={() => {
            window.history.back();
        }} />
        <Anchor href="/products/add" label="Add Product" key="addproduct" />
        
    </Box>
      <Box>
        <Heading level={2}>{data.name}</Heading>
        <NutrientBar
          total={totalMacroInGrams(data)}
          carbohydrates={data.carbohydrates.total}
          fat={data.fat.total}
          protein={data.protein.total}
        />
        <Heading level={4}>
          {calculate(data.energy.kcal, state.per, state.unit.grams)}
          kcal /{calculate(data.energy.kj, state.per, state.unit.grams)}
          kJ
        </Heading>

        <PerWrapper
          fill={false}
          direction="row"
          alignContent="center"
          justify="center"
        >
          <Heading margin={{ right: "medium" }} level={3}>
            Per
          </Heading>
          <TextInput
            placeholder="100"
            value={state.per}
            onChange={(event: { target: { value: any } }) => {
              const {
                target: { value },
              } = event;

              setState({ ...state, per: value });
            }}
          />

          <Select
            name="select"
            placeholder="Select"
            labelKey="name"
            value={state.unit}
            options={availablePortions}
            onChange={({ option }) => {
              setState({ ...state, unit: option });
            }}
          />
        </PerWrapper>

        <NutrientTable
          product={data}
          amount={state.per}
          baseUnit={state.unit.grams}
        />
      </Box>

      <Box
        justify="between"
        alignContent="center"
        direction="row"
        margin={{ top: "xlarge" }}
        gap="large"
      >
          <Anchor
            href={urlToPortion(data.id)}
            label="Portions"
            key="addproduct"
          />
          <Button
            fill={false}
            color="status-critical"
            type="button"
            label="Delete Product"
            onClick={() => {
              deleteProduct(data.id).then((status) => {});
            }}
          />
      </Box>
    </Box>
  );
};

export default ProductPage;
