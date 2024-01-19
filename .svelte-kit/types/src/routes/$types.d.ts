import type * as Kit from '@sveltejs/kit';

type Expand<T> = T extends infer O ? { [K in keyof O]: O[K] } : never;
// @ts-ignore
type MatcherParam<M> = M extends (param : string) => param is infer U ? U extends string ? U : string : string;
type RouteParams = {  };
type RouteId = '/';
type MaybeWithVoid<T> = {} extends T ? T | void : T;
export type RequiredKeys<T> = { [K in keyof T]-?: {} extends { [P in K]: T[K] } ? never : K; }[keyof T];
type OutputDataShape<T> = MaybeWithVoid<Omit<App.PageData, RequiredKeys<T>> & Partial<Pick<App.PageData, keyof T & keyof App.PageData>> & Record<string, any>>
type EnsureDefined<T> = T extends null | undefined ? {} : T;
type OptionalUnion<U extends Record<string, any>, A extends keyof U = U extends U ? keyof U : never> = U extends unknown ? { [P in Exclude<A, keyof U>]?: never } & U : never;
export type Snapshot<T = any> = Kit.Snapshot<T>;
type PageParentData = EnsureDefined<LayoutData>;
type LayoutRouteId = RouteId | "/" | "/(app)/activity" | "/(app)/activity/connection/[id]" | "/(app)/me" | "/(app)/me/search" | "/(app)/me/settings" | "/(app)/me/settings/about" | "/(app)/me/settings/app" | "/(app)/me/settings/app/theme" | "/(app)/me/settings/backup" | "/(app)/me/settings/profile" | "/(app)/me/settings/profile/avatar" | "/(app)/me/settings/profile/name" | "/(app)/scan" | "/(journey)/goals" | "/(journey)/goals/[id]/faqs" | "/(journey)/goals/[id]/step/[id]" | "/badges/[id]" | "/credentials/[id]" | "/prompt/accept-connection" | "/prompt/credential-offer" | "/prompt/password-required" | "/prompt/share-credentials" | "/welcome" | "/welcome/customize/avatar" | "/welcome/customize/name" | "/welcome/customize/theme" | "/welcome/password" | "/welcome/password/completed" | "/welcome/password/confirm" | "/welcome/pledge" | "/welcome/terms" | null
type LayoutParams = RouteParams & { id?: string }
type LayoutParentData = EnsureDefined<{}>;

export type PageServerData = null;
export type PageData = Expand<PageParentData>;
export type LayoutServerData = null;
export type LayoutLoad<OutputData extends OutputDataShape<LayoutParentData> = OutputDataShape<LayoutParentData>> = Kit.Load<LayoutParams, LayoutServerData, LayoutParentData, OutputData, LayoutRouteId>;
export type LayoutLoadEvent = Parameters<LayoutLoad>[0];
export type LayoutData = Expand<Omit<LayoutParentData, keyof LayoutParentData & EnsureDefined<LayoutServerData>> & OptionalUnion<EnsureDefined<LayoutParentData & EnsureDefined<LayoutServerData>>>>;