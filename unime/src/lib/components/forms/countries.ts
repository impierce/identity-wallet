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

// Additional countries
const additional: Country[] = [
  { code: 'CH', name: 'Schweiz / Suisse / Svizzera' },
  { code: 'GB', name: 'United Kingdom' },
];

const countries = memberStates.concat(additional).sort((a, b) => a.code.localeCompare(b.code));

export default countries;
