import { Box, Heading } from "grommet";
import React from "react";
import styled from "styled-components";

const bodyIndex = {
    mass: 62,
    fat: 25,
}

const Body = () => {


  return (
    <Box pad="medium">
      <Heading>Body</Heading>
      <Box>
          <Heading level={2}>
              Mass (Weight)
          </Heading>
          {bodyIndex.mass} kg
          <Heading level={2}>
              Fat Percentage
          </Heading>
          {bodyIndex.fat} %
      </Box>
    </Box>
  );
};

export default Body;
