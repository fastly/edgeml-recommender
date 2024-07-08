import { includeBytes } from 'fastly:experimental'

const SOURCES = {
  '/fastly/script': {
    content: includeBytes('src/assets/client.js'),
    type: 'application/javascript'
  },
  '/fastly/style': {
    content: includeBytes('src/assets/client.css'),
    type: 'text/css'
  },
  '/fastly/logo': {
    content: includeBytes('src/assets/fastly-logo.svg'),
    type: 'image/svg+xml'
  }
}

export const hasSynth = name => name in SOURCES

export const synthResponse = name => {
  const src = SOURCES[name]
  return new Response(src.content, {
    status: 200,
    headers: { 'content-type': src.type }
  })
}

export const jsonResponse = data => {
  return new Response(JSON.stringify(data), {
    status: 200,
    headers: { 'content-type': 'application/json' }
  })
}
