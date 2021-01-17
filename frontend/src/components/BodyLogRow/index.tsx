import { Box, Menu } from "grommet";
import React from "react";
import { BodyLog } from "../../util/schema/body";

const mapDate = (date: Date): string => {
  const today = new Date();
  const parsed = new Date(date);

  if (parsed.toDateString() === today.toDateString()
  ) {
    return "Today";
  }

  const yesterday = new Date(today)

  yesterday.setDate(yesterday.getDate() - 1)

  if (parsed.toDateString() === yesterday.toDateString()
  ) {
    return "Yesterday";
  }

  return parsed.toDateString();
};

const BodyLogRow = ({ date, mass, fat }: BodyLog) => {
  return (
    <Box
      direction="row"
      border="bottom"
      justify="between"
      align="center"
      pad={{ top: "medium", bottom: "medium" }}
    >
      <Box width={{min: "20%"}}>{mapDate(date)}</Box> <Box width={{min: "10%"}}>{mass} kg</Box> <Box width={{min: "10%"}}>{fat}% Fat</Box>
      <Menu items={[{ label: "Edit", href: "/" }]} />
    </Box>
  );
};

export default BodyLogRow;
