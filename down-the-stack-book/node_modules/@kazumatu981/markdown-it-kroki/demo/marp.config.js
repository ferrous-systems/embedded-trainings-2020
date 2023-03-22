
module.exports = {
    inputDir: './',
    engine: ({ marp }) => marp.use(require('../index'), {
        entrypoint: "https://kroki.io",
        marpAutoScaling: true
    })
}