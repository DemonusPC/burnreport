import React from "react";
import { Box, TextInput, Form, Button, FormField } from "grommet";
import { Portion } from "../../util/schema/product";

interface PortionFormProps {
  product: number;
  selectedFunction: (product: Portion) => void;
}

const parseProduct = (product: number, value: any): Portion => {
  return {
    product: product,
    name: value.name,
    grams: value.portion,
  };
};

const PortionForm = ({ product, selectedFunction }: PortionFormProps) => {
  return (
    <Form
      onSubmit={(event: any) => {
        const parsed = parseProduct(product, event.value);

        if (parsed.grams > 0) {
          selectedFunction(parsed);
        }
      }}
    >
      <FormField name={"name"} label={"Name"} required>
        <TextInput name={"name"} />
      </FormField>

      <FormField name={"portion"} label={"portion (in grams)"} required>
        <TextInput name={"portion"} />
      </FormField>

      <Box direction="row" gap="medium">
        <Button type="submit" primary label="Add Portion" />
        <Button type="reset" label="Reset" />
      </Box>
    </Form>
  );
};

export default PortionForm;
