import {
  Box,
  Button,
  Heading,
  Text,
  NameValueList,
  NameValuePair,
} from "grommet";
import React from "react";
import { useParams } from "react-router";
import styled from "styled-components";
import useSWR from "swr";
import { Return } from "grommet-icons";
import { useHistory } from "react-router-dom";
import AnchorLink from "../../components/AnchorLink";
import { Spi } from "../AddSpi";
import { fetcher } from "../../util/data/requests";


interface ProductParams {
  id: string;
}

const SpiPage = () => {
  const history = useHistory();
  const params: ProductParams = useParams<ProductParams>();
  const parsed = Number.parseInt(params.id);

  const { data, error } = useSWR<Spi | null>(
    encodeURI(`/api/spi/${parsed}`),
    fetcher
  );

  if (error) return <div>Error</div>;
  if (!data) return <div>loading...</div>;

  return (
    <Box pad="large" gridArea="main">
      <Box
        direction="row"
        align="center"
        alignContent="between"
        justify="around"
        gap="large"
      >
        <Button
          type="button"
          label="Back"
          icon={<Return />}
          onClick={() => {
            window.history.back();
          }}
        />
        <AnchorLink to="/products/add" label="Add Product" />
      </Box>
      <Box>
        <Heading level={2}>{data.name}</Heading>
        <NameValueList>
          <NameValuePair name="Numeric Code">
            <Text color="text-strong">{data.numericCode}</Text>
          </NameValuePair>
          <NameValuePair name="Alphabetic Code">
            <Text color="text-strong">{data.alphabeticCode}</Text>
          </NameValuePair>
        </NameValueList>
      </Box>

    </Box>
  );
};

export default SpiPage;
