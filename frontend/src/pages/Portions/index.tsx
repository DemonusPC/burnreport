import React, { useState, useEffect } from "react";
import {
  Heading,
  Box,
  Button,
} from "grommet";

import { useParams } from "react-router";

import { Portion } from "../../util/schema/product";
import PortionForm from "../../containers/PortionForm";
import { Redirect } from "react-router-dom";
import { postPortions, getProductSizesById, deletePortion } from "../../util/data/requests";
import PortionTable from "../../components/PortionTable";

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

const removeAndDeletePortion = (state: Array<Portion>, name: string, productId: number, removeFunction: any) => {
  deletePortion(productId, name);
  const result = state.filter((p) => p.name !== name);
  return result;
  
}

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


const Portions = () => {
  const [state, setState] = useState(emptyState());
  const [current, setCurrent] = useState(emptyState());
  const [sent, setSent] = useState(false);
  const { id } = useParams();

  useEffect(() => {
    const fetchAndSet = async () => {
      const portions = await getProductSizesById(id);

      setCurrent(portions);
    }

    fetchAndSet();
  }, [id]);

  // console.log(state);
  return (
    <Box pad="large">
      <Box>
        <Heading>Current Portions</Heading>
        <PortionTable portions={current} stateSetter={setCurrent} productId={id} stateReducer={removeAndDeletePortion} />
      </Box>
      <Box>
        <Heading>Add Portion Sizes</Heading>
        <PortionTable portions={state} stateSetter={setState} productId={id} stateReducer={removePortion} />

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

export default Portions;
