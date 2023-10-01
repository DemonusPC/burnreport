import {
  Box,
  Button,
  Heading,
  Select,
  TextInput,
  Accordion,
  AccordionPanel,
} from "grommet";
import React from "react";
import { useParams } from "react-router";
import styled from "styled-components";
import NutrientTable from "../../containers/NutrientTable";
import { deleteProduct } from "../../util/data/requests";
import { Portion } from "../../product/product";
import { Return } from "grommet-icons";
import AdditionalTable from "../../containers/AdditionalTable";
import { vitaminsToRow } from "../../nutrients/vitamins";
import { useHistory } from "react-router-dom";
import AnchorLink from "../../components/AnchorLink";
import {
  nutrientsToBarTotal,
  nutrientsToBarValues,
} from "../../nutrients/nutrients";
import Bar from "../../containers/Bar";
import { UseProduct, UseProductPortions, useProduct, useProductPortions } from "../product/productApi";

const PerWrapper = styled(Box)`
  align-items: center;
`;

const urlToPortion = (id: number): string => {
  return encodeURI(`/products/${id}/portions`);
};

const base: Portion = {
  id: 0,
  product: 0,
  name: "grams",
  grams: 1,
};

interface ProductParams {
  id: string;
}

type ProductPageProps = {
  productFetcher?: UseProduct
  portionFetcher?: UseProductPortions
}

const ProductPage = ({ productFetcher = useProduct, portionFetcher = useProductPortions }: ProductPageProps) => {
  const history = useHistory();
  const params: ProductParams = useParams<ProductParams>();
  const parsed = Number.parseInt(params.id);
  const [state, setState] = React.useState({
    per: 100,
    unit: base,
    unitOptions: [base],
  });

  const { data, error } = productFetcher(parsed);

  const portions = portionFetcher(parsed);


  if (error || portions.error) return <div>Error</div>;
  if (!data || !portions.portions) return <div>loading...</div>;

  const availablePortions = portions.portions;

  return (
    <Box pad="large" gridArea="main">
      <Box
        direction="row"
        align="center"
        alignContent="between"
        justify="around"
        gap="large"
      >
        <Button
          type="button"
          label="Back"
          icon={<Return />}
          onClick={() => {
            window.history.back();
          }}
        />
        <AnchorLink to="/products/add" label="Add Product" />
      </Box>
      <Box>
        <Heading level={2}>{data.name}</Heading>
        <Bar
          data={data.nutrients}
          mapToBarValues={nutrientsToBarValues}
          calculateTotal={nutrientsToBarTotal}
        />
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
            aria-label="per-input"
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
            aria-label="select-unit"
            value={state.unit}
            options={availablePortions}
            onChange={({ option }) => {
              setState({ ...state, unit: option });
            }}
          />
        </PerWrapper>

        <NutrientTable
          nutrients={data.nutrients}
          amount={state.per}
          baseUnit={state.unit.grams}
        />
        <Accordion animate={true} multiple={false}>
          <AccordionPanel label="Vitamins">
            <AdditionalTable
              entity={data.nutrients.vitamins}
              mapper={vitaminsToRow}
              unit={"mg"}
            />
          </AccordionPanel>
        </Accordion>
      </Box>

      <Box
        justify="between"
        alignContent="center"
        direction="row"
        margin={{ top: "xlarge" }}
        gap="large"
      >
        <AnchorLink
          to={urlToPortion(data.id)}
          label="Portions"
          key="toPortions"
        />
        <Button
          fill={false}
          color="status-critical"
          type="button"
          label="Delete Product"
          onClick={async () => {
            await deleteProduct(data.id);
            history.push("/products");
          }}
        />
      </Box>
    </Box>
  );
};

export default ProductPage;
