import {
  Product,
  ProductAPIStatus,
  ProductSize,
  Portion,
} from "../schema/product";
import { Report } from "../schema/report";
import { ReportResult } from "../../containers/ReportRender";
import { SearchSuggestion } from "../../containers/ProductSearchForm";

export interface RestResult<T> {
  status: boolean;
  data?: T;
}

export interface ResultList<T> {
  result: Array<T>;
}

export const getProductSearchSuggestions = async (
  suggestion: string
): Promise<Array<SearchSuggestion>> => {
  const request = await fetch(
    encodeURI(`/api/search/suggestions?p=${suggestion}`)
  );

  const result: ResultList<SearchSuggestion> = await request.json();

  return result.result;
};

export const getProductSearch = async (
  suggestion: string
): Promise<Array<Product>> => {
  const request = await fetch(encodeURI(`/api/search?p=${suggestion}`));

  const result: ResultList<Product> = await request.json();

  return result.result;
};

export const getSingleProductById = async (id: number) => {
  const request = await fetch(encodeURI(`/api/products/${id}`));

  const result: ResultList<Product> = await request.json();

  return result.result;
};

const generatePostRequest = <T, O>(uri: string): ((data: T) => Promise<O>) => {
  return async (data: T): Promise<O> => {
    const response = await fetch(`/api/report`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });

    const result: O = await response.json();

    return result;
  };
};

export const postReport = generatePostRequest<Report, ReportResult>(
  `/api/report`
);

export const postProduct = generatePostRequest<Product, ProductAPIStatus>(
  `/api/products`
);

export const deleteProduct = async (id: number): Promise<ProductAPIStatus> => {
  const response = await fetch(`/api/products/${id}`, {
    method: "DELETE",
    mode: "cors",
  });

  const result: ProductAPIStatus = await response.json();

  return result;
};

export const postCSVProducts = async (data: any): Promise<ProductAPIStatus> => {
  const response = await fetch(`/api/products/csv`, {
    method: "POST",
    body: data,
  });

  const result: ProductAPIStatus = await response.json();

  return result;
};

export const getProductSizesById = async (id: number) => {
  const request = await fetch(encodeURI(`/api/products/${id}/portions`));
  const result: ResultList<ProductSize> = await request.json();

  return result.result;
};

export const postPortions = async (
  portions: Array<Portion>
): Promise<RestResult<ProductAPIStatus>> => {
  const response = await fetch(`/api/products/portions`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(portions),
  });

  if (!response.ok) {
    const result: RestResult<ProductAPIStatus> = {
      status: false,
    };
    return result;
  }

  const jsonData: ProductAPIStatus = await response.json();

  const result: RestResult<ProductAPIStatus> = {
    status: true,
    data: jsonData,
  };

  return result;
};

export const deletePortion = async (
  id: number,
  name: string
): Promise<ProductAPIStatus> => {
  const response = await fetch(
    encodeURI(`/api/products/${id}/portions/${name}`),
    {
      method: "DELETE",
    }
  );

  const result: ProductAPIStatus = await response.json();

  return result;
};
