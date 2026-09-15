/**
 * Ожидания e2e-тестов.
 *
 * Значения по умолчанию совпадают с локальным запуском (`cargo leptos watch`),
 * но переопределяются через env — CI и стейджи используют те же тесты.
 */
export const APP_NAME = process.env.APP_NAME ?? "base_template";

/** Заголовок домашней страницы (совпадает с <Title text=...> в src/app.rs). */
export const APP_TITLE = process.env.APP_TITLE ?? APP_NAME;

/** h1 на главной странице. */
export const HOME_HEADING = `Добро пожаловать в ${APP_NAME}`;