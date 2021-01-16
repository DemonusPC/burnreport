import {
  Box,
  Heading,
  Form,
  FormField,
  TextInput,
  DateInput,
  Button,
} from "grommet";
import React from "react";
import { postBodyLog } from "../../util/data/requests";
import { BodyLog } from "../../util/schema/body";

const propertyToNumber = (property: number): number => {
  if (property) {
    if (!Number.isNaN(+property)) {
      return +property;
    }
  }
  return 0;
};

const toBodyLog = (flat: any): BodyLog => {
  console.log(flat);
  return {
    date: new Date(flat.date),
    mass: propertyToNumber(flat.mass),
    fat: propertyToNumber(flat.fat),
  };
};

const AddBodyLog = () => {
  const [body, setBody] = React.useState({
    date: new Date().toString(),
    mass: 0.0,
    fat: 0.0,
  });

  return (
    <Box pad="medium" align="center">
      <Heading>Add Body Log</Heading>
      <Form
        onSubmit={(event: any) => {
          const entry = toBodyLog(event.value);
          postBodyLog(entry).then((result) => {
            window.history.replaceState(window.history.state, "", "/body");
            window.location.reload();
          });
        }}
      >
        <FormField name="date" label="Date" required>
          <DateInput
            name="date"
            format="dd/mm/yyyy"
            value={body.date}
            onChange={({ value }) => {
              let spread = {date: value.toString()};
              setBody(prevState => {
                return {...prevState, ...spread};
              });

            }}
          />
        </FormField>
        <FormField name={"mass"} label={"Mass"} required>
          <TextInput name={"mass"} />
        </FormField>

        <FormField name={"fat"} label={"Fat"} required>
          <TextInput name={"fat"} />
        </FormField>

        <Box direction="row" gap="medium">
          <Button type="submit" primary label="Submit" />
          <Button type="reset" label="Reset" />
        </Box>
      </Form>
    </Box>
  );
};

export default AddBodyLog;
