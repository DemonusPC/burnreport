import React from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";

import { Grommet } from "grommet";
import Index from "./pages/Index";
import Navigation from "./containers/Navigation";
import AddProduct from "./pages/AddProduct";
import Portions from "./pages/Portions";
import Body from "./pages/Body";
import AddBodyLog from "./pages/AddBodyLog";
import ProductsSearch from "./pages/ProductsSearch";
import ProductsList from "./pages/ProductsList";
import ProductPage from "./pages/ProductPage";
import TestPage from "./pages/TestPage";
import { burnReportTheme } from "./theme/light";

const App = () => {
  const [themeMode, setThemeMode] = React.useState<"light" | "dark">("light");

  return (
    <>
      <Router>
        <Grommet full theme={burnReportTheme} themeMode={themeMode}>
          <Navigation theme={themeMode} themeSwitcher={setThemeMode} />
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
            <Route path="/test">
              <TestPage />
            </Route>
            <Route path="/">
              <Index />
            </Route>
          </Switch>
        </Grommet>
      </Router>
    </>
  );
};

export default App;
