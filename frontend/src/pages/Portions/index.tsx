import React, { useState } from "react";
import { Heading, Box, Button } from "grommet";
import { AddCircle } from "grommet-icons";
import { useParams } from "react-router";

import { Portion } from "../../util/schema/product";
import PortionForm from "../../containers/PortionForm";

import {
  postPortions,
  deletePortion,
  ResultList,
} from "../../util/data/requests";
import PortionList from "../../components/PortionList";
import useSWR, { mutate } from "swr";

interface IdParams {
  id: string;
}

const formatToPortion = (unformatted: any): Portion => {
  return {
    product: parseInt(unformatted.product),
    name: unformatted.name,
    grams: parseFloat(unformatted.grams),
  };
};

// Any since it can take any of the two functions
// I need to type this correclty
const mutatePortions = async (
  id: number,
  nextData: ResultList<Portion>,
  apiCall: any
) => {
  mutate(`/api/products/${id}/portions`, nextData, false);

  await apiCall();

  mutate(`/api/products/${id}/portions`);
};

const Portions = () => {
  const { id } = useParams<IdParams>();
  const { data, error } = useSWR<ResultList<Portion>>(
    encodeURI(`/api/products/${id}/portions`)
  );
  const [adding, setAdding] = useState(false);

  if (error) return <div>Error</div>;
  if (!data) return <div>loading...</div>;

  const current = data.result;

  return (
    <>
      <Box pad="large" width="large">
        <Heading size="small">Portions</Heading>
        <PortionList
          portions={current}
          productId={Number.parseInt(id)}
          removeFunction={async (portionName: string) => {
            const nextdata: ResultList<Portion> = {
              result: current.filter(
                (value: Portion) => value.name !== portionName
              ),
            };

            await mutatePortions(Number.parseInt(id), nextdata, async () => {
              await deletePortion(Number.parseInt(id), portionName);
            });
          }}
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
                selectedFunction={async (portion: any) => {
                  const formatted: Portion = formatToPortion(portion);

                  const nextdata: ResultList<Portion> = {
                    result: [...current, formatted],
                  };

                  await mutatePortions(
                    Number.parseInt(id),
                    nextdata,
                    async () => {
                      await postPortions([formatted]);
                    }
                  );

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
