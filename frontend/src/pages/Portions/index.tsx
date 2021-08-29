import React, { useState, useEffect } from "react";
import { Heading, Box, Button } from "grommet";
import { AddCircle } from "grommet-icons";

import { useParams } from "react-router";

import { Portion } from "../../util/schema/product";
import PortionForm from "../../containers/PortionForm";
import cogoToast from "cogo-toast";

import {
  postPortions,
  getProductSizesById,
  deletePortion,
} from "../../util/data/requests";
import PortionList from "../../components/PortionList";

interface IdParams {
  id: string;
}

const emptyState = (): Array<Portion> => {
  return [];
};

const removeAndDeletePortion = (
  state: Array<Portion>,
  name: string,
  productId: number,
  removeFunction: any
) => {
  deletePortion(productId, name);
  const result = state.filter((p) => p.name !== name);
  return result;
};

const submit = async (portions: Array<any>) => {
  const parsed: Array<Portion> = portions.map((portion: any) => {
    return {
      product: parseInt(portion.product),
      name: portion.name,
      grams: parseFloat(portion.grams),
    };
  });

  const result = await postPortions(parsed);
  return result;
};

const refreshPortions = async (id: number, setState: any) => {
  const portions = await getProductSizesById(id);
  setState(portions);
};

const Portions = () => {
  const [current, setCurrent] = useState(emptyState());
  const [adding, setAdding] = useState(false);
  const { id } = useParams<IdParams>();

  useEffect(() => {
    const fetchAndSet = async () => {
      const portions = await getProductSizesById(Number.parseInt(id));

      setCurrent(portions);
    };

    fetchAndSet();
  }, [id]);

  return (
    <>
      <Box pad="large" width="large">
        <Heading size="small">Portions</Heading>
        <PortionList
          portions={current}
          setState={setCurrent}
          productId={Number.parseInt(id)}
          removeFunction={removeAndDeletePortion}
        />
        <Box margin={{ top: "large" }}>
          {!adding ? (
            <Button
              primary
              alignSelf="start"
              icon={<AddCircle />}
              label="Add portion"
              onClick={() => setAdding(true)}
            />
          ) : (
            <>
              <PortionForm
                product={Number.parseInt(id)}
                selectedFunction={async (portion: Portion) => {
                  const submitResult = await submit([portion]);
                  if (submitResult.status) {
                    cogoToast.success("Portion added");
                    await refreshPortions(Number.parseInt(id), setCurrent);
                  } else {
                    cogoToast.error("Failed to add portion");
                  }
                  setAdding(false);
                }}
                cancelfunction={() => setAdding(false)}
              />
            </>
          )}
        </Box>
      </Box>
    </>
  );
};

export default Portions;
