import { Box, Button, Form, FormField, Heading, TextInput } from "grommet";
import React from "react";
import { Redirect } from "react-router-dom";
import { SearchSuggestion } from "../../../containers/ProductSearchForm";
import ProductSelect, {
  emptyState,
  ProductSelectState,
  selectStateToConsumed,
} from "../../../containers/ProductSelect";
import { postRecipie } from "../../../util/data/requests";

type IngredientCreateCommand = {
  amount: number;
  product_id: number;
};

export type RecipieCreateCommand = {
  name: string;
  ingredients: IngredientCreateCommand[];
};

const AddRecipie = () => {
  const [sent, setSent] = React.useState(false);

  const [ingredients, setIngredients] = React.useState<ProductSelectState>(
    emptyState()
  );

  const send = (event: any) => {
    const boxed = selectStateToConsumed(ingredients);

    const payload: RecipieCreateCommand = {
      name: event.value.name,
      ingredients: boxed.map((i) => ({
        amount: i.amount,
        product_id: i.numericIdentifier,
      })),
    };

    postRecipie(payload).then(() => setSent(true));
  };

  return (
    <Box pad="large" gridArea="main">
      <Box>
        <Heading>Add Recipie</Heading>
        <Box>
          <Form onSubmit={send}>
            <FormField name={"name"} label={"Recipie name"} required={true}>
              <TextInput name={"name"} />
            </FormField>
            <ProductSelect state={ingredients} setState={setIngredients} />
            <Box direction="row" gap="medium">
              <Button type="submit" primary label="Submit" />
              <Button
                type="reset"
                label="Reset"
                onClick={() => {
                  setIngredients(emptyState());
                }}
              />
            </Box>
          </Form>
        </Box>
        {sent && <Redirect to="/recipies" />}
      </Box>
    </Box>
  );
};

export default AddRecipie;
