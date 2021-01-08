import { Box, Heading } from "grommet";
import useSWR from 'swr';
import React from "react";
import BodyChart from "../../containers/BodyChart";
import { valuesToChartData } from "../../util/data/calculations";
import { BodyOverview } from "../../util/schema/body";

const Body = () => {
  const { data, error } = useSWR<BodyOverview>("/api/body/today");
  
  if (error) return (<div>Error could not load the body data</div>);
  if (!data) return (<div>loading...</div>);
  
  const montlhyMass = valuesToChartData(data.monthly.mass);
  const monthlyFat = valuesToChartData(data.monthly.fat); 

  return (
    <Box pad="medium" align="center">
      <Heading>Body</Heading>
      <Box>
        <Heading level={2}>Today</Heading>
        <Heading level={3}>Mass (Weight)</Heading>
        {data.today.mass} kg
        <Heading level={3}>Fat Percentage (%)</Heading>
        {data.today.fat} %<Heading level={2}>Monthly</Heading>
        <Heading level={3}>Mass (Weight)</Heading>
        <Box width="1280px">
          <BodyChart data={montlhyMass} parseDomain={(data) => [50,100]} />
        </Box>
        <Heading level={3}>Fat Percentage (%)</Heading>
        <Box width="1280px">
        <BodyChart data={monthlyFat} parseDomain={(data) => [5,30]} />
        </Box>
      </Box>
    </Box>
  );
};

export default Body;
