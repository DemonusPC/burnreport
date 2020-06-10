import React from "react";
import styled from "styled-components";
import {
  Button,
  Box,
  TextInput,
  TableRow,
  TableCell,
  Text,
  Select,
} from "grommet";
import { Close } from "grommet-icons";
import { ProductSize } from "../../util/schema/product";
import { getProductSizesById } from "../../util/data/requests";

const PerWrapper = styled(Box)`
  align-items: center;
  /* max-width: 15em; */
`;

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

const base: ProductSize = {
  id: 0,
  product: 0,
  name: "grams",
  grams: 1,
};

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

  // useEffect(() => {
  //   const fetchAndSet = async () => {
  //     const serverPortions = await getProductSizesById(id);

  //     setPortions({ unitOptions: [base].concat(serverPortions) });
  //   };

  //   fetchAndSet();
  // }, [id]);

  return (
    <TableRow key={id}>
      <TableCell>
        <Text>{name}</Text>
      </TableCell>
      <TableCell>
        <Text>
          <PerWrapper
            fill={false}
            direction="row"
            alignContent="center"
            justify="center"
          >
            <TextInput placeholder="0.0" value={amount} onChange={changeFunc} />

            <Select
              name="select"
              placeholder="Select"
              labelKey="name"
              value={unit}
              options={unitOptions}
              onChange={({ option }) => {
                // setState({ ...state, unit: option });
                setUnit(option);
              }}
            />
          </PerWrapper>
        </Text>
      </TableCell>
      <TableCell>
        <Button
          plain={false}
          size="small"
          icon={<Close />}
          onClick={deleteFunc}
          color="status-critical"
        />
      </TableCell>
    </TableRow>
  );
};

export default ProductItem;
