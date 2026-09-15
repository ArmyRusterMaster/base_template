import { test, expect } from "@playwright/test";

import { APP_NAME } from "../constants";

test.describe("инфраструктурный API", () => {
  test("GET /api/health возвращает статус, версию и коннекторы", async ({
    request,
  }) => {
    const response = await request.get("/api/health");

    expect(response.status()).toBe(200);
    expect(response.headers()["content-type"]).toContain("application/json");

    const body = await response.json();
    expect(body.status).toBe("ok");
    // Версия в формате vX.Y.Z-<hash> (см. docs/versioning.md).
    expect(body.version).toMatch(/^v\d+\.\d+\.\d+-.+/);
    expect(Array.isArray(body.connectors)).toBe(true);
    expect(body.connectors[0]).toMatchObject({ id: "echo", status: "ok" });
  });

  test("GET /api/health помечает запрос request-id", async ({ request }) => {
    const response = await request.get("/api/health");

    expect(response.headers()["x-request-id"]).toBeTruthy();
  });

  test("неизвестный путь отдаёт shell без деталей ошибки", async ({
    request,
  }) => {
    const response = await request.get("/unknown-path");

    // SSR-fallback отдаёт shell приложения (клиентский роутер показывает 404),
    // важно, что наружу не утекают внутренние детали.
    expect([200, 404]).toContain(response.status());
    const body = await response.text();
    expect(body).toContain(APP_NAME);
    expect(body).not.toContain("Internal server error");
  });
});