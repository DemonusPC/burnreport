import React from "react";
import { Link } from "react-router-dom";

import { Anchor, Text, Header, Nav, Box, Button } from "grommet";
import { Scorecard, Cube, Moon, Sun } from "grommet-icons";

interface NavigationProps {
  theme: "light" | "dark";
  themeSwitcher: (theme: "light" | "dark") => void;
}

const iconRender = (theme: "light" | "dark") => {
  if (theme === "dark") {
    return <Sun />;
  }
  return <Moon />;
};

const Navigation = ({ theme, themeSwitcher }: NavigationProps) => {
  return (
    <Header background="background-back" pad="medium">
      <Nav direction="row">
        <Text weight="bold" color="text">
          Burnreport
        </Text>
        <Link to="/">
          <Anchor
            icon={<Scorecard />}
            color="text"
            label={"Report"}
            key={"report"}
            as="div"
          />
        </Link>
        <Link to="/products">
          <Anchor
            icon={<Cube />}
            color="text"
            href={"/products"}
            label={"Products"}
            key={"products"}
            as="div"
          />
        </Link>
      </Nav>
      <Box direction="row">
        <Button
          type="button"
          plain
          icon={iconRender(theme)}
          onClick={() => {
            const nextTheme = theme === "light" ? "dark" : "light";
            document.cookie = `theme=${nextTheme}; SameSite=Strict; path=/`;
            themeSwitcher(nextTheme);
          }}
        />
      </Box>
    </Header>
  );
};

export default Navigation;

// {/* <Link to="/">Report</Link>
// <Link to="/products">Products</Link> */}
