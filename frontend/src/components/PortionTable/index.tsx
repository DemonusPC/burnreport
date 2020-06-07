import React from 'react';
import styled from 'styled-components';
import { Table, TableHeader, TableRow, TableCell, TableBody, Text, Button } from 'grommet';
import { Portion } from '../../util/schema/product';
import { Close } from "grommet-icons";

const FullTAble = styled(Table)`
  width: 100%;
`;

const mapPortionItems = (portions: Portion[], setState: any, productId: number, removeFunction: any) => {
    return portions.map((portion) => (
      <TableRow key={portion.name}>
        <TableCell>
          <Text>{portion.name}</Text>
        </TableCell>
        <TableCell>
          <Text>{portion.grams}</Text>
        </TableCell>
        <TableCell>
          <Button
            plain={false}
            size="small"
            icon={<Close />}
            onClick={() => {
              const state = removeFunction(portions, portion.name, productId);
              setState(state);
            }}
            color="status-critical"
          />
        </TableCell>
      </TableRow>
    ));
  };

  interface PortionTableProps {
      productId: number,
      portions: Array<Portion>,
      stateSetter: any,
      stateReducer: any,
  }

const PortionTable = ({productId, portions, stateSetter, stateReducer} : PortionTableProps) => {


    return (
        <FullTAble>
        <TableHeader>
          <TableRow>
            <TableCell key={"name"} scope="col">
              <Text>Name</Text>
            </TableCell>
            <TableCell key={"amount"} scope="col">
              <Text>portion size in grams</Text>
            </TableCell>
            <TableCell key={"delete"} scope="col">
              <Text></Text>
            </TableCell>
          </TableRow>
        </TableHeader>
        <TableBody>{mapPortionItems(portions, stateSetter, productId, stateReducer)}</TableBody>
      </FullTAble>
    )
};

export default PortionTable;