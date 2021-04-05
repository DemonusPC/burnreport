import React from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";

import { Grommet } from "grommet";
import { grommet } from "grommet/themes";
import Index from "./pages/Index";
import Navigation from "./containers/Navigation";
import AddProduct from "./pages/AddProduct";
import Portions from "./pages/Portions";
import Body from "./pages/Body";
import AddBodyLog from "./pages/AddBodyLog";
import ProductsSearch from "./pages/ProductsSearch";
import ProductsList from "./pages/ProductsList";
import ProductPage from "./pages/ProductPage";

const App = () => (
  <>
    <Router>
      <Grommet full theme={grommet}>
        <Navigation />
        <Switch>
          <Route path="/body/update/:date">
            <AddBodyLog />
          </Route>
          <Route path="/body/add">
            <AddBodyLog />
          </Route>
          <Route path="/body">
            <Body />
          </Route>
          <Route path="/products/add">
            <AddProduct />
          </Route>
          <Route path="/products/:id/portions">
            <Portions />
          </Route>
          <Route path="/products/list">
            <ProductsList />
          </Route>
          <Route path="/products/:id">
            <ProductPage />
          </Route>
          <Route path="/products">
            <ProductsSearch />
          </Route>
          <Route path="/">
            <Index />
          </Route>
        </Switch>
      </Grommet>
    </Router>
  </>
);

export default App;
