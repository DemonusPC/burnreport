import React from "react";
import { Box } from "grommet";
import SearchForm from "../SearchForm";
import { Portion } from "../../product/product";
import {
  getProductSizesById,
} from "../../util/data/requests";
import { ReportItem, ConsumedRaw } from "../../report/report";
import ProductItem from "../ProductItem";
import {
  SearchSuggestion,
  searchSuggestionToProductEntity,
} from "../ProductSearchForm";
import { GetSearch, getProductSearchSuggestions } from "../../pages/product/productApi";

export type ProductSelectState = Map<number, ConsumedRaw>;

export const emptyState = (): ProductSelectState => {
  return new Map();
};

const boxConsumedProduct = (raw: ConsumedRaw): ReportItem => {
  const amount = raw.amount * raw.unit.grams;
  return {
    numericIdentifier: raw.id,
    entity: raw.entity,
    amount: amount,
  };
};

export const selectStateToConsumed = (
  state: ProductSelectState
): ReportItem[] => {
  const rawProducts: Array<ConsumedRaw> = Array.from(state.values());
  const boxed = rawProducts.map((raw) => boxConsumedProduct(raw));
  return boxed;
};

const baseUnit: Portion = {
  id: 0,
  product: 0,
  name: "grams",
  grams: 1,
};

// Add Product Items
const addConsumedProduct = (suggestion: SearchSuggestion, setState: any) => {
  const boxedProduct: ConsumedRaw = {
    id: suggestion.id,
    name: suggestion.text,
    entity: searchSuggestionToProductEntity(suggestion.entity),
    amount: 100,
    unit: baseUnit,
    unitOptions: [baseUnit],
  };

  setState((prevState: any) => {
    const newState = new Map(prevState);
    newState.set(suggestion.id, boxedProduct);
    return newState;
  });
};
// Remove Product Items
const deleteConsumedProduct = (productId: number, setState: any) => {
  setState((prevState: any) => {
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
  setState((prevState: any) => {
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
const changeProductUnit = (productId: number, unit: Portion, setState: any) => {
  setState((prevState: any) => {
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

  setState((prevState: any) => {
    const target: ConsumedRaw = prevState.get(productId);
    if (target) {
      const newState = new Map(prevState);
      target.unitOptions = [baseUnit].concat(serverPortions);
      newState.set(productId, target);
      return newState;
    }

    return prevState;
  });
};

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
      setUnit={(option: Portion) => {
        changeProductUnit(product.id, option, setState);
      }}
    />
  ));
};

interface ReportFormProps {
  state: ProductSelectState;
  setState: (state: ProductSelectState) => void;
  search?: GetSearch
}

const ProductSelect = ({ state, setState, search = getProductSearchSuggestions }: ReportFormProps) => {
  return (
    <Box>
      <Box pad={{ bottom: "large" }}>{mapProductItems(state, setState)}</Box>

      <Box pad={{ bottom: "large" }}>
        <SearchForm
          selectedFunction={(suggestion: SearchSuggestion) => {
            addConsumedProduct(suggestion, setState);
            getProductUnitOptions(suggestion.id, setState);
          }}
          suggestFunction={search}
        />
      </Box>
    </Box>
  );
};

export default ProductSelect;
