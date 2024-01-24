// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CancelUserFlow } from "./CancelUserFlow";
import type { CreateNew } from "./CreateNew";
import type { CredentialOffersSelected } from "./CredentialOffersSelected";
import type { CredentialsSelected } from "./CredentialsSelected";
import type { QrCodeScanned } from "./QrCodeScanned";
import type { SetDevMode } from "./SetDevMode";
import type { SetLocale } from "./SetLocale";
import type { UnlockStorage } from "./UnlockStorage";
import type { UpdateCredentialMetadata } from "./UpdateCredentialMetadata";
import type { UpdateProfileSettings } from "./UpdateProfileSettings";
import type { UserDataQuery } from "./UserDataQuery";

export type Action = { "type": "[App] Get state" } | { "type": "[Storage] Unlock", payload: UnlockStorage, } | { "type": "[App] Reset" } | { "type": "[DID] Create new", payload: CreateNew, } | { "type": "[Settings] Set locale", payload: SetLocale, } | { "type": "[Settings] Update profile", payload: UpdateProfileSettings, } | { "type": "[QR Code] Scanned", payload: QrCodeScanned, } | { "type": "[Authenticate] Connection accepted" } | { "type": "[User Flow] Cancel", payload?: CancelUserFlow, } | { "type": "[DEV] Set dev mode", payload: SetDevMode, } | { "type": "[DEV] Load profile" } | { "type": "[Authenticate] Credentials selected", payload: CredentialsSelected, } | { "type": "[Credential Offer] Selected", payload: CredentialOffersSelected, } | { "type": "[Credential Metadata] Update", payload: UpdateCredentialMetadata, } | { "type": "[User Journey] Cancel" } | { "type": "[User Data] Query", payload: UserDataQuery, };