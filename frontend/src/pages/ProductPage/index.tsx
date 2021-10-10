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
import useSWR from "swr";
import NutrientBar from "../../containers/NutrientBar";
import NutrientTable from "../../containers/NutrientTable";
import { calculateToDisplay } from "../../util/data/calculations";
import { deleteProduct, fetcher, ResultList } from "../../util/data/requests";
import { Product, ProductSize } from "../../util/schema/product";
import { Return } from "grommet-icons";
import AdditionalTable from "../../containers/AdditionalTable";
import { vitaminsToRow } from "../../util/schema/vitamins";
import { useHistory } from "react-router-dom";
import AnchorLink from "../../components/AnchorLink";

const PerWrapper = styled(Box)`
  align-items: center;
`;

const urlToPortion = (id: number): string => {
  return encodeURI(`/products/${id}/portions`);
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
  const history = useHistory();
  const params: ProductParams = useParams<ProductParams>();
  const parsed = Number.parseInt(params.id);
  const [state, setState] = React.useState({
    per: 100,
    unit: base,
    unitOptions: [base],
  });

  const { data, error } = useSWR<Product | null>(
    encodeURI(`/api/products/${parsed}`),
    fetcher
  );
  const portions = useSWR<ResultList<ProductSize>>(
    encodeURI(`/api/products/${parsed}/portions`)
  );

  if (error || portions.error) return <div>Error</div>;
  if (!data || !portions.data) return <div>loading...</div>;

  const availablePortions = [base].concat(portions.data.result);

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
        <NutrientBar nutrients={data.nutrients} />
        <Heading level={4}>
          {calculateToDisplay(
            data.nutrients.energy.kcal,
            state.per,
            state.unit.grams
          )}
          kcal /
          {calculateToDisplay(
            data.nutrients.energy.kj,
            state.per,
            state.unit.grams
          )}
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
