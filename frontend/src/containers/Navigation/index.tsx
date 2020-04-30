import React from 'react';
import {
    Link
  } from "react-router-dom";

import { Anchor, Header, Nav } from "grommet";

const Navigation = () => (
    <Header background="dark-1" pad="medium">
    <Nav direction="row">
        <Link to="/"><Anchor label={"Report"} key={"report"} /></Link>
        <Link to="/products"><Anchor href={"/products"} label={"Products"} key={"products"} /></Link>
    </Nav>
    </Header>
);

export default Navigation;

    // {/* <Link to="/">Report</Link>
    // <Link to="/products">Products</Link> */}
