import { Button, Box, Text } from "grommet";
import React from "react";
import BodyLogRow from "../../components/BodyLogRow";
import { BodyLog, BodyOverview } from "../../util/schema/body";

const BodyTable = ({ today, past }: BodyOverview) => {
  if (!today) {
  }

  const rows = past.map((logValue: BodyLog) => {
    return <BodyLogRow key={logValue.date.toString()} date={logValue.date} mass={logValue.mass} fat={logValue.fat} />;
  });

  return (
    <>
      {!today && 
        <Box direction = "row" align="center" gap="medium" pad={{top: "medium", bottom: "small"}}>
          <Text size="large" weight="bold">No log item for today!</Text>
          <Button
            primary
            label="Add Body Log"
            href="/body/add"
            alignSelf="start"
          />
        </Box>
      }
      <>{rows}</>
    </>
  );
};

export default BodyTable;
