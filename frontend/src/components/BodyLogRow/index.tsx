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

const formatToDate = (date: Date): string => {
    const parsed = new Date(date);

    if(!parsed){
        return formatToDate(new Date());
    }

    return `${parsed.getFullYear()}-${parsed.getMonth() + 1}-${parsed.getDate()}`;
}

const BodyLogRow = ({ date, mass, fat }: BodyLog): JSX.Element => {
  return (
    <Box
      direction="row"
      border="bottom"
      justify="between"
      align="center"
      pad={{ top: "medium", bottom: "medium" }}
    >
      <Box width={{min: "20%"}}>{mapDate(date)}</Box> <Box width={{min: "15%"}}>{mass} kg</Box> <Box width={{min: "15%"}}>{fat}% Fat</Box>
      <Menu items={[{ label: "Edit", href: encodeURI("/body/update/" + formatToDate(date)) }]} />
    </Box>
  );
};

export default BodyLogRow;
