// This file was auto-generated by 'typesafe-i18n'. Any manual changes will be overwritten.
/* eslint-disable */
import type { BaseTranslation as BaseTranslationType, LocalizedString, RequiredParams } from 'typesafe-i18n'

export type BaseTranslation = BaseTranslationType
export type BaseLocale = 'en'

export type Locales =
	| 'de'
	| 'en'
	| 'nl'

export type Translation = RootTranslation

export type Translations = RootTranslation

type RootTranslation = {
	/**
	 * H​i​ ​{​n​a​m​e​}​!​ ​P​l​e​a​s​e​ ​l​e​a​v​e​ ​a​ ​s​t​a​r​ ​i​f​ ​y​o​u​ ​l​i​k​e​ ​t​h​i​s​ ​p​r​o​j​e​c​t​:​ ​h​t​t​p​s​:​/​/​g​i​t​h​u​b​.​c​o​m​/​i​v​a​n​h​o​f​e​r​/​t​y​p​e​s​a​f​e​-​i​1​8​n
	 * @param {string} name
	 */
	HI: RequiredParams<'name'>
	/**
	 * W​e​l​c​o​m​e
	 */
	WELCOME: string
	/**
	 * P​l​e​a​s​e​ ​e​n​t​e​r​ ​y​o​u​r​ ​n​a​m​e
	 */
	PROMPT_NAME: string
	/**
	 * C​r​e​a​t​e​ ​n​e​w​ ​i​d​e​n​t​i​t​y
	 */
	CREATE_IDENTITY: string
	/**
	 * Y​o​u​ ​h​a​v​e​ ​j​u​s​t​ ​c​r​e​a​t​e​d​ ​a​ ​n​e​w​ ​d​i​g​i​t​a​l​ ​i​d​e​n​t​i​t​y​!
	 */
	CREATE_IDENTITY_SUCCESS_TITLE: string
	/**
	 * C​o​n​t​i​n​u​e​ ​b​y​ ​a​d​d​i​n​g​ ​m​o​r​e​ ​i​n​f​o​r​m​a​t​i​o​n​ ​a​b​o​u​t​ ​y​o​u​r​s​e​l​f​ ​o​r​ ​s​c​a​n​ ​a​ ​c​r​e​d​e​n​t​i​a​l​.
	 */
	CREATE_IDENTITY_SUCCESS_BODY: string
	/**
	 * A​p​p​ ​s​e​t​t​i​n​g​s
	 */
	APP_SETTINGS: string
	/**
	 * Y​o​u​r​ ​D​I​D​s
	 */
	YOUR_DIDS: string
	/**
	 * R​e​s​e​t​ ​a​p​p
	 */
	RESET_APP: string
	/**
	 * Y​o​u​ ​h​a​v​e​n​'​t​ ​u​s​e​d​ ​a​n​y​ ​c​r​e​d​e​n​t​i​a​l​s​ ​y​e​t​.
	 */
	NO_HISTORY: string
	/**
	 * S​e​l​e​c​t​ ​i​n​f​o​r​m​a​t​i​o​n​ ​y​o​u​ ​w​a​n​t​ ​t​o​ ​s​h​a​r​e
	 */
	SHARE_CREDENTIALS_TITLE: string
	/**
	 * S​h​a​r​e
	 */
	SHARE_CREDENTIALS_CONFIRM: string
	/**
	 * C​a​n​c​e​l
	 */
	CANCEL: string
}

export type TranslationFunctions = {
	/**
	 * Hi {name}! Please leave a star if you like this project: https://github.com/ivanhofer/typesafe-i18n
	 */
	HI: (arg: { name: string }) => LocalizedString
	/**
	 * Welcome
	 */
	WELCOME: () => LocalizedString
	/**
	 * Please enter your name
	 */
	PROMPT_NAME: () => LocalizedString
	/**
	 * Create new identity
	 */
	CREATE_IDENTITY: () => LocalizedString
	/**
	 * You have just created a new digital identity!
	 */
	CREATE_IDENTITY_SUCCESS_TITLE: () => LocalizedString
	/**
	 * Continue by adding more information about yourself or scan a credential.
	 */
	CREATE_IDENTITY_SUCCESS_BODY: () => LocalizedString
	/**
	 * App settings
	 */
	APP_SETTINGS: () => LocalizedString
	/**
	 * Your DIDs
	 */
	YOUR_DIDS: () => LocalizedString
	/**
	 * Reset app
	 */
	RESET_APP: () => LocalizedString
	/**
	 * You haven't used any credentials yet.
	 */
	NO_HISTORY: () => LocalizedString
	/**
	 * Select information you want to share
	 */
	SHARE_CREDENTIALS_TITLE: () => LocalizedString
	/**
	 * Share
	 */
	SHARE_CREDENTIALS_CONFIRM: () => LocalizedString
	/**
	 * Cancel
	 */
	CANCEL: () => LocalizedString
}

export type Formatters = {}
