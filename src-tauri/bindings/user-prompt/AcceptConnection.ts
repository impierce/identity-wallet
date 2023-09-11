import type { CurrentUserPromptType } from "./CurrentUserPromptType";

export interface AcceptConnection { type: CurrentUserPromptType, client_name: string, logo_uri: string }