import React from "react";

import { Text, Header, Nav, Box, Button } from "grommet";
import { Scorecard, Cube, Moon, Sun } from "grommet-icons";
import AnchorLink from "../../components/AnchorLink";

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
      <Nav direction="row" margin={{ top: "4px" }}>
        <Text weight="bold" color="text">
          Burnreport
        </Text>
        <AnchorLink
          icon={<Scorecard />}
          color="text"
          label={"Report"}
          key={"report"}
          to="/"
        />
        <AnchorLink
          to="/products"
          icon={<Cube />}
          color="text"
          href={"/products"}
          label={"Products"}
          key={"products"}
        />
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
