// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CurrentUserPrompt } from "./user-prompt/CurrentUserPrompt";
import type { Profile } from "./Profile";

export interface TransferState { active_profile: Profile | null, locale: string, credentials?: Array<{id: string, data: object}>, current_user_prompt: CurrentUserPrompt | null, debug_messages: Array<string>, }