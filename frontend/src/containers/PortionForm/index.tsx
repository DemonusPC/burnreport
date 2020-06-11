import React from "react";
import { Box, TextInput, Form, Button, FormField } from "grommet";
import { Portion } from "../../util/schema/product";

import { AddCircle, Clear } from "grommet-icons";

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
      <Box direction="row" pad={{top: "large", bottom:"large"}} gap="medium">
        <Box direction="row" gap="xsmall">
        <FormField name={"name"} label={"Name"} required>
          <TextInput name={"name"} plain={false} />
        </FormField>

        <FormField name={"portion"} label={"portion (in grams)"} required>
          <TextInput name={"portion"} plain={false} />
        </FormField>
        </Box>
        <Box direction="row" align="center" margin={{top: "medium"}} gap="xsmall">
        <Button type="submit" icon={<AddCircle />} plain={false} />
        <Button type="reset" icon={<Clear />} plain={false} />
        </Box>
      </Box>
    </Form>
  );
};

export default PortionForm;
