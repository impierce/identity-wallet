## Testing strategy

### Frontend

#### Unit tests

- `vitest`
- only test frontend-specific logic (such as "determine if dark theme needs to be applied" or "format date")

#### Component tests

- `vitest` (every dependency is mocked)

- `playwright`
- Tests should include (but not limited to):
  - layout & display of screens & flows on different screen sizes and devices, so no components are cut off or scaled strangely
  - components triggering the correct actions towards the backend
  - `@testing-library/svelte` (using `jsdom`)
  - `@testing-library/jest-dom` for convenient matchers
  - test globals are enabled to reduce boilerplate imports

> We only test the frontend code here, the Tauri API is mocked!

#### End-to-end tests

- covered through full application tests that run on a simulator

### Backend

- focus on state changes, especially reducers _need_ to be tested thoroughly

### Full application

- [WebDriver, Webdriver IO](https://tauri.app/v1/guides/testing/webdriver/introduction/) _(not implemented yet)_
- `webdriver.io`, `appium`
- Since all business logic is handled entirely in the Rust backend, the need for extensive e2e testing is not given.
  Some smoke tests should be enough.
