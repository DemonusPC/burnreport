import React, { useState } from "react";

import {
  Heading,
  Box,
  Table,
  TableHeader,
  TableRow,
  TableCell,
  TableBody,
  Text,
  Button,
} from "grommet";
import { Close } from "grommet-icons";

import { useParams } from "react-router";

import { Portion } from "../../util/schema/product";
import PortionForm from "../../containers/PortionForm";
import { Redirect } from "react-router-dom";
import { postPortions } from "../../util/data/requests";

const emptyState = (): Array<Portion> => {
  return [];
};

const addPortion = (
  state: Array<Portion>,
  portion: Portion
): Array<Portion> => {
  if (state.some((c) => c.name === portion.name)) {
    return state;
  }

  return state.concat([portion]);
};

const removePortion = (state: Array<Portion>, name: string) => {
  const result = state.filter((p) => p.name !== name);
  return result;
};

const mapPortionItems = (portions: Portion[], setState: any) => {
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
            const state = removePortion(portions, portion.name);
            setState(state);
          }}
          color="status-critical"
        />
      </TableCell>
    </TableRow>
  ));
};

const submit = (portions: Array<any>, setSent: any) => {
  console.log(portions);
  const parsed : Array<Portion> = portions.map((portion: any) => {
    return {
      product: parseInt(portion.product),
      name: portion.name,
      grams: parseFloat(portion.grams)
    }
  })

  postPortions(parsed).then((result: { status: any; }) => {
    console.log(result.status);
    setSent(true);
  });
}


const AddProductSize = () => {
  const [state, setState] = useState(emptyState());
  const [sent, setSent] = useState(false);
  const { id } = useParams();

  // console.log(state);
  return (
    <Box pad="large">
      <Box>
        <Heading>Add Portion Sizes</Heading>
        <Table>
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
          <TableBody>{mapPortionItems(state, setState)}</TableBody>
        </Table>

        <PortionForm
          product={id}
          selectedFunction={(portion: Portion) =>
            setState(addPortion(state, portion))
          }
        />
      </Box>
      <Box direction="row" gap="medium">
      <Button type="submit" label="Submit" onClick={() => submit(state, setSent) } />
      <Button type="reset" label="Clear" onClick={() => setState(emptyState())} />
      </Box>
      {sent && <Redirect to="/products" />}
    </Box>
  );
};

export default AddProductSize;
