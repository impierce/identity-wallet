// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { AddRecentSearch } from "./AddRecentSearch";
import type { CancelUserFlow } from "./CancelUserFlow";
import type { CreateNew } from "./CreateNew";
import type { CredentialOffersSelected } from "./CredentialOffersSelected";
import type { CredentialsSelected } from "./CredentialsSelected";
import type { DeleteRecentSearch } from "./DeleteRecentSearch";
import type { DevProfile } from "./DevProfile";
import type { QrCodeScanned } from "./QrCodeScanned";
import type { SearchQuery } from "./SearchQuery";
import type { SetLocale } from "./SetLocale";
import type { UnlockStorage } from "./UnlockStorage";
import type { UpdateCredentialMetadata } from "./UpdateCredentialMetadata";
import type { UpdateProfileSettings } from "./UpdateProfileSettings";

export type Action = { "type": "[App] Get state" } | { "type": "[Storage] Unlock", payload: UnlockStorage, } | { "type": "[App] Reset" } | { "type": "[DID] Create new", payload: CreateNew, } | { "type": "[Settings] Set locale", payload: SetLocale, } | { "type": "[Settings] Update profile", payload: UpdateProfileSettings, } | { "type": "[QR Code] Scanned", payload: QrCodeScanned, } | { "type": "[Authenticate] Connection accepted" } | { "type": "[User Flow] Cancel", payload?: CancelUserFlow, } | { "type": "[DEV] Load DEV profile", payload: DevProfile, } | { "type": "[DEV] Toggle DEV mode" } | { "type": "[Authenticate] Credentials selected", payload: CredentialsSelected, } | { "type": "[Credential Offer] Selected", payload: CredentialOffersSelected, } | { "type": "[Credential Metadata] Update", payload: UpdateCredentialMetadata, } | { "type": "[User Journey] Cancel" } | { "type": "[Search] Query", payload: SearchQuery, } | { "type": "[Search] Add recent", payload: AddRecentSearch, } | { "type": "[Search] Delete recent", payload: DeleteRecentSearch, };