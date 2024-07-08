export default class TagInsertStream extends TransformStream {
  constructor (newTags) {
    super({
      transform: (chunk, controller) => {
        let chunkStr = this.textDecoder.decode(chunk)
        const idx = chunkStr.indexOf('</body>')
        if (idx !== -1) {
          chunkStr = chunkStr.slice(0, idx) + newTags + chunkStr.slice(idx - 1)
          const outputBytes = this.textEncoder.encode(chunkStr)
          controller.enqueue(outputBytes)
        } else {
          controller.enqueue(chunk)
        }
      }
    })
    this.textEncoder = new TextEncoder()
    this.textDecoder = new TextDecoder()
  }
}
