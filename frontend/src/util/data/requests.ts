import { Product, ProductAPIStatus, Portion } from '../../product/product'
import { Report, ReportResult } from '../../report/report'
import { SearchSuggestion } from '../../containers/ProductSearchForm'
import { RecipieCreateCommand } from '../../pages/recipies/AddRecipie'
import { Spi } from '../../pages/AddSpi'

export interface RestResult<T> {
  status: boolean
  data?: T
}

export interface ResultList<T> {
  result: T[]
}

export const fetcher = async (url: string) =>
  await fetch(url).then(async (r) => {
    if (!r.ok) {
      throw new Error('An error occured')
    }
    return await r.json()
  })

export const getProductSearchSuggestions = async (
  suggestion: string
): Promise<SearchSuggestion[]> => {
  const request = await fetch(
    encodeURI(`/api/search?p=${suggestion}&e=product`)
  )

  const result: ResultList<SearchSuggestion> = await request.json()

  return result.result
}

const generatePostRequest = <T, O>(uri: string): ((data: T) => Promise<O>) => {
  return async (data: T): Promise<O> => {
    const response = await fetch(uri, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(data)
    })

    const result: O = await response.json()

    return result
  }
}

// This is different from a generator
const deleteRequest = async <O>(uri: string): Promise<O> => {
  const response = await fetch(uri, {
    method: 'DELETE'
  })

  const result: O = await response.json()

  return result
}

export const postReport = generatePostRequest<Report, ReportResult>(
  '/api/report'
)

export const postRecipie = generatePostRequest<RecipieCreateCommand, never>('/api/recipies');

export const deleteRecipie = async (id: number): Promise<ProductAPIStatus> => {
  return await deleteRequest(`/api/recipies/${id}`)
}


export const postProduct = generatePostRequest<Product, ProductAPIStatus>(
  '/api/products'
)


export const deleteProduct = async (id: number): Promise<ProductAPIStatus> => {
  return await deleteRequest(`/api/products/${id}`)
}

export const postCSVProducts = async (data: any): Promise<ProductAPIStatus> => {
  const response = await fetch('/api/products/csv', {
    method: 'POST',
    body: data
  })

  const result: ProductAPIStatus = await response.json()

  return result
}

export const getProductSizesById = async (id: number) => {
  const request = await fetch(encodeURI(`/api/products/${id}/portions`))
  const result: ResultList<Portion> = await request.json()

  return result.result
}

export const postPortions = generatePostRequest<
  Portion[],
  RestResult<ProductAPIStatus>
>('/api/products/portions')

export const deletePortion = async (
  id: number,
  name: string
): Promise<ProductAPIStatus> => {
  return await deleteRequest<ProductAPIStatus>(
    `/api/products/${id}/portions/${name}`
  )
}

export const postSpi = generatePostRequest<Spi, ProductAPIStatus>(
  '/api/spi'
)