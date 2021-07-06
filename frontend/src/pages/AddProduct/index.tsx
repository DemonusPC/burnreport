import React from "react";
import {
  Heading,
  Box,
  Form,
  FormField,
  TextInput,
  Button,
  MaskedInput,
} from "grommet";
import { Product } from "../../util/schema/product";
import { Redirect } from "react-router-dom";
import { postProduct, postCSVProducts } from "../../util/data/requests";

interface Category {
  name: string;
  fields: Array<NutritionField>;
}

// Note on the highprecision
// The lowest daily reference intake is 5 micrograms according to the eu
// legislation on the daily reference. To have some form of floor I decided
// to make 0.001 miligram to be the smallest value.
// Vitamins are stored as miligrams unlike basic macronutrients
interface NutritionField {
  name: string;
  fieldType: "text" | "masked" | "maskHighPrecision";
  defaultValue?: number;
  required?: boolean;
}

const formStructure: Array<Category> = [
  {
    name: "Product Information",
    fields: [
      {
        name: "name",
        fieldType: "text",
        required: true,
      },
      {
        name: "manufacturer",
        fieldType: "text",
        required: true,
      },
    ],
  },
  {
    name: "Energy",
    fields: [
      {
        name: "kj",
        fieldType: "masked",
        required: true,
      },
      {
        name: "kcal",
        fieldType: "masked",
        required: true,
      },
    ],
  },
  {
    name: "Carbohydrates",
    fields: [
      {
        name: "carbohydrates",
        fieldType: "masked",
        required: true,
      },
      {
        name: "sugar",
        fieldType: "masked",
        required: true,
      },
      {
        name: "addedSugar",
        fieldType: "masked",
        required: true,
      },
      {
        name: "fiber",
        fieldType: "masked",
        required: true,
      },
      {
        name: "starch",
        fieldType: "masked",
        required: true,
      },
    ],
  },

  {
    name: "Fats",
    fields: [
      {
        name: "fat",
        fieldType: "masked",
        required: true,
      },
      {
        name: "saturated",
        fieldType: "masked",
        required: true,
      },
      {
        name: "monosaturated",
        fieldType: "masked",
        required: true,
      },
      {
        name: "trans",
        fieldType: "masked",
        required: true,
      },
    ],
  },
  {
    name: "Protein",
    fields: [
      {
        name: "protein",
        fieldType: "masked",
        required: true,
      },
    ],
  },
  {
    name: "Salt",
    fields: [
      {
        name: "salt",
        fieldType: "maskHighPrecision",
        required: true,
      },
    ],
  },
  {
    name: "Vitamins",
    fields: [
      {
        name: "a",
        fieldType: "maskHighPrecision",
      },
      {
        name: "d",
        fieldType: "maskHighPrecision",
      },
      {
        name: "e",
        fieldType: "maskHighPrecision",
      },
      {
        name: "k",
        fieldType: "maskHighPrecision",
      },
      {
        name: "b1",
        fieldType: "maskHighPrecision",
      },
      {
        name: "b2",
        fieldType: "maskHighPrecision",
      },
      {
        name: "b3",
        fieldType: "maskHighPrecision",
      },
      {
        name: "b5",
        fieldType: "maskHighPrecision",
      },
      {
        name: "b6",
        fieldType: "maskHighPrecision",
      },
      {
        name: "b7",
        fieldType: "maskHighPrecision",
      },
      {
        name: "b9",
        fieldType: "maskHighPrecision",
      },
      {
        name: "b12",
        fieldType: "maskHighPrecision",
      },
      {
        name: "c",
        fieldType: "maskHighPrecision",
      },
    ],
  },
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
        b12: propertyToNumber(flat.b12),
        c: propertyToNumber(flat.c),
      },
    },
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
        setReport(true);
      });
    }
  };

  reader.readAsText(file);
};

const mapFields = (fields: Array<NutritionField>) => {
  return fields.map((field: NutritionField) => {
    if (field.fieldType === "maskHighPrecision") {
      return (
        <FormField
          name={field.name}
          label={capitalise(field.name)}
          required={field.required || false}
        >
          <MaskedInput
            name={field.name}
            mask={[
              {
                regexp: /^\d+$/,
                placeholder: "0",
              },
              { fixed: "." },
              {
                length: 3,
                regexp: /^[0-9]{1,4}$/,
                placeholder: "000",
              },
            ]}
          />
        </FormField>
      );
    } else if (field.fieldType === "masked") {
      return (
        <FormField
          name={field.name}
          label={capitalise(field.name)}
          required={field.required || false}
        >
          <MaskedInput
            name={field.name}
            mask={[
              {
                regexp: /^\d+$/,
                placeholder: "0",
              },
              { fixed: "." },
              {
                length: 2,
                regexp: /^[0-9]{1,4}$/,
                placeholder: "00",
              },
            ]}
          />
        </FormField>
      );
    }

    return (
      <FormField
        name={field.name}
        label={capitalise(field.name)}
        required={field.required || false}
      >
        <TextInput name={field.name} />
      </FormField>
    );
  });
};

const mapCategories = (categories: Array<Category>) => {
  return categories.map((c: Category) => (
    <>
      <Heading level={2}>{c.name}</Heading>
      {mapFields(c.fields)}
    </>
  ));
};

const AddProduct = () => {
  const [sent, setSent] = React.useState(false);

  return (
    <Box pad="large" align="center">
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
            {mapCategories(formStructure)}

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
