import React from "react";
import { Button, TextInput, TableRow, TableCell, Text } from "grommet";
import { Close } from "grommet-icons";

interface ProductItemProps {
  id: number,
  name: string;
  amount: number;
  changeFunc: (event: any) => void;
  deleteFunc: () => void;
}

const ProductItem = ({
  id,
  name,
  amount,
  changeFunc,
  deleteFunc,
}: ProductItemProps) => {
  return (

      <TableRow key={id}>
        <TableCell>
          <Text>{name}</Text>
        </TableCell>
        <TableCell>
          <Text>
            <TextInput
              placeholder="type here"
              value={amount}
              onChange={changeFunc}
            />
          </Text>
        </TableCell>
        <TableCell>
            <Button
              plain={false}
              size = "small"
              icon={<Close />}
              onClick={deleteFunc}
              color="status-critical"
            />

        </TableCell>
      </TableRow>
  );
};

export default ProductItem;
