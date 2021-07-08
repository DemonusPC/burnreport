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
  unit?: "g" | "mg";
}

const requiredFields: Array<Category> = [
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
        unit: "g",
      },
      {
        name: "sugar",
        fieldType: "masked",
        required: true,
        unit: "g",
      },
      {
        name: "addedSugar",
        fieldType: "masked",
        required: true,
        unit: "g",
      },
      {
        name: "fiber",
        fieldType: "masked",
        required: true,
        unit: "g",
      },
      {
        name: "starch",
        fieldType: "masked",
        required: true,
        unit: "g",
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
        unit: "g",
      },
      {
        name: "saturated",
        fieldType: "masked",
        required: true,
        unit: "g",
      },
      {
        name: "monosaturated",
        fieldType: "masked",
        required: true,
        unit: "g",
      },
      {
        name: "trans",
        fieldType: "masked",
        required: true,
        unit: "g",
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
        unit: "g",
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
        unit: "g",
      },
    ],
  },
];

const vitaminForm: Array<Category> = [
    {
        name: "Vitamins",
        fields: [
          {
            name: "a",
            fieldType: "maskHighPrecision",
            unit: "mg",
          },
          {
            name: "d",
            fieldType: "maskHighPrecision",
            unit: "mg",
          },
          {
            name: "e",
            fieldType: "maskHighPrecision",
            unit: "mg",
          },
          {
            name: "k",
            fieldType: "maskHighPrecision",
            unit: "mg",
          },
          {
            name: "b1",
            fieldType: "maskHighPrecision",
            unit: "mg",
          },
          {
            name: "b2",
            fieldType: "maskHighPrecision",
            unit: "mg",
          },
          {
            name: "b3",
            fieldType: "maskHighPrecision",
            unit: "mg",
          },
          {
            name: "b5",
            fieldType: "maskHighPrecision",
            unit: "mg",
          },
          {
            name: "b6",
            fieldType: "maskHighPrecision",
            unit: "mg",
          },
          {
            name: "b7",
            fieldType: "maskHighPrecision",
            unit: "mg",
          },
          {
            name: "b9",
            fieldType: "maskHighPrecision",
            unit: "mg",
          },
          {
            name: "b12",
            fieldType: "maskHighPrecision",
            unit: "mg",
          },
          {
            name: "c",
            fieldType: "maskHighPrecision",
            unit: "mg",
          },
        ],
      },
];

const capitalise = (value: string) => {
  return value.charAt(0).toUpperCase() + value.slice(1);
};

const mapFields = (fields: Array<NutritionField>) => {
  return fields.map((field: NutritionField) => {
    const label = `${capitalise(field.name)} ${
      field.unit ? `(${field.unit})` : ""
    } ${field.required ? " *" : ""}`;

    if (field.fieldType === "maskHighPrecision") {
      return (
        <FormField
          name={field.name}
          label={label}
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
          label={label}
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
        label={label}
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

interface ProductFormProps {
  onSubmit: (event: any) => void;
}

const ProductForm = ({ onSubmit }: ProductFormProps): JSX.Element => {
    // For now only vitamins are optional so we can store just a single state
    const [optional, setOptional] = React.useState(false);
    return (
    <Form onSubmit={onSubmit}>
      {mapCategories(requiredFields)}

        { optional
            ? <>{mapCategories(vitaminForm)}</>
            : <Button type="button" label = "Add Vitamins" onClick={() => setOptional(true)} />  
        }

      <Box direction="row" gap="medium">
        <Button type="submit" primary label="Submit" />
        <Button type="reset" label="Reset" />
      </Box>
    </Form>
  );
};

export default ProductForm;
