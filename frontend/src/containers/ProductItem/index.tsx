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
          <Text>
            <Button
              plain={false}
              icon={<Close />}
              onClick={deleteFunc}
              color="status-critical"
            />
          </Text>
        </TableCell>
      </TableRow>
  );
};

// return (
//     <Box direction="row">
//         <div>{name}</div>
//         <Box>            <TextInput
//         placeholder="type here"
//         value={amount}
//         onChange={changeFunc}
//         /></Box>

//         <Button plain={false} icon={<Close />} onClick={deleteFunc} color="status-critical" />
//     </Box>
// )

export default ProductItem;
