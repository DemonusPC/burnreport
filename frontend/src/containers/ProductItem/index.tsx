import React from "react";
import styled from "styled-components";
import {
  Button,
  Box,
  TextInput,
  Text,
  Select,
} from "grommet";
import { Close } from "grommet-icons";
import { ProductSize } from "../../util/schema/product";

const PerWrapper = styled(Box)`
  align-items: center;
`;

const shortName = (name: string) : string => {
  const max = 28
  if(name.length > max) {
    return name.slice(0, max - 3) + "...";
  }
  return name;
}

interface ProductItemProps {
  id: number;
  name: string;
  amount: number;
  changeFunc: (event: any) => void;
  deleteFunc: () => void;
  unit: ProductSize;
  unitOptions: Array<ProductSize>;
  setUnit: any;
}

const ProductItem = ({
  id,
  name,
  amount,
  changeFunc,
  deleteFunc,
  unit,
  unitOptions,
  setUnit
}: ProductItemProps) => {

  return (
    <Box elevation="medium" pad="medium" margin={{bottom: "large"}} border="all" key={id}>
      <Box  direction="row" alignContent="center" justify="between">
        <Text weight="bold">{shortName(name)}</Text>
        <Button
          plain={true}
          icon={<Close />}
          onClick={deleteFunc}
        />
        </Box>

          <PerWrapper
            direction="row"
            alignContent="center"
            justify="center"
            margin={{top: "medium"}}
          >
            <TextInput placeholder="0.0" value={amount} onChange={changeFunc} />

            <Select
              name="select"
              placeholder="Select"
              labelKey="name"
              value={unit}
              options={unitOptions}
              onChange={({ option }) => {
                setUnit(option);
              }}
            />
          </PerWrapper>

    </Box>
  );
};

export default ProductItem;
