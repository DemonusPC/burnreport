import React from "react";
import { Heading, Box, Form, FormField, TextInput, Button } from "grommet";
import { Product } from "../../util/schema/product";
import { Redirect } from "react-router-dom";
import { postProduct, postCSVProducts } from "../../util/data/requests";

const cols = [
  "name",
  "manufacturer",
  "kcal",
  "kj",
  "carbohydrates",
  "sugar",
  "addedSugar",
  "fiber",
  "starch",
  "fat",
  "saturated",
  "monounsaturated",
  "trans",
  "protein",
  "salt",
  "a",
  "d",
  "e",
  "k",
  "b1",
  "b2",
  "b3",
  "b5",
  "b6",
  "b7",
  "b9",
  "b12",
  "c"
];

const capitalise = (value: string) => {
  return value.charAt(0).toUpperCase() + value.slice(1);
};

const propertyToNumber = (property: number): number => {
  if (property) {
    if (!Number.isNaN(+property)) {
      return +property;
    }
  }
  return 0;
};

const toProduct = (flat: any): Product => {
  return {
    id: 0,
    name: flat.name,
    manufacturer: flat.manufacturer,
    energy: {
      kcal: propertyToNumber(flat.kcal),
      kj: propertyToNumber(flat.kj),
    },
    carbohydrates: {
      total: propertyToNumber(flat.carbohydrates),
      fiber: propertyToNumber(flat.fiber),
      sugar: propertyToNumber(flat.sugar),
      addedSugar: propertyToNumber(flat.addedSugar),
      starch: propertyToNumber(flat.starch),
    },
    fat: {
      total: propertyToNumber(flat.fat),
      saturated: propertyToNumber(flat.saturated),
      monounsaturated: propertyToNumber(flat.monounsaturated),
      trans: propertyToNumber(flat.trans),
    },
    protein: {
      total: propertyToNumber(flat.protein),
    },
    salt: {
      total: propertyToNumber(flat.salt),
    },
    vitamins: {
      fat: {
        a: propertyToNumber(flat.a),
        d: propertyToNumber(flat.d),
        e: propertyToNumber(flat.e),
        k: propertyToNumber(flat.k),
      },
      water: {
        b1: propertyToNumber(flat.b1),
        b2: propertyToNumber(flat.b2),
        b3: propertyToNumber(flat.b3),
        b5: propertyToNumber(flat.b5),
        b6: propertyToNumber(flat.b6),
        b7: propertyToNumber(flat.b7),
        b9: propertyToNumber(flat.b9),
        b12:propertyToNumber(flat.b12),
        c: propertyToNumber(flat.c),
      }
    }
  };
};

const fileChosen = (file: any | undefined, setReport: any) => {
  const reader = new FileReader();
  reader.onloadend = (e: any) => {
    const content = reader.result;
    if (content) {
      const form = new FormData();
      form.append("file", content.toString());

      postCSVProducts(form).then((status) => {
        setReport(true)
      })
    }
  };

  reader.readAsText(file);
};

const AddProduct = () => {
  const [sent, setSent] = React.useState(false);

  return (
    <Box pad="large">
      <Box>
        <Heading>Add Product</Heading>
        <Box width="large">
          <Form
            onSubmit={(event: any) => {
              const product = toProduct(event.value);
              postProduct(product).then((result) => {
                setSent(true);
              });
            }}
          >
            {cols.map((col) => (
              <FormField name={col} label={capitalise(col)} required>
                <TextInput name={col} />
              </FormField>
            ))}

            <Box direction="row" gap="medium">
              <Button type="submit" primary label="Submit" />
              <Button type="reset" label="Reset" />
            </Box>
          </Form>
        </Box>
        {sent && <Redirect to="/products" />}
      </Box>
      <Box margin={{ top: "xlarge" }}>
        <Heading>Upload Products as CSV</Heading>
        <input
          type="file"
          accept=".csv"
          onChange={(e) => {
            if (e.target.files) {
              fileChosen(e.target.files[0], setSent);
            }
          }}
        />
      </Box>
    </Box>
  );
};

export default AddProduct;
