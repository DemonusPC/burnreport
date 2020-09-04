import React from "react";
import {
  Box,
  Heading,
  Button,
} from "grommet";
import SearchForm from "../SearchForm";
import { Product, ProductSize } from "../../util/schema/product";
import { getProductSearch, postReport, getProductSizesById } from "../../util/data/requests";
import { ConsumedProduct, Report, ConsumedRaw } from "../../util/schema/report";
import ProductItem from "../ProductItem";

const emptyState = (): Map<number, ConsumedRaw> => {
  return new Map();
};

const boxConsumedProduct = (raw: ConsumedRaw): ConsumedProduct => {
  const amount = raw.amount * raw.unit.grams;
  return {
    id: raw.id,
    name: raw.name,
    amount: amount,
  };
};

const sendReport = (consumed: ConsumedProduct[], setReport: any) => {
  const report: Report = {
    consumed,
  };

  postReport(report).then((json) => {
    setReport({ completed: true, report: json });
  });
};

interface ReportFormProps {
  setReportFunction: any;
}

// Data Operations

// Report Opertaions

const baseUnit: ProductSize = {
  id: 0,
  product: 0,
  name: "grams",
  grams: 1,
};


// Add Product Items
const addConsumedProduct = (
  product: Product,
  setState: any
) => {
  const boxedProduct: ConsumedRaw = {
    id: product.id,
    name: product.name,
    amount: 100,
    unit: baseUnit,
    unitOptions: [baseUnit],
  };

  setState((prevState : any) => {
    const newState = new Map(prevState);
    newState.set(product.id, boxedProduct);
    return newState;
  });
};
// Remove Product Items
const deleteConsumedProduct = (
  productId: number,
  setState: any
) => {
  setState((prevState : any) => {
    const newState = new Map(prevState);
    newState.delete(productId);
    return newState;
  });
};

// ProductItem Operations

// ChangeAmount
const changeProductAmount = (
  productId: number,
  amount: number,
  setState: any
) => {
  
  setState((prevState : any) => {
    const target = prevState.get(productId);
    if (target) {
      const newState = new Map(prevState);
      target.amount = amount;
      newState.set(productId, target);
      return newState;
    }
    return prevState;
  });

};

// Change Unit
const changeProductUnit = (
  productId: number,
  unit: ProductSize,
  setState: any
) => {
  setState((prevState : any) => {
    const target = prevState.get(productId);
    if (target) {
      const newState = new Map(prevState);
      target.unit = unit;
      newState.set(productId, target);
      return newState;
    }
    return prevState;
  });
};

// Get options for product
const getProductUnitOptions = async (productId: number, setState: any) => {
  const serverPortions = await getProductSizesById(productId);

  setState((prevState : any) => {
    const target : ConsumedRaw = prevState.get(productId);
    if(target){
      const newState = new Map(prevState);
      target.unitOptions = [baseUnit].concat(serverPortions);
      newState.set(productId, target);
      return newState;
    }

    return prevState;
  });
}


// Produyct Item Mapping

const mapProductItems = (state: any, setState: any) => {
  // This is quite inefficient
  const rawProducts: Array<ConsumedRaw> = Array.from(state.values());
  return rawProducts.map((product: ConsumedRaw) => (
    <ProductItem
      key={product.id}
      id={product.id}
      name={product.name}
      amount={product.amount}
      changeFunc={(event) => {
        changeProductAmount(product.id, +event.target.value, setState);
      }}
      deleteFunc={() => {
        deleteConsumedProduct(product.id, setState);
      }}
      unit={product.unit}
      unitOptions={product.unitOptions}
      setUnit={((option: ProductSize) => {
        changeProductUnit(product.id, option, setState)
      } )}
    />
  ));
};

const ReportForm = ({ setReportFunction }: ReportFormProps) => {
  const [state, setState] = React.useState(emptyState());

  return (
    <Box>
      <Heading>Create Report</Heading>
      <Box pad={{ bottom: "large" }}>
          {mapProductItems(state, setState)}
      </Box>

      <Box pad={{ bottom: "large" }}>
        <SearchForm
          selectedFunction={(product: Product) => {
            addConsumedProduct(product, setState);
            getProductUnitOptions(product.id, setState);
          }}
          suggestFunction={getProductSearch}
        />
      </Box>

      <Box>
        <Box direction="row" gap="medium">
          <Button type="submit" primary label="Submit" onClick={() => {
                const rawProducts: Array<ConsumedRaw> = Array.from(state.values());
                const boxed = rawProducts.map((raw) => boxConsumedProduct(raw));
                sendReport(boxed, setReportFunction);
          }} />
          <Button
            type="reset"
            label="Reset"
            onClick={() => {
              setState(emptyState());
            }}
          />
        </Box>
      </Box>
    </Box>
  );
};

export default ReportForm;
