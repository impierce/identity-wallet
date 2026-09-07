import { findOfficialSkill, toAlignments } from './alignment';

const ESCO_SKILL = {
  type: ['Alignment'],
  targetName: 'work in teams',
  targetUrl: 'https://esco.ec.europa.eu/en/classification/skill?uri=http://data.europa.eu/esco/skill/S1.4.1',
  targetCode: 'S1.4.1',
  targetDescription: 'Work confidently within a group with each doing their part in the service of the whole.',
  targetFramework: 'ESCO',
  targetType: 'ext:EscoSkill',
};

describe('findOfficialSkill', () => {
  test('leaves out a code that identifies an entry opaquely', () => {
    // ESCO identifies its concepts by a UUID, which says nothing to the holder of the credential.
    expect(findOfficialSkill({ ...ESCO_SKILL, targetCode: '4cca4ccf-b956-4561-af8e-151b66af9631' })?.code).toBeNull();
  });

  test('resolves an alignment that references an entry of a skills framework', () => {
    expect(findOfficialSkill(ESCO_SKILL)).toEqual({
      name: 'work in teams',
      framework: 'ESCO',
      code: 'S1.4.1',
      kind: 'skill',
      url: ESCO_SKILL.targetUrl,
      description: ESCO_SKILL.targetDescription,
    });
  });

  test('resolves the kind of entry an alignment references', () => {
    expect(findOfficialSkill({ ...ESCO_SKILL, targetType: 'ext:EscoOccupation' })?.kind).toBe('occupation');
    expect(findOfficialSkill({ ...ESCO_SKILL, targetType: 'ceasn:Competency' })?.kind).toBe('skill');
    expect(findOfficialSkill({ ...ESCO_SKILL, targetType: ['CFItem', 'ext:EscoSkill'] })?.kind).toBe('skill');
    expect(findOfficialSkill({ ...ESCO_SKILL, targetType: 'ceterms:Credential' })?.kind).toBeNull();
    expect(findOfficialSkill({ ...ESCO_SKILL, targetType: undefined })?.kind).toBeNull();
  });

  test('ignores alignments that cannot be traced back to a framework', () => {
    expect(findOfficialSkill({ ...ESCO_SKILL, targetFramework: undefined })).toBeNull();
    expect(findOfficialSkill({ ...ESCO_SKILL, targetCode: '  ' })).toBeNull();
    expect(findOfficialSkill({ targetName: 'Leadership', targetUrl: 'https://example.org' })).toBeNull();
    expect(findOfficialSkill(undefined)).toBeNull();
  });
});

describe('toAlignments', () => {
  test('reads alignments that are wrapped in a list', () => {
    expect(toAlignments([ESCO_SKILL])).toEqual([ESCO_SKILL]);
  });

  test('reads a single alignment that is not wrapped in a list', () => {
    expect(toAlignments(ESCO_SKILL)).toEqual([ESCO_SKILL]);
  });

  test('reads anything else as no alignments at all', () => {
    expect(toAlignments([null, 'Leadership'])).toEqual([]);
    expect(toAlignments(undefined)).toEqual([]);
    expect(toAlignments('Leadership')).toEqual([]);
  });
});
