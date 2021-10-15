import React from "react";
import styled from "styled-components";
import { Button, Box, Text, Select, MaskedInput } from "grommet";
import { Close } from "grommet-icons";
import { ProductSize } from "../../product/product";
import { standardMask } from "../../util/schema/masks";

const PerWrapper = styled(Box)`
  align-items: center;
  background-color: transparent;
`;

const shortName = (name: string): string => {
  const max = 28;
  if (name.length > max) {
    return name.slice(0, max - 3) + "...";
  }
  return name;
};

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
  setUnit,
}: ProductItemProps) => {
  return (
    <Box
      pad="medium"
      margin={{ bottom: "large" }}
      border="all"
      key={id}
      round={"4px"}
    >
      <Box direction="row" alignContent="center" justify="between">
        <Text weight="bold">{shortName(name)}</Text>
        <Button plain={true} icon={<Close />} onClick={deleteFunc} />
      </Box>
      <PerWrapper
        direction="row"
        alignContent="center"
        justify="center"
        margin={{ top: "medium" }}
      >
        <MaskedInput
          mask={standardMask}
          defaultValue={100.0}
          onChange={changeFunc}
        />

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
