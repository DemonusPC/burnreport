import React from "react";
import { Box, TextInput, Form, Button, FormField, Heading } from "grommet";
import { Portion } from "../../product/product";

import { AddCircle } from "grommet-icons";

interface PortionFormProps {
  product: number;
  selectedFunction: (product: Portion) => void;
  cancelfunction: () => void;
}

const parseProduct = (product: number, value: any): Portion => {
  return {
    product: product,
    name: value.name,
    grams: value.portion,
  };
};

const PortionForm = ({
  product,
  selectedFunction,
  cancelfunction,
}: PortionFormProps) => {
  return (
    <Form
      onSubmit={(event: any) => {
        const parsed = parseProduct(product, event.value);

        if (parsed.grams > 0) {
          selectedFunction(parsed);
        }
      }}
    >
      <Box gap="medium">
        <Heading level={2} size="small">
          New portion
        </Heading>
        <FormField name={"name"} label={"Name"} required>
          <TextInput name={"name"} plain={false} />
        </FormField>

        <FormField name={"portion"} label={"Portion (in grams)"} required>
          <TextInput name={"portion"} plain={false} />
        </FormField>
        <Box
          direction="row"
          align="center"
          margin={{ top: "medium" }}
          gap="small"
        >
          <Button primary type="submit" icon={<AddCircle />} label="Add" />

          <Button type="button" label="Cancel" onClick={cancelfunction} />
        </Box>
      </Box>
    </Form>
  );
};

export default PortionForm;
