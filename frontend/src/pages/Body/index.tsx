import { Box, Heading } from "grommet";
import useSWR from "swr";
import React from "react";
import BodyChart from "../../containers/BodyChart";
import {
  extractFat,
  extractMass,
  valuesToChartData,
} from "../../util/data/calculations";
import { BodyOverview } from "../../util/schema/body";
import BodyTable from "../../containers/BodyTable";

const Body = () => {
  const { data, error } = useSWR<BodyOverview>("/api/body/overview");

  if (error) return <div>Error could not load the body data</div>;
  if (!data) return <div>loading...</div>;

  const montlhyMass = valuesToChartData(data.past, extractMass);
  const monthlyFat = valuesToChartData(data.past, extractFat);

  return (
    <Box pad="medium" align="center">
      <Heading>Body</Heading>
      <Box>
        <BodyTable {...data} />

        <Heading level={3}>Mass (Weight)</Heading>
        <Box width="1280px">
          <BodyChart data={montlhyMass} parseDomain={(data) => [50, 100]} />
        </Box>
        <Heading level={3}>Fat Percentage (%)</Heading>
        <Box width="1280px">
          <BodyChart data={monthlyFat} parseDomain={(data) => [5, 30]} />
        </Box>
      </Box>
    </Box>
  );
};

export default Body;
