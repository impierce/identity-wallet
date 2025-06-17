import iso3166 from './iso-3166-1_alpha-2.json';

type Country = {
  code: string;
  name: string;
};

// Member states of the European Union
const memberStates: Country[] = [
  { code: 'AT', name: 'Österreich' },
  { code: 'BE', name: 'België / Belgique / Belgien' },
  { code: 'BG', name: 'България' },
  { code: 'CY', name: 'Κύπρος / Kıbrıs' },
  { code: 'CZ', name: 'Česká republika' },
  { code: 'DE', name: 'Deutschland' },
  { code: 'DK', name: 'Danmark' },
  { code: 'EE', name: 'Eesti' },
  { code: 'ES', name: 'España' },
  { code: 'FI', name: 'Suomi' },
  { code: 'FR', name: 'France' },
  { code: 'GR', name: 'Ελλάδα' },
  { code: 'HR', name: 'Hrvatska' },
  { code: 'HU', name: 'Magyarország' },
  { code: 'IE', name: 'Ireland' },
  { code: 'IT', name: 'Italia' },
  { code: 'LT', name: 'Lietuva' },
  { code: 'LU', name: 'Lëtzebuerg / Luxembourg' },
  { code: 'LV', name: 'Latvija' },
  { code: 'MT', name: 'Malta' },
  { code: 'NL', name: 'Nederland' },
  { code: 'PL', name: 'Polska' },
  { code: 'PT', name: 'Portugal' },
  { code: 'RO', name: 'România' },
  { code: 'SE', name: 'Sverige' },
  { code: 'SI', name: 'Slovenija' },
  { code: 'SK', name: 'Slovensko' },
];

// Additional supported countries
const additional: Country[] = [
  { code: 'CH', name: 'Schweiz / Suisse / Svizzera' },
  { code: 'GB', name: 'United Kingdom' },
];

// Merge and sort `memberStates` and `additional`
// const countries = memberStates.concat(additional).sort((a, b) => a.code.localeCompare(b.code));

// source: https://github.com/lukes/ISO-3166-Countries-with-Regional-Codes/blob/v10.0/slim-2/slim-2.json
// adjustments:
// - 'GB': replace 'United Kingdom of Great Britain and Northern Ireland' with 'United Kingdom'
// - 'NL': replace 'Netherlands, Kingdom of the' with 'Netherlands'
const countries: Country[] = iso3166.map((i) => ({ code: i['alpha-2'], name: i.name }));

export default countries;
