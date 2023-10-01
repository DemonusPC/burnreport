import { Box, Button, Form, FormField, Heading, TextInput } from "grommet";
import React from "react";
import { Redirect } from "react-router-dom";
import ProductSelect, {
  emptyState,
  ProductSelectState,
  selectStateToConsumed,
} from "../../../containers/ProductSelect";
import { postRecipie } from "../../../util/data/requests";
import { GetSearch, getProductSearchSuggestions } from "../../product/productApi";

type IngredientCreateCommand = {
  amount: number;
  product_id: number;
};

export type RecipieCreateCommand = {
  name: string;
  ingredients: IngredientCreateCommand[];
};

type AddRecipieProps = {
  productSearch?: GetSearch,
  addRecipie?: (data: RecipieCreateCommand) => Promise<void>
}

const AddRecipie = ({ productSearch = getProductSearchSuggestions, addRecipie = postRecipie }: AddRecipieProps) => {
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

    addRecipie(payload).then(() => setSent(true));
  };

  return (
    <Box pad="large" gridArea="main">
      <Box>
        <Heading>Add Recipie</Heading>
        <Box>
          <Form onSubmit={send}>
            <FormField name={"name"} label={"Recipie name"} required={true}>
              <TextInput aria-label="recipie-name" name={"name"} />
            </FormField>
            <ProductSelect state={ingredients} setState={setIngredients} search={productSearch} />
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
