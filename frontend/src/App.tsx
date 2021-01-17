import React from "react";
import Products from "./pages/Products";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";

import { Grommet } from "grommet";
import { grommet } from "grommet/themes";
import Index from "./pages/Index";
import Navigation from "./containers/Navigation";
import AddProduct from "./pages/AddProduct";
import Portions from './pages/Portions';
import Body from "./pages/Body";
import AddBodyLog from "./pages/AddBodyLog";

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
