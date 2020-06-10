import React from 'react';
import styled from 'styled-components';
import { Box, Heading, TableHeader, Table, Text, TableRow, TableCell, TableBody, Button } from 'grommet';
import SearchForm from '../SearchForm';
import { Product, ProductSize } from '../../util/schema/product';
import { getProductSearch, postReport } from '../../util/data/requests';
import { ConsumedProduct, Report, ConsumedRaw } from '../../util/schema/report';
import ProductItem from '../ProductItem';


const emptyState = () : Map<number, ConsumedRaw> => {
  return new Map();
}

const boxConsumedProduct = (raw: ConsumedRaw) : ConsumedProduct => {
  const amount = raw.amount * raw.unit.grams;
  return {
    id: raw.id,
    name: raw.name,
    amount: amount
  }
}

const StyledTable = styled(Table)`
  min-width: 40%;
`;


const sendReport = (consumed: ConsumedProduct[], setReport: any) => {
  const report : Report = {
    consumed
  }; 
  
  postReport(report).then((json) => {
    setReport({completed: true, report: json});
  }); 
}

interface ReportFormProps {
  setReportFunction: any;
}


// Data Operations

// Report Opertaions

const baseUnit : ProductSize = {
  id: 0,
  product: 0,
  name: "grams",
  grams: 1,
}

// Add Product Items
const addConsumedProduct = (product: Product, state: Map<number, ConsumedRaw>, setState: any) => {
  const boxedProduct : ConsumedRaw = {
    id: product.id,
    name: product.name,
    amount: 100,
    unit: baseUnit,
    unitOptions: [baseUnit]
  };

  const newState = new Map(state);
  newState.set(product.id, boxedProduct);
  setState(newState);
}
// Remove Product Items
const deleteConsumedProduct = (productId: number, state: Map<number, ConsumedRaw>, setState: any ) => {
  const newState = new Map(state);
  newState.delete(productId);
  setState(newState);
}

// ProductItem Operations

// ChangeAmount
const changeProductAmount = (productId: number, amount: number, state: Map<number, ConsumedRaw>, setState: any) => {
  const newState = new Map(state);
  const target = state.get(productId);
  if(target) {
    target.amount = amount;
    newState.set(productId, target);
    setState(newState);
  }
}

// Change Uni
const changeProductUnit = (productId: number, unit: ProductSize, state: Map<number, ConsumedRaw>, setState: any) => {
  const target = state.get(productId);
  if(target) {
    target.unit = unit;
    state.set(productId, target);
    setState(state);
  }
}



// Produyct Item Mapping 

const mapProductItems = (
  state: any,
  setState: any
) => {
  // This is quite inefficient
  const rawProducts: Array<ConsumedRaw> = Array.from(state.values());
  return rawProducts.map((product: ConsumedRaw) => (
    <ProductItem
      key={product.id}
      id={product.id}
      name={product.name}
      amount={product.amount}
      changeFunc={(event) => {
        changeProductAmount(product.id,+event.target.value, state, setState);

      }}
      deleteFunc={() => {
        deleteConsumedProduct(product.id, state, setState);
      }}
    />
  ));
};

const ReportForm = ({setReportFunction} : ReportFormProps)  => {
  const [state, setState] = React.useState(emptyState());
  const submit = () => {

    const rawProducts: Array<ConsumedRaw> = Array.from(state.values());
    const boxed = rawProducts.map((raw) => boxConsumedProduct(raw));
    sendReport(boxed, setReportFunction);
  }
  
  const reset = () => {
    setState(emptyState());
  }

  console.log(state);
   
    return (
        <Box>
        <Heading>Create Report</Heading>
        <Box pad={{ bottom: "large" }}>
          <StyledTable>
            <TableHeader>
              <TableRow>
                <TableCell key={"name"} scope="col">
                  <Text>Name</Text>
                </TableCell>
                <TableCell key={"amount"} scope="col">
                  <Text>Amount</Text>
                </TableCell>
                <TableCell key={"delete"} scope="col">
                  <Text></Text>
                </TableCell>
              </TableRow>
            </TableHeader>
            <TableBody>
              {mapProductItems(state, setState)}
            </TableBody>
          </StyledTable>
        </Box>
  
        <Box pad={{ bottom: "large" }}>
          <SearchForm
            selectedFunction={(product: Product) => {
              addConsumedProduct(product, state, setState);
            }}
            suggestFunction={getProductSearch}
          />
        </Box>
  
        <Box>
          <Box direction="row" gap="medium">
            <Button type="submit" primary label="Submit" onClick={submit} />
            <Button type="reset" label="Reset" onClick={reset} />
          </Box>
        </Box>
      </Box>
    )
}

export default ReportForm;