import { test, expect } from "@playwright/test";

test.describe("навигация и страницы", () => {
  test("клиентская навигация: главная → кабинет → статус", async ({
    page,
  }) => {
    await page.goto("/");

    await page.getByRole("link", { name: "Кабинет", exact: true }).click();
    await expect(page).toHaveURL(/\/account$/);
    await expect(page.locator("h1")).toHaveText("Личный кабинет");
    await expect(page.getByText("user@example.com")).toBeVisible();

    await page.getByRole("link", { name: "Статус", exact: true }).click();
    await expect(page).toHaveURL(/\/status$/);
    await expect(page.locator("h1")).toHaveText("Статус сервисов");
  });

  test("страница статуса подгружает коннекторы из /api/health", async ({
    page,
  }) => {
    await page.goto("/status");

    // Данные приходят на клиенте после гидратации: SSR отдаёт заглушку.
    await expect(page.getByText("echo", { exact: true })).toBeVisible({
      timeout: 15_000,
    });
    await expect(page.getByText("ok", { exact: true })).toBeVisible();
  });

  test("неизвестный маршрут отдаёт 404 внутри layout", async ({ page }) => {
    await page.goto("/no-such-page");

    await expect(page.getByText("Страница не найдена")).toBeVisible();
    await expect(page.getByText("404", { exact: true })).toBeVisible();
    await expect(page.getByRole("link", { name: "На главную" })).toBeVisible();
  });
});