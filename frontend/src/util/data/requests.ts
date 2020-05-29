import { Product, ProductAPIStatus, ProductSize } from "../schema/product";
import { Report } from "../schema/report";
import { ReportResult } from "../../containers/ReportRender";

export const getProductSearch = async (suggestion: string): Promise<Array<Product>> => {
    const request =  await fetch(encodeURI(`/api/search?p=${suggestion}`));

    const result : Array<Product> = await request.json();

    return result;
}


export const getSingleProductById = async (id: number) => {
    const request =  await fetch(encodeURI(`/api/products/${id}`));

    const result : Array<Product> = await request.json();

    return result;
}
    
export const postReport = async (report: Report): Promise<ReportResult> => {
    const response = await fetch(`/api/report`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        origin: "*",
      },
      mode: "cors",
      body: JSON.stringify(report)
    });

    const result: ReportResult = await response.json();

    return result;
}

export const postProduct = async (product: Product): Promise<ProductAPIStatus> => {
  const response = await fetch(`/api/products`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      origin: "*",
    },
    mode: "cors",
    body: JSON.stringify(product)
  });

  const result: ProductAPIStatus = await response.json();

  return result;
}

export const deleteProduct = async (id: number): Promise<ProductAPIStatus> => {
  const response = await fetch(`/api/products/${id}`, {
    method: 'DELETE',
    headers: {
      origin: "*",
    },
    mode: "cors",
  });

  const result: ProductAPIStatus = await response.json();

  return result;
}

export const postCSVProducts = async(data: any): Promise<ProductAPIStatus> => {
  const response = await fetch(`/api/products/csv`, {
    method: 'POST',
    headers: {
      origin: "*",
    },
    mode: "cors",
    body: data
  });

  const result: ProductAPIStatus = await response.json();

  return result;
}

export const getProductSizesById = async (id: number) => {
  const request =  await fetch(encodeURI(`/api/products/${id}/sizes`));

  const result : Array<ProductSize> = await request.json();

  return result;
}