// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { AddRecentSearch } from "./AddRecentSearch";
import type { AddTrustListEntry } from "./AddTrustListEntry";
import type { CancelUserFlow } from "./CancelUserFlow";
import type { CreateNew } from "./CreateNew";
import type { CredentialOffersSelected } from "./CredentialOffersSelected";
import type { CredentialsSelected } from "./CredentialsSelected";
import type { DeleteRecentSearch } from "./DeleteRecentSearch";
import type { DeleteTrustListEntry } from "./DeleteTrustListEntry";
import type { DevProfile } from "./DevProfile";
import type { EditTrustListEntry } from "./EditTrustListEntry";
import type { QrCodeScanned } from "./QrCodeScanned";
import type { SearchQuery } from "./SearchQuery";
import type { SetLocale } from "./SetLocale";
import type { SetPreferredDidMethod } from "./SetPreferredDidMethod";
import type { SetPreferredKeyType } from "./SetPreferredKeyType";
import type { ToggleTrustListEntry } from "./ToggleTrustListEntry";
import type { TrustListsAdd } from "./TrustListAdd";
import type { TrustListsDelete } from "./TrustListDelete";
import type { TrustListsEdit } from "./TrustListEdit";
import type { UnlockStorage } from "./UnlockStorage";
import type { UpdateCredentialMetadata } from "./UpdateCredentialMetadata";
import type { UpdateProfileSettings } from "./UpdateProfileSettings";
import type { UpdateSortingPreference } from "./UpdateSortingPreference";

export type Action = { "type": "[App] Get state" } | { "type": "[Storage] Unlock", payload: UnlockStorage, } | { "type": "[App] Reset" } | { "type": "[DID] Create new", payload: CreateNew, } | { "type": "[Settings] Set locale", payload: SetLocale, } | { "type": "[Settings] Update profile", payload: UpdateProfileSettings, } | { "type": "[QR Code] Scanned", payload: QrCodeScanned, } | { "type": "[Authenticate] Connection accepted" } | { "type": "[User Flow] Cancel", payload?: CancelUserFlow, } | { "type": "[DEV] Load DEV profile", payload: DevProfile, } | { "type": "[DEV] Toggle DEV mode" } | { "type": "[Authenticate] Credentials selected", payload: CredentialsSelected, } | { "type": "[Credential Offer] Selected", payload: CredentialOffersSelected, } | { "type": "[Credential Metadata] Update", payload: UpdateCredentialMetadata, } | { "type": "[User Journey] Cancel" } | { "type": "[Settings] Update sorting preference", payload: UpdateSortingPreference, } | { "type": "[Search] Query", payload: SearchQuery, } | { "type": "[Search] Add recent", payload: AddRecentSearch, } | { "type": "[Search] Delete recent", payload: DeleteRecentSearch, } | { "type": "[DID] Set preferred method", payload: SetPreferredDidMethod, } | { "type": "[Keys] Set preferred key type", payload: SetPreferredKeyType, } | { "type": "[Trust List] Add Entry", payload: AddTrustListEntry, } | { "type": "[Trust List] Edit Entry", payload: EditTrustListEntry, } | { "type": "[Trust List] Delete Entry", payload: DeleteTrustListEntry, } | { "type": "[Trust List] Toggle Entry", payload: ToggleTrustListEntry, } | { "type": "[Trust Lists] Add", payload: TrustListsAdd, } | { "type": "[Trust Lists] Edit", payload: TrustListsEdit, } | { "type": "[Trust Lists] Delete", payload: TrustListsDelete, };