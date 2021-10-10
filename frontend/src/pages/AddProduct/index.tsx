import React from "react";
import { Heading, Box, Text, FileInput } from "grommet";
import {
  MonoUnsaturatedFat,
  PolyunsaturatedFat,
  Product,
  Unit,
} from "../../util/schema/product";
import { Redirect } from "react-router-dom";
import { postProduct, postCSVProducts } from "../../util/data/requests";
import ProductForm from "../../containers/ProductForm";

const propertyToNumber = (property: number): number => {
  if (property) {
    if (!Number.isNaN(+property)) {
      return +property;
    }
  }
  return 0;
};

const toProduct = (flat: any): Product => {
  const result: Product = {
    id: 0,
    name: flat.name,
    unit: Unit.Grams,
    nutrients: {
      energy: {
        kcal: propertyToNumber(flat.kcal),
        kj: propertyToNumber(flat.kj),
      },
      carbohydrates: {
        total: propertyToNumber(flat.carbohydrates),
        sugar: propertyToNumber(flat.sugar),
        fiber: flat.fiber ? propertyToNumber(flat.fiber) : undefined,
        addedSugar: flat.addedSugar
          ? propertyToNumber(flat.addedSugar)
          : undefined,
        starch: flat.starch ? propertyToNumber(flat.starch) : undefined,
      },
      fat: {
        total: propertyToNumber(flat.fat),
        saturated: propertyToNumber(flat.saturated),
        trans: flat.trans ? propertyToNumber(flat.trans) : undefined,
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
    },
  };

  const mono: MonoUnsaturatedFat | undefined =
    flat.monounsaturaed || flat.omega7 || flat.omega9
      ? {
          total: propertyToNumber(flat.monounsaturated) || 0,
          omega7: propertyToNumber(flat.omega7),
          omega9: propertyToNumber(flat.omega9),
        }
      : undefined;

  const poly: PolyunsaturatedFat | undefined =
    flat.monounsaturaed || flat.omega7 || flat.omega9
      ? {
          total: propertyToNumber(flat.polyunsaturated) || 0,
          omega3: propertyToNumber(flat.omega3),
          omega6: propertyToNumber(flat.omega6),
        }
      : undefined;

  if (poly || mono) {
    result.nutrients.fat.unsaturated = {
      mono,
      poly,
    };
  }
  return result;
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
    <Box pad="large" gridArea="main">
      <Box>
        <Heading>Add Product</Heading>
        <Text>All values should be provided per 100 g / ml</Text>
        <Box>
          <ProductForm onSubmit={onSubmit} />
        </Box>
        {sent && <Redirect to="/products" />}
      </Box>
      <Box margin={{ top: "xlarge" }}>
        <Heading>Upload Products as CSV</Heading>
        <FileInput
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
