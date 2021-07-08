import React from "react";
import {
  Heading,
  Box,
  FormField,
  Text,
  TextInput,
  MaskedInput,
} from "grommet";
import { Product } from "../../util/schema/product";
import { Redirect } from "react-router-dom";
import { postProduct, postCSVProducts } from "../../util/data/requests";
import ProductForm from '../../containers/ProductForm';

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


const AddProduct = () => {
  const [sent, setSent] = React.useState(false);

  const onSubmit = (event: any) => {
    const product = toProduct(event.value);
    postProduct(product).then((result) => {
      setSent(true);
    });
  };

  return (
    <Box pad="large" align="center">
      <Box>
        <Heading>Add Product</Heading>
        <Text>All values should be provided per 100 g / ml</Text>
        <Box width="large">
          <ProductForm onSubmit={onSubmit} />
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
