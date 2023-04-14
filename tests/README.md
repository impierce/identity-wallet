## Testing strategy

### Frontend

- The UI components are tested in [their own repository](https://github.com/impierce/ui-components), so there is no need to assure they are working as expected.
- Since all business logic is handled entirely in the Rust backend, the need for extensive unit testing is also not given.
- Tests should include (but not limited to):
    - layout & display of screens & flows on different screen sizes and devices, so no components are cut off or scaled strangely
    - components triggering the correct actions towards the backend
- We use the following libraries:
    - `vitest`
    - `@testing-library/svelte` (using `jsdom`)
    - `@testing-library/jest-dom` for convenient matchers
    - test globals are enabled to reduce boilerplate imports

### Backend

- focus on state changes, especially reducers need to be tested thoroughly

### Full application

- [WebDriver, Webdriver IO](https://tauri.app/v1/guides/testing/webdriver/introduction/) _(not implemented yet)_
