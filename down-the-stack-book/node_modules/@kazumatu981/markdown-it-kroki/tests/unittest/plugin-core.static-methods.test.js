const expect = require('chai').expect;
const { MarkdownItKrokiCore } = require('../../lib/plugin-core');

describe('# [unit-test] plugin-core.js', () => {
    describe('## static method: readLanguageAndAltText() - language', () => {
        [
            { test: null, expected: '' },
            { test: undefined, expected: '' },
            { test: '', expected: '' },
            { test: '    ', expected: '' },
            { test: 'plantuml', expected: 'plantuml' },
            { test: '  plantuml', expected: 'plantuml' },
            { test: 'plantuml  ', expected: 'plantuml' },
            { test: 'plantuml +++', expected: 'plantuml' },
            { test: 'html+md', expected: 'html+md' },
            { test: 'graphviz[]', expected: 'graphviz' },
            { test: 'graphviz[test]', expected: 'graphviz' },
            { test: 'graphviz [test test]', expected: 'graphviz' },
        ].forEach(testCase => {
            it(`### Can read diagramLanguage. in case \'${testCase.test}\'`, () => {
                const actual = MarkdownItKrokiCore.readLanguageAndAltText(testCase.test);
                const expected = testCase.expected;
                expect(actual.language).to.be.equal(expected);
            })
        });
    });
    describe('## static method: readLanguageAndAltText() - alt', () => {
        [
            { test: null, expected: '' },
            { test: undefined, expected: '' },
            { test: '', expected: '' },
            { test: '    ', expected: '' },
            { test: 'plantuml', expected: '' },
            { test: '  plantuml', expected: '' },
            { test: 'plantuml  ', expected: '' },
            { test: 'plantuml +++', expected: '' },
            { test: 'html+md', expected: '' },
            { test: 'graphviz[]', expected: '' },
            { test: 'graphviz[test]', expected: 'test' },
            { test: 'graphviz [test test]', expected: 'test test' },
        ].forEach(testCase => {
            it(`### Can read diagramLanguage. in case \'${testCase.test}\'`, () => {
                const actual = MarkdownItKrokiCore.readLanguageAndAltText(testCase.test);
                const expected = testCase.expected;
                expect(actual.alt).to.be.equal(expected);
            })
        });
    });
});