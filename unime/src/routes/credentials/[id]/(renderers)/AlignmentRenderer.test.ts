import '@testing-library/jest-dom';

import { setLocale } from '$i18n/i18n-svelte';
import { loadAllLocales } from '$i18n/i18n-util.sync';

import { render, screen } from '@testing-library/svelte';

import AlignmentRenderer from './AlignmentRenderer.svelte';

describe('AlignmentRenderer', () => {
  beforeAll(() => {
    loadAllLocales();
    setLocale('en');
  });

  test('renders an alignment that references a skills framework as an official skill', () => {
    render(AlignmentRenderer, {
      alignment: {
        targetName: 'work in teams',
        targetUrl: 'https://esco.ec.europa.eu/en/classification/skill?uri=http://data.europa.eu/esco/skill/S1.4.1',
        targetCode: 'S1.4.1',
        targetDescription: 'Work confidently within a group.',
        targetFramework: 'ESCO',
        targetType: 'ext:EscoSkill',
      },
    });

    expect(screen.getByText('work in teams')).toBeInTheDocument();
    expect(screen.getByText('ESCO')).toBeInTheDocument();
    expect(screen.getByText('Skill')).toBeInTheDocument();
    expect(screen.getByText('S1.4.1')).toBeInTheDocument();
    expect(screen.getByRole('link')).toHaveAttribute(
      'href',
      'https://esco.ec.europa.eu/en/classification/skill?uri=http://data.europa.eu/esco/skill/S1.4.1',
    );
  });

  test('renders a free alignment as its name and description', () => {
    render(AlignmentRenderer, {
      alignment: {
        targetName: 'Leadership',
        targetUrl: 'https://example.org',
        targetDescription: 'I can develop a vision and communicate it clearly to inspire others.',
      },
    });

    expect(screen.getByText('Leadership')).toBeInTheDocument();
    expect(
      screen.getByText('I can develop a vision and communicate it clearly to inspire others.'),
    ).toBeInTheDocument();
    expect(screen.queryByRole('link')).not.toBeInTheDocument();
  });
});
