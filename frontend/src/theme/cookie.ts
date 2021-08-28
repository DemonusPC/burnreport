export const cookieTheme = (): "light" | "dark" => {
  const cookie = document.cookie;

  if (cookie) {
    if (cookie === "theme=dark") {
      return "dark";
    }
    return "light";
  }

  document.cookie = "theme=light";
  return "light";
};
