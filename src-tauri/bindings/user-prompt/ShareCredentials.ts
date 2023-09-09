import type { CurrentUserPromptType } from "./CurrentUserPromptType";

export interface ShareCredentials { type: CurrentUserPromptType, client_name: string, logo_uri: string, options: Array<string>, }
