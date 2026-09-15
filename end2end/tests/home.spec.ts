import { test, expect } from "@playwright/test";

import { APP_TITLE, HOME_HEADING } from "../constants";

test.describe("главная страница", () => {
  test("отдаёт SSR-разметку с заголовком и h1", async ({ page }) => {
    await page.goto("/");

    await expect(page).toHaveTitle(APP_TITLE);
    await expect(page.locator("h1")).toHaveText(HOME_HEADING);
  });

  test("содержит карточки возможностей шаблона", async ({ page }) => {
    await page.goto("/");

    // Роли (heading) вместо getByText: текст "коннекторы" встречается ещё и в
    // описании страницы — строгий режим Playwright на дублях падает.
    await expect(
      page.getByRole("heading", { name: "SSR + Hydration" }),
    ).toBeVisible();
    await expect(page.getByRole("heading", { name: "Коннекторы" })).toBeVisible();
    await expect(page.getByRole("heading", { name: "Deploy" })).toBeVisible();
  });
});