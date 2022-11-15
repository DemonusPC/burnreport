import React from 'react'
import { BrowserRouter as Router, Switch, Route } from 'react-router-dom'

import { Grommet } from 'grommet'
import Index from './pages/Index'
import Navigation from './containers/Navigation'
import AddProduct from './pages/AddProduct'
import Portions from './pages/Portions'
import ProductsSearch from './pages/ProductsSearch'
import ProductsList from './pages/ProductsList'
import ProductPage from './pages/ProductPage'
import TestPage from './pages/TestPage'
import { burnReportTheme } from './theme/base'
import { cookieTheme } from './theme/cookie'
import ResponsiveGrid from './containers/ResponsiveGrid'
import RecipieSearch from './pages/recipies/Search'
import RecipieList from './pages/recipies/RecipieList'
import RecipieView from './pages/recipies/RecipieView'

const App = () => {
  const [themeMode, setThemeMode] = React.useState<'light' | 'dark'>(
    cookieTheme()
  )

  return (
    <>
      <Router>
        <Grommet full theme={burnReportTheme} themeMode={themeMode}>
          <Navigation theme={themeMode} themeSwitcher={setThemeMode} />
          <ResponsiveGrid>
            <Switch>
              <Route path="/products/add">
                <AddProduct />
              </Route>
              <Route path="/products/list">
                <ProductsList />
              </Route>
              <Route path="/products/:id/portions">
                <Portions />
              </Route>
              <Route path="/products/:id">
                <ProductPage />
              </Route>
              <Route path="/products">
                <ProductsSearch />
              </Route>
              <Route path="/recipies/list">
                <RecipieList />
              </Route>
              <Route path="/recipies/:id">
                <RecipieView />
              </Route>
              <Route path="/recipies">
                <RecipieSearch />
              </Route>
              <Route path="/test">
                <TestPage />
              </Route>
              <Route path="/">
                <Index />
              </Route>
            </Switch>
          </ResponsiveGrid>
        </Grommet>
      </Router>
    </>
  )
}

export default App
