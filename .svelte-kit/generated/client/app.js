import * as client_hooks from '../../../src/hooks.client.ts';

export { matchers } from './matchers.js';

export const nodes = [
	() => import('./nodes/0'),
	() => import('./nodes/1'),
	() => import('./nodes/2'),
	() => import('./nodes/3'),
	() => import('./nodes/4'),
	() => import('./nodes/5'),
	() => import('./nodes/6'),
	() => import('./nodes/7'),
	() => import('./nodes/8'),
	() => import('./nodes/9'),
	() => import('./nodes/10'),
	() => import('./nodes/11'),
	() => import('./nodes/12'),
	() => import('./nodes/13'),
	() => import('./nodes/14'),
	() => import('./nodes/15'),
	() => import('./nodes/16'),
	() => import('./nodes/17'),
	() => import('./nodes/18'),
	() => import('./nodes/19'),
	() => import('./nodes/20'),
	() => import('./nodes/21'),
	() => import('./nodes/22'),
	() => import('./nodes/23'),
	() => import('./nodes/24'),
	() => import('./nodes/25'),
	() => import('./nodes/26'),
	() => import('./nodes/27'),
	() => import('./nodes/28'),
	() => import('./nodes/29'),
	() => import('./nodes/30'),
	() => import('./nodes/31'),
	() => import('./nodes/32'),
	() => import('./nodes/33'),
	() => import('./nodes/34'),
	() => import('./nodes/35'),
	() => import('./nodes/36'),
	() => import('./nodes/37'),
	() => import('./nodes/38'),
	() => import('./nodes/39')
];

export const server_loads = [];

export const dictionary = {
		"/": [8],
		"/(app)/activity": [9,[2]],
		"/(app)/activity/connection/[id]": [10],
		"/badges/[id]": [25],
		"/credentials/[id]": [26],
		"/(journey)/goals": [22,[3]],
		"/(journey)/goals/[id]/faqs": [23,[3,4]],
		"/(journey)/goals/[id]/step/[id]": [24,[3,4]],
		"/(app)/me": [11,[2]],
		"/(app)/me/search": [12,[2]],
		"/(app)/me/settings": [13,[2]],
		"/(app)/me/settings/about": [14,[2]],
		"/(app)/me/settings/app": [15,[2]],
		"/(app)/me/settings/app/theme": [16,[2]],
		"/(app)/me/settings/backup": [17,[2]],
		"/(app)/me/settings/profile": [18,[2]],
		"/(app)/me/settings/profile/avatar": [19,[2]],
		"/(app)/me/settings/profile/name": [20,[2]],
		"/prompt/accept-connection": [27,[5]],
		"/prompt/credential-offer": [28,[5]],
		"/prompt/password-required": [29,[5]],
		"/prompt/share-credentials": [30,[5]],
		"/(app)/scan": [21],
		"/welcome": [31,[6]],
		"/welcome/customize/avatar": [32,[6,7]],
		"/welcome/customize/name": [33],
		"/welcome/customize/theme": [34,[6,7]],
		"/welcome/password": [35,[6]],
		"/welcome/password/completed": [36,[6]],
		"/welcome/password/confirm": [37,[6]],
		"/welcome/pledge": [38,[6]],
		"/welcome/terms": [39,[6]]
	};

export const hooks = {
	handleError: client_hooks.handleError || (({ error }) => { console.error(error) }),
};

export { default as root } from '../root.svelte';