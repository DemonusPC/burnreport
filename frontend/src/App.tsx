import React from 'react';
import Products from './pages/Products';
import {
  BrowserRouter as Router,
  Switch,
  Route,
} from "react-router-dom";

import { Grommet } from "grommet";
import { grommet } from "grommet/themes";
import Index from './pages/Index';
import Navigation from './containers/Navigation';



const App = () => (
  <>
      <Router>
      <Grommet full theme={grommet}>
      <Navigation />
        {/* A <Switch> looks through its children <Route>s and
            renders the first one that matches the current URL. */}
        <Switch>
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

