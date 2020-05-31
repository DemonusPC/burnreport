import React from "react";
import Products from "./pages/Products";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";

import { Grommet } from "grommet";
import { grommet } from "grommet/themes";
import Index from "./pages/Index";
import Navigation from "./containers/Navigation";
import AddProduct from "./pages/AddProduct";
import Portions from './pages/Portions';

const App = () => (
  <>
    <Router>
      <Grommet full theme={grommet}>
        <Navigation />
        <Switch>
          <Route path="/products/add">
            <AddProduct />
          </Route>
          <Route path="/products/:id/portions">
            <Portions />
          </Route>
          <Route path="/products">
            <Products />
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
