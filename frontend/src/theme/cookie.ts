export const cookieTheme = (): "light" | "dark" => {
  const cookie = document.cookie;

  if (cookie) {
    if (cookie === "theme=dark; SameSite=Strict") {
      return "dark";
    }
    return "light";
  }

  document.cookie = "theme=light; SameSite=Strict";
  return "light";
};
