const { MarkdownItKrokiCore } = require('./lib/plugin-core');

module.exports = (md, opt) => {
    const plugin = new MarkdownItKrokiCore(md);
    plugin.setOptions(opt).use();
};
